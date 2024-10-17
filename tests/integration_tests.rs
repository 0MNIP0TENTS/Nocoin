// tests/integration_tests.rs

#[cfg(test)]
mod tests {
    use crate::NocoinBlockchain;
    use crate::wallet::create_wallet;
    use crate::enhanced_pos::Validator;

    #[test]
    fn test_full_transaction_flow() {
        let mut blockchain = NocoinBlockchain::new();

        // Create and add a wallet
        let wallet = create_wallet(false);
        blockchain.add_wallet(wallet);

        // Add validators
        blockchain.add_validator(Validator {
            public_key: vec![1, 2, 3],
            stake: 1000,
            reputation_score: 1.0,
            carbon_score: 0.2,
            slashed: false,
        });

        // Create a transaction
        let private_key = blockchain.wallets[0].private_key.clone();
        let transaction = blockchain.create_transaction("sender_id", "receiver_id", 100, &private_key, 1);

        // Validate the transaction asynchronously
        let rt = tokio::runtime::Runtime::new().unwrap();
        let is_valid = rt.block_on(crate::transactions::validate_transaction_async(&transaction));
        assert!(is_valid);

        // Generate zk-SNARK proof
        let zk_proof = blockchain.generate_zk_proof(&transaction, false);
        assert!(zk_proof.is_valid());

        // Select a validator
        let selected = blockchain.select_validator();
        assert!(selected.is_some());
    }
}
