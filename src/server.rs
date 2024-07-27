use crate::client::ClientCommands;
use crate::core::types::TranscationInfo;
use crate::core::types::{Block, BlockId, Blocks, Transaction, Transactions};
use bincode::{deserialize, serialize};
use log::{error, info};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn handle_client(
    mut stream: TcpStream,
    shared_blocks: Arc<RwLock<Blocks>>,
    shared_transcations: Arc<Mutex<Transactions>>,
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

        let return_value: Result<i64, String> = match command {
            ClientCommands::Balance { account } => {
                info!("account_id: {} recieved", account);
                let blocks = shared_blocks.read().unwrap();
                match (*blocks).calculate_total(account) {
                    Some(value) => Ok(value as i64),
                    None => Err(format!("could not find balance for account: {}", account)),
                }
            }
            ClientCommands::CreateAccount {
                account,
                starting_balance,
            } => {
                info!("Received CreateAccount command");
                let transaction_id = 123; // generate automatically
                {
                    let transaction = Transaction {
                        id: transaction_id,
                        to: account,
                        from: 9999,
                        amount: starting_balance,
                    };

                    let mut transactions = shared_transcations.lock().unwrap();
                    transactions.push(transaction);
                }

                let mut block_contains_transaction = false;
                while !block_contains_transaction {
                    let (lock, cvar) = &*shared_condvar;
                    let mut block_id = lock.lock().unwrap();
                    let block_id = cvar.wait(block_id).unwrap(); // we only care about the signal
                    block_contains_transaction = true;
                }
                Ok(account)
            }
            ClientCommands::Transfer {
                from_account,
                to_account,
                amount,
            } => {
                info!("Received Transfer command");

                let mut sum: Option<f64> = None;
                {
                    let transactions = shared_transcations.lock().unwrap();
                    let transactions_total = transactions.calculate_total(from_account);

                    let blocks = shared_blocks.read().unwrap();
                    let blocks_total = blocks.calculate_total(from_account);

                    sum = match (transactions_total, blocks_total) {
                        (Some(val1), Some(val2)) => Some(val1 + val2),
                        (Some(val), None) | (None, Some(val)) => Some(val),
                        (None, None) => None,
                    };
                }

                match sum {
                    Some(s) => {
                        if s <= amount {
                            {
                                let mut transactions = shared_transcations.lock().unwrap();
                                transactions.push(Transaction {
                                    id: 111,
                                    to: to_account,
                                    from: from_account,
                                    amount: amount,
                                });
                            }
                            let mut block_contains_transaction = false;
                            while !block_contains_transaction {
                                let (lock, cvar) = &*shared_condvar;
                                let mut block_id = lock.lock().unwrap();
                                let block_id = cvar.wait(block_id).unwrap(); // we only care about the signal
                                block_contains_transaction = true;
                            }
                            Ok(from_account)
                        } else {
                            Err(format!(
                                "Not enough in account {} to transfer {}",
                                from_account, amount
                            ))
                        }
                    }
                    None => Err(format!("Account not found: {}", from_account)),
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
    info!("Server listening on port {}", port);

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
            info!("Periodic thread running.");
            {
                let mut transcations = transcation_queue_clone.lock().unwrap();

                let block = Block {
                    id: block_id,
                    transactions: transcations.clone(),
                };
                transcations.clear();

                let mut blocks = blocks_clone.write().unwrap();
                blocks.push(block);

                let (lock, cvar) = &*condvar_clone;
                let mut notified = lock.lock().unwrap();
                *notified = block_id;

                cvar.notify_all();
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
                // clone signal
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
