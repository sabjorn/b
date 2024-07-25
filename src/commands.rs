use crate::core::types::AccountId;
use log::info;

pub fn create_account(account: AccountId, starting_balance: f64, port: u16) {
    info!(
        "Account with ID {} created with starting balance {} on port {}.",
        account, starting_balance, port
    );
}

pub fn transfer(from_account: AccountId, to_account: AccountId, amount: f64, port: u16) {
    info!(
        "Transferred {} from account {} to account {} on port {}.",
        amount, from_account, to_account, port
    );
}

pub fn check_balance(port: u16) {
    info!("Balance checked on port {}.", port);
}
