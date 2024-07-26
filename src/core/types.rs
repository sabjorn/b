use std::collections::HashMap;

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

trait TransactionTotal {
    fn calculate_total(&self, account: AccountId) -> Option<f64>;
}

impl TransactionTotal for Transactions {
    fn calculate_total(&self, account: AccountId) -> Option<f64> {
        let mut sum = 0.;
        let mut found = false;

        for t in self {
            if t.to == account {
                sum += t.amount;
                found = true;
            }
            if t.from == account {
                sum -= t.amount;
                found = true;
            }
        }

        if found {
            Some(sum)
        } else {
            None
        }
    }
}

struct Block {
    id: BlockId,
    transactions: Transactions,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_total_for_existing_id(){
        let transactions: Transactions = vec!(
            Transaction{ 
                id: 1,
                to: 1,
                from: 2,
                amount: 2.34
            },
            Transaction{ 
                id: 2,
                to: 1,
                from: 2,
                amount: 10.00
            });

        let result = transactions.calculate_total(1);
        assert!(result.is_some());
        assert_eq!(result, Some(12.34));

        let result = transactions.calculate_total(2);
        assert!(result.is_some());
        assert_eq!(result, Some(-12.34));
    }

    #[test]
    fn test_calculate_total_no_id_returns_none(){
        let transactions: Transactions = vec!(
            Transaction{ 
                id: 1,
                to: 1,
                from: 2,
                amount: 2.34
            },
            Transaction{ 
                id: 2,
                to: 1,
                from: 2,
                amount: 10.00
            });

        let result = transactions.calculate_total(3);
        assert!(result.is_none());
    }
}
