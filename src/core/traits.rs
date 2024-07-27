use super::types::{AccountId, BlockId, TransactionId};

pub trait TransactionInfo {
    fn contains_account(&self, account: AccountId) -> bool;
    fn calculate_total(&self, account: AccountId) -> Option<f64>;
}

pub trait BlockInfo {
    fn contains_transaction(&self, block: BlockId, transaction: TransactionId) -> bool;
}
