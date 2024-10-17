#[cfg(test)]
mod tests {
    use crate::zk_snarks;
    use crate::transactions::Transaction;

    #[test]
    fn test_zk_proof_generation() {
        let transaction = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100,
            signatures: Vec::new(),
            fee: 1,
            timestamp: 1620000000,
        };
        let proof = zk_snarks::generate_zk_proof(&transaction, false);
        assert!(proof.is_valid());
    }

    #[test]
    fn test_zk_proof_verification() {
        let transaction = Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 100,
            signatures: Vec::new(),
            fee: 1,
            timestamp: 1620000000,
        };
        let proof = zk_snarks::generate_zk_proof(&transaction, false);
        assert!(zk_snarks::verify_zk_proof(&proof));
    }
}
