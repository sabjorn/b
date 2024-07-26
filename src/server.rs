use crate::client::ClientCommands;
use crate::core::types::{Block, BlockId, Blocks, Transaction, Transactions};
use bincode::{deserialize, serialize};
use log::{error, info};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
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
            ClientCommands::Transfer {
                from_account,
                to_account,
                amount,
            } => {
                info!("Received Transfer command");
                Ok(123)
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
    let condvar = Arc::new((Mutex::new(false), Condvar::new()));

    let blocks_clone = Arc::clone(&blocks);
    let transcation_queue_clone = Arc::clone(&transaction_queue);
    let condvar_clone = Arc::clone(&condvar);

    let interval = Duration::from_secs(10);
    thread::spawn(move || {
        let mut block_id: BlockId = 0;

        let mut blocks: Blocks = Vec::new();
        loop {
            info!("Periodic thread running.");
            let mut transcations = transcation_queue_clone.lock().unwrap();

            let block = Block {
                id: block_id,
                transactions: transcations.clone(),
            };
            transcations.clear();
            block_id += 1;

            let mut blocks = blocks_clone.write().unwrap();
            blocks.push(block);

            let (lock, cvar) = &*condvar_clone;
            let mut notified = lock.lock().unwrap();
            *notified = true;
            cvar.notify_all();

            thread::sleep(interval);
        }
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // clone signal
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                error!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
