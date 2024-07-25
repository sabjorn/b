use std::collections::HashMap;
//use chrono::{DateTime, Utc};

pub type AccountId = i64;
pub type TranscationId = i64;
pub type BlockId = i64;

pub struct Transaction {
    id: TranscationId,
    to: AccountId,
    from: AccountId,
    amount: f64,
}

pub struct TransactionList(pub Vec<Transaction>);

impl TransactionList {
    pub fn new() -> Self {
        TransactionList(Vec::new())
    }
}


struct Block {
    id: BlockId,
    //time: DateTime<Utc>,
    transactions: Vec<Transaction>,
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    fn create_relationships() -> (Collection, Owned) {
//    }
