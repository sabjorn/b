pub type AccountId = i64;
pub type TranscationId = i64;
pub type BlockId = i64;
type Transactions = Vec<Transaction>;

pub struct Transaction {
    id: TranscationId,
    to: AccountId,
    from: AccountId,
    amount: f64,
}

pub struct Block {
    id: BlockId,
    transactions: Transactions,
}

trait TransactionTotal {
    fn calculate_total(&self, account: AccountId) -> Option<f64>;
}

impl TransactionTotal for Transactions {
    fn calculate_total(&self, account: AccountId) -> Option<f64> {
        let mut found = false;
        let sum: f64 = self
            .into_iter()
            .filter(|t| {
                if t.to == account || t.from == account {
                    found = true;
                    return true;
                }

                false
            })
            .map(|t| {
                if t.to == account {
                    return t.amount;
                }
                if t.from == account {
                    return -t.amount;
                }

                0.0
            })
            .sum();

        if found {
            return Some(sum);
        }

        None
    }
}

impl TransactionTotal for Block {
    fn calculate_total(&self, account: AccountId) -> Option<f64> {
        self.transactions.calculate_total(account)
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
    }
}
