use super::types::Id;
use sha2::{Digest, Sha256};
use std::time::Duration;

pub fn generate_id(transaction_id: Id, account_id: Id, amount: f64, duration: Duration) -> Id {
    let current_time = duration.as_secs();

    let mut hasher = Sha256::new();

    hasher.update(transaction_id.to_be_bytes());
    hasher.update(account_id.to_be_bytes());
    hasher.update(amount.to_be_bytes());
    hasher.update(current_time.to_be_bytes());

    let result = hasher.finalize();

    let mut int_bytes = [0u8; 8];
    int_bytes.copy_from_slice(&result[0..8]);
    u64::from_be_bytes(int_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id_produces_same_output() {
        let duration = Duration::new(0, 0);

        let id_1 = generate_id(1, 2, 1.1, duration);
        let id_2 = generate_id(1, 2, 1.1, duration);

        assert_eq!(id_1, id_2);
    }

    #[test]
    fn test_generate_id_produces_different_outputs() {
        let id_1 = generate_id(1, 3, 1.1, Duration::new(0, 0));
        let id_2 = generate_id(1, 2, 1.1, Duration::new(0, 0));
        assert_ne!(id_1, id_2);

        let id_1 = generate_id(2, 2, 1.1, Duration::new(0, 0));
        let id_2 = generate_id(1, 2, 1.1, Duration::new(0, 0));
        assert_ne!(id_1, id_2);

        let id_1 = generate_id(1, 2, 3.3, Duration::new(0, 0));
        let id_2 = generate_id(1, 2, 1.1, Duration::new(0, 0));
        assert_ne!(id_1, id_2);

        let id_1 = generate_id(1, 2, 1.1, Duration::new(0, 0));
        let id_2 = generate_id(1, 2, 1.1, Duration::new(1, 0));
        assert_ne!(id_1, id_2);
    }
}
