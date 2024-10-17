#[test]
fn test_sign_and_verify_transaction() {
    let wallet = create_wallet();
    let transaction = Transaction::new("sender", "receiver", 100);
    
    let signature = sign_transaction(&wallet, &transaction);
    assert!(verify_signature(&wallet.public_key, &transaction.to_bytes(), &signature));
}

#[test]
fn test_validator_selection() {
    let validators = vec![
        Validator { public_key: vec![1, 2, 3], stake: 1000, reputation_score: 1.0, carbon_score: 0.9 },
        Validator { public_key: vec![4, 5, 6], stake: 2000, reputation_score: 1.0, carbon_score: 0.9 },
    ];
    
    let selected = select_validator(&validators);
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().stake, 2000);
}

#[test]
fn test_oracle_data_integration() {
    let contract = SmartContract::new();
    let oracle_data = fetch_oracle_data(&contract);
    
    assert!(oracle_data.is_valid());
}
