#[cfg(test)]
mod tests {
    use super::transaction;

    #[test]
    fn test_create_and_verify_transaction() {
        let sender_public_key = vec![1, 2, 3];
        let receiver_public_key = vec![4, 5, 6];
        let private_key = vec![7, 8, 9];

        let tx = transaction::create_transaction(
            &sender_public_key, 
            &receiver_public_key, 
            100, 
            private_key.clone()
        );

        assert!(transaction::verify_transaction(&tx, sender_public_key));
    }
}
