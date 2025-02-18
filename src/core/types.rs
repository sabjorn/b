use super::traits::{BlockInfo, TransactionInfo};
use super::utilities::generate_id;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub type Id = u64;
pub type AccountId = Id;
pub type TransactionId = Id;
pub type BlockId = Id;
pub type Transactions = Vec<Transaction>;
pub type Blocks = Vec<Block>;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: TransactionId,
    to: AccountId,
    from: AccountId,
    amount: f64,
}

impl Transaction {
    pub fn new(to: AccountId, from: AccountId, amount: f64) -> Transaction {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let id = generate_id(to, from, amount, current_time);

        Transaction {
            id,
            to,
            from,
            amount,
        }
    }
}

impl TransactionInfo for Transactions {
    fn contains_account(&self, account: AccountId) -> bool {
        self.iter().any(|t| t.to == account || t.from == account)
    }

    fn calculate_total(&self, account: AccountId) -> Option<f64> {
        let sum: Option<f64> = self
            .iter()
            .filter_map(|t| {
                if t.to == account {
                    return Some(t.amount);
                }
                if t.from == account {
                    return Some(-t.amount);
                }
                None
            })
            .fold(None, |acc, amount| Some(acc.unwrap_or(0.0) + amount));
        sum
    }
}

#[derive(Debug)]
pub struct Block {
    pub id: BlockId,
    pub transactions: Transactions,
}

impl TransactionInfo for Block {
    fn contains_account(&self, account: AccountId) -> bool {
        self.transactions.contains_account(account)
    }
    fn calculate_total(&self, account: AccountId) -> Option<f64> {
        self.transactions.calculate_total(account)
    }
}

impl TransactionInfo for Blocks {
    fn contains_account(&self, account: AccountId) -> bool {
        self.iter().any(|b| b.contains_account(account))
    }

    fn calculate_total(&self, account: AccountId) -> Option<f64> {
        let sum: Option<f64> = self
            .iter()
            .filter_map(|block| block.calculate_total(account))
            .fold(None, |acc, amount| Some(acc.unwrap_or(0.0) + amount));
        sum
    }
}

impl BlockInfo for Blocks {
    fn contains_transaction(&self, block_id: BlockId, transaction_id: TransactionId) -> bool {
        let block = match self.get(block_id as usize) {
            Some(t) => t,
            None => {
                return false;
            }
        };

        block.transactions.iter().any(|t| t.id == transaction_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod transactions_tests {
        use super::*;

        fn create_transcations() -> Transactions {
            vec![
                Transaction {
                    id: 1,
                    to: 1,
                    from: 2,
                    amount: 2.34,
                },
                Transaction {
                    id: 2,
                    to: 1,
                    from: 2,
                    amount: 10.00,
                },
            ]
        }

        #[test]
        fn test_calculate_total_for_existing_id() {
            let transactions = create_transcations();

            let result = transactions.calculate_total(1);
            assert!(result.is_some());
            assert_eq!(result, Some(12.34));

            let result = transactions.calculate_total(2);
            assert!(result.is_some());
            assert_eq!(result, Some(-12.34));
        }

        #[test]
        fn test_calculate_total_no_id_returns_none() {
            let transactions = create_transcations();

            let result = transactions.calculate_total(3);
            assert!(result.is_none());
        }

        #[test]
        fn test_contains_account() {
            let transactions = create_transcations();

            let result = transactions.contains_account(1);
            assert!(result);

            let result = transactions.contains_account(2);
            assert!(result);

            let result = transactions.contains_account(3);
            assert!(!result);
        }

        mod block_tests {
            use super::*;

            fn create_block() -> Block {
                Block {
                    id: 0,
                    transactions: vec![
                        Transaction {
                            id: 1,
                            to: 1,
                            from: 2,
                            amount: 2.34,
                        },
                        Transaction {
                            id: 2,
                            to: 1,
                            from: 2,
                            amount: 10.00,
                        },
                    ],
                }
            }

            #[test]
            fn test_calculate_total_for_existing_id() {
                let block = create_block();

                let result = block.calculate_total(1);
                assert!(result.is_some());
                assert_eq!(result, Some(12.34));

                let result = block.calculate_total(2);
                assert!(result.is_some());
                assert_eq!(result, Some(-12.34));
            }

            #[test]
            fn test_calculate_total_no_id_returns_none() {
                let block = create_block();

                let result = block.calculate_total(3);
                assert!(result.is_none());
            }

            #[test]
            fn test_contains_account() {
                let block = create_block();

                let result = block.contains_account(1);
                assert!(result);

                let result = block.contains_account(2);
                assert!(result);

                let result = block.contains_account(3);
                assert!(!result);
            }
        }

        mod blocks_tests {
            use super::*;

            fn create_blocks() -> Blocks {
                vec![
                    Block {
                        id: 0,
                        transactions: vec![
                            Transaction {
                                id: 1,
                                to: 1,
                                from: 2,
                                amount: 2.34,
                            },
                            Transaction {
                                id: 2,
                                to: 1,
                                from: 2,
                                amount: 10.00,
                            },
                        ],
                    },
                    Block {
                        id: 1,
                        transactions: vec![
                            Transaction {
                                id: 3,
                                to: 1,
                                from: 2,
                                amount: 200.00,
                            },
                            Transaction {
                                id: 4,
                                to: 1,
                                from: 2,
                                amount: 3000.00,
                            },
                        ],
                    },
                ]
            }

            #[test]
            fn test_calculate_total_for_existing_id() {
                let blocks = create_blocks();

                let result = blocks.calculate_total(1);
                assert!(result.is_some());
                assert_eq!(result, Some(3212.34));

                let result = blocks.calculate_total(2);
                assert!(result.is_some());
                assert_eq!(result, Some(-3212.34));
            }

            #[test]
            fn test_calculate_total_no_id_returns_none() {
                let blocks = create_blocks();

                let result = blocks.calculate_total(3);
                assert!(result.is_none());
            }

            #[test]
            fn test_contains_transaction() {
                let blocks = create_blocks();
                let result = blocks.contains_transaction(0, 1);
                assert!(result);

                let result = blocks.contains_transaction(1, 1);
                assert_eq!(result, false);
            }

            #[test]
            fn test_contains_transaction_false_if_block_non_existant() {
                let blocks: Blocks = Vec::new();

                let result = blocks.contains_transaction(0, 1);
                assert_eq!(result, false);
            }

            #[test]
            fn test_contains_account() {
                let blocks = create_blocks();

                let result = blocks.contains_account(1);
                assert!(result);

                let result = blocks.contains_account(2);
                assert!(result);

                let result = blocks.contains_account(3);
                assert!(!result);
            }
        }
    }
}
