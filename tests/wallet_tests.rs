#[cfg(test)]
mod tests {
    use crate::wallet;
    use crate::transactions::Transaction;

    #[test]
    fn test_wallet_creation() {
        let wallet = wallet::create_wallet(false);
        assert_eq!(wallet.balance, 0);
        assert!(wallet.public_key.len() > 0);
        assert!(wallet.private_key.len() > 0);
    }

    #[test]
    fn test_wallet_signature() {
        let wallet = wallet::create_wallet(false);
        let transaction = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100,
            signatures: Vec::new(),
            fee: 1,
            timestamp: 1620000000,
        };
        let signature = wallet::sign_transaction(&wallet, &transaction);
        assert!(signature.len() > 0);
    }
}
