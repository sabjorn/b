use std::io::{Read, Write};
use std::net::TcpStream;
use crate::core::types::AccountId;
use log::info;

pub fn create_account(account: AccountId, starting_balance: f64, port: u16) {
    info!(
        "Account with ID {} created with starting balance {} on port {}.",
        account, starting_balance, port
    );
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).expect("fix me");
    println!("Connected to the server");

    let message = "Hello from client!";
    stream.write(message.as_bytes()).expect("actually -- fix me");
    println!("Sent: {}", message);

    let mut buffer = [0; 512];
    let bytes_read = stream.read(&mut buffer).expect("no - fix me");
    println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    //Ok(())
}

pub fn transfer(from_account: AccountId, to_account: AccountId, amount: f64, port: u16) {
    info!(
        "Transferred {} from account {} to account {} on port {}.",
        amount, from_account, to_account, port
    );
}

pub fn check_balance(account: AccountId, port: u16) {
    info!("Balance checked for account {} on port {}.", account, port);
}
