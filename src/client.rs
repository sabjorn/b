use crate::core::types::AccountId;
use crate::server::ServerResponse;
use bincode::{deserialize, serialize};
use clap::Subcommand;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum ClientCommands {
    CreateAccount {
        account: AccountId,
        starting_balance: f64,
    },
    Transfer {
        from_account: AccountId,
        to_account: AccountId,
        amount: f64,
    },
    Balance {
        account: AccountId,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientReturn {}

pub fn run_client(command: ClientCommands, port: u16) {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).expect("fix me");
    debug!("Client connected to the server");

    let serialized_command = serialize(&command).expect("Failed to serialize command");
    stream
        .write(&serialized_command)
        .expect("failed to send serialized command");
    info!("Sent command: {:?}", command);

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).expect("no - fix me");
    let return_value: Result<ServerResponse, String> = deserialize(&buffer[..bytes_read]).unwrap();

    match return_value {
        Err(e) => error!("recieved error: {}", e),
        Ok(val) => match val {
            ServerResponse::Transferred {
                block_id,
                transaction_id,
            } => info!(
                "transfer success. \n\tblock_id: {}\n\ttransaction_id: {}",
                block_id, transaction_id
            ),
            ServerResponse::Balance { balance } => info!("balance: {}", balance),
        },
    }
}
