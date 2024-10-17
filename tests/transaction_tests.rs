#[cfg(test)]
mod tests {
    use crate::transactions;
    use crate::pq_crypto;

    #[test]
    fn test_transaction_creation() {
        let private_key = vec![1, 2, 3];
        let transaction = transactions::create_transaction(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
            &private_key,
            1
        );
        assert_eq!(transaction.sender, "Alice");
        assert_eq!(transaction.receiver, "Bob");
        assert_eq!(transaction.amount, 100);
    }

    #[tokio::test]
    async fn test_async_transaction_validation() {
        let private_key = vec![1, 2, 3];
        let transaction = transactions::create_transaction(
            "Alice".to_string(),
            "Bob".to_string(),
            100,
            &private_key,
            1
        );
        let is_valid = transactions::validate_transaction_async(&transaction).await;
        assert!(is_valid);
    }
}
