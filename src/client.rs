use crate::core::types::AccountId;
use log::debug;
use std::io::{Read, Write};
use std::net::TcpStream;

pub enum Command {
    //StartNode,
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

pub fn run_client(command: Command, port: u16) {
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).expect("fix me");
    debug!("Client connected to the server");

    let message = match command {
        Command::CreateAccount {
            account,
            starting_balance,
        } => {
            format!(
                "Create Account -- account number: {}, starting_balance: {}",
                account, starting_balance
            )
        }
        _ => {
            format!("lol")
        }
    };

    stream
        .write(message.as_bytes())
        .expect("actually -- fix me");
    println!("Sent: {}", message);

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).expect("no - fix me");
    let received = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received: {}", received);
}
