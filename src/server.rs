use crate::client::ClientCommands;
use crate::core::constants::MASTER_ID;
use crate::core::traits::{BlockInfo, TransactionInfo};
use crate::core::types::{
    AccountId, Block, BlockId, Blocks, Transaction, TransactionId, Transactions,
};
use bincode::{deserialize, serialize};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub enum ServerResponse {
    Transferred {
        block_id: BlockId,
        transaction_id: TransactionId,
    },
    Balance {
        balance: f64,
    },
}

fn check_account_exists(
    account: AccountId,
    shared_blocks: &Arc<RwLock<Blocks>>,
    shared_transactions: &Arc<Mutex<Transactions>>,
) -> bool {
    shared_blocks.read().unwrap().contains_account(account)
        || shared_transactions
            .lock()
            .unwrap()
            .contains_account(account)
}

fn wait_on_block_id(
    transaction_id: TransactionId,
    shared_blocks: &Arc<RwLock<Blocks>>,
    shared_condvar: &Arc<(Mutex<BlockId>, Condvar)>,
) -> BlockId {
    let mut block_id: BlockId = 0;
    let mut block_contains_transaction = false;
    while !block_contains_transaction {
        let (lock, cvar) = shared_condvar.as_ref();
        let cond_block_id = lock.lock().unwrap();
        let cond_block_id = cvar.wait(cond_block_id).unwrap();
        block_id = *cond_block_id;
        block_contains_transaction = shared_blocks
            .read()
            .unwrap()
            .contains_transaction(block_id, transaction_id);
    }
    block_id
}

fn handle_client(
    mut stream: TcpStream,
    shared_blocks: Arc<RwLock<Blocks>>,
    shared_transactions: Arc<Mutex<Transactions>>,
    shared_condvar: Arc<(Mutex<BlockId>, Condvar)>,
) {
    let mut buffer = [0; 512];
    loop {
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from socket");

        if bytes_read == 0 {
            return;
        }

        let command: ClientCommands = match deserialize(&buffer[..bytes_read]) {
            Ok(cmd) => cmd,
            Err(e) => {
                let error_message = format!("Failed to deserialize: {}", e);
                error!("{}", &error_message);
                let return_value: Result<(), String> = Err(error_message);
                let serialized_command =
                    serialize(&return_value).expect("Failed to serialize command");

                stream
                    .write(&serialized_command)
                    .expect("failed to send serialized command");
                return;
            }
        };

        let return_value: Result<ServerResponse, String> = match command {
            ClientCommands::Balance { account } => {
                info!("account_id: {} recieved", account);
                match account == MASTER_ID {
                    true => Err(format!("could not check balance for account id: MASTER_ID")),
                    false => shared_blocks
                        .read()
                        .unwrap()
                        .calculate_total(account)
                        .map(|value| ServerResponse::Balance { balance: value })
                        .ok_or_else(|| format!("could not find balance for account: {}", account)),
                }
            }
            ClientCommands::CreateAccount {
                account,
                starting_balance,
            } => {
                info!("Received CreateAccount command");
                let create_account = || {
                    let transaction = Transaction::new(account, MASTER_ID, starting_balance);
                    let transaction_id = transaction.id;

                    {
                        let mut transactions = shared_transactions.lock().unwrap();
                        transactions.push(transaction);
                    }

                    let block_id =
                        wait_on_block_id(transaction_id, &shared_blocks, &shared_condvar);

                    Ok(ServerResponse::Transferred {
                        block_id,
                        transaction_id,
                    })
                };

                match (
                    account == MASTER_ID,
                    check_account_exists(account, &shared_blocks, &shared_transactions),
                ) {
                    (true, _) => Err(format!(
                        "could not create account for account id: MASTER_ID"
                    )),
                    (_, true) => Err(format!("Account {} already exists", account)),
                    (false, false) => create_account(),
                }
            }
            ClientCommands::Transfer {
                from_account,
                to_account,
                amount,
            } => {
                info!("Received Transfer command");
                let transfer = || {
                    let mut total_balance: Option<f64> = None;
                    {
                        let transactions = shared_transactions.lock().unwrap();
                        let transactions_total = transactions.calculate_total(from_account);

                        let blocks = shared_blocks.read().unwrap();
                        let blocks_total = blocks.calculate_total(from_account);

                        // total_balance Option type is a proxy for accounts existing
                        total_balance = match (transactions_total, blocks_total) {
                            (Some(val1), Some(val2)) => Some(val1 + val2),
                            (Some(val), None) | (None, Some(val)) => Some(val),
                            (None, None) => None,
                        };
                    }

                    match total_balance {
                        Some(balance) => {
                            if balance >= amount {
                                let transaction =
                                    Transaction::new(to_account, from_account, amount);
                                let transaction_id = transaction.id;
                                {
                                    let mut transactions = shared_transactions.lock().unwrap();
                                    transactions.push(transaction)
                                }

                                let block_id = wait_on_block_id(
                                    transaction_id,
                                    &shared_blocks,
                                    &shared_condvar,
                                );

                                Ok(ServerResponse::Transferred {
                                    block_id,
                                    transaction_id,
                                })
                            } else {
                                Err(format!(
                                    "Not enough in account {} to transfer {}",
                                    from_account, amount
                                ))
                            }
                        }
                        None => Err(format!("Account not found: {}", from_account)),
                    }
                };
                match (from_account == MASTER_ID, to_account == MASTER_ID) {
                    (true, _) => Err(format!(
                        "could not transfer because from account id was: MASTER_ID"
                    )),
                    (_, true) => Err(format!(
                        "could not transfer because to account id was: MASTER_ID"
                    )),
                    (false, false) => transfer(),
                }
            }
            _ => Err("Got command that is not implemented".to_string()),
        };

        let serialized_command = serialize(&return_value).expect("Failed to serialize command");
        stream
            .write(&serialized_command)
            .expect("failed to send serialized command");

        return;
    }
}

pub fn start_node(port: u16) -> std::io::Result<()> {
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let listener = TcpListener::bind(address).unwrap_or_else(|e| {
        error!("Error creating a TcpListener for port {} -- {}", port, e);
        panic!("Program will exit due to error.");
    });
    info!("b server listening on port {}", port);

    let blocks: Arc<RwLock<Blocks>> = Arc::new(RwLock::new(Vec::new()));
    let transaction_queue: Arc<Mutex<Transactions>> = Arc::new(Mutex::new(Vec::new()));
    let condvar = Arc::new((Mutex::new(0), Condvar::new()));

    let blocks_clone = Arc::clone(&blocks);
    let transcation_queue_clone = Arc::clone(&transaction_queue);
    let condvar_clone = Arc::clone(&condvar);

    let interval = Duration::from_secs(10);
    thread::spawn(move || {
        let mut block_id: BlockId = 0;
        loop {
            thread::sleep(interval);
            info!("Publishing block.");
            {
                let mut transactions = transcation_queue_clone.lock().unwrap();

                let block = Block {
                    id: block_id,
                    transactions: transactions.clone(),
                };
                transactions.clear();

                let mut blocks = blocks_clone.write().unwrap();
                blocks.push(block);

                let (lock, cvar) = &*condvar_clone;
                let mut notified = lock.lock().unwrap();
                *notified = block_id;

                cvar.notify_all();

                info!("Block published: {:?}", &blocks[block_id as usize]);
            }
            block_id += 1;
        }
    });

    for stream in listener.incoming() {
        let blocks_clone = Arc::clone(&blocks);
        let transcation_queue_clone = Arc::clone(&transaction_queue);
        let condvar_clone = Arc::clone(&condvar);
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream, blocks_clone, transcation_queue_clone, condvar_clone);
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
