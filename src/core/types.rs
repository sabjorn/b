pub type AccountId = i64;
pub type TransactionId = i64;
pub type BlockId = i64;
pub type Transactions = Vec<Transaction>;
pub type Blocks = Vec<Block>;

#[derive(Clone)]
pub struct Transaction {
    pub id: TransactionId,
    pub to: AccountId,
    pub from: AccountId,
    pub amount: f64,
}

pub struct Block {
    pub id: BlockId,
    pub transactions: Transactions,
}

pub trait TransactionInfo {
    fn calculate_total(&self, account: AccountId) -> Option<f64>;
}

pub trait BlockInfo {
    fn contains_transaction(&self, block: BlockId, transaction: TransactionId) -> bool;
}

impl TransactionInfo for Transactions {
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

impl TransactionInfo for Block {
    fn calculate_total(&self, account: AccountId) -> Option<f64> {
        self.transactions.calculate_total(account)
    }
}

impl TransactionInfo for Blocks {
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

        #[test]
        fn test_calculate_total_for_existing_id() {
            let transactions: Transactions = vec![
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
            ];

            let result = transactions.calculate_total(1);
            assert!(result.is_some());
            assert_eq!(result, Some(12.34));

            let result = transactions.calculate_total(2);
            assert!(result.is_some());
            assert_eq!(result, Some(-12.34));
        }

        #[test]
        fn test_calculate_total_no_id_returns_none() {
            let transactions = vec![
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
            ];

            let result = transactions.calculate_total(3);
            assert!(result.is_none());
        }

        mod block_tests {
            use super::*;

            #[test]
            fn test_calculate_total_for_existing_id() {
                let block = Block {
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
                };

                let result = block.calculate_total(1);
                assert!(result.is_some());
                assert_eq!(result, Some(12.34));

                let result = block.calculate_total(2);
                assert!(result.is_some());
                assert_eq!(result, Some(-12.34));
            }

            #[test]
            fn test_calculate_total_no_id_returns_none() {
                let block = Block {
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
                };

                let result = block.calculate_total(3);
                assert!(result.is_none());
            }
        }

        mod blocks_tests {
            use super::*;

            #[test]
            fn test_calculate_total_for_existing_id() {
                let blocks = vec![
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
                ];

                let result = blocks.calculate_total(1);
                assert!(result.is_some());
                assert_eq!(result, Some(3212.34));

                let result = blocks.calculate_total(2);
                assert!(result.is_some());
                assert_eq!(result, Some(-3212.34));
            }

            #[test]
            fn test_calculate_total_no_id_returns_none() {
                let blocks = vec![
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
                ];

                let result = blocks.calculate_total(3);
                assert!(result.is_none());
            }

            #[test]
            fn test_contains_transaction() {
                let blocks = vec![
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
                ];

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
        }
    }
}
