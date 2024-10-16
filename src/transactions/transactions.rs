// transactions.rs

pub struct Transaction {
    pub sender: Vec<u8>,
    pub receiver: Vec<u8>,
    pub amount: u64,
    pub signature: Vec<u8>,  // Quantum-safe signature
    pub timestamp: u64,
}

pub fn create_transaction(sender: Vec<u8>, receiver: Vec<u8>, amount: u64, private_key: Vec<u8>) -> Transaction {
    let tx = Transaction {
        sender: sender.clone(),
        receiver,
        amount,
        signature: vec![],
        timestamp: get_current_timestamp(),
    };

    let serialized_tx = serialize_transaction(&tx);
    let signature = sign_transaction(private_key, &serialized_tx);
    Transaction {
        signature,
        ..tx
    }
}

pub fn verify_transaction(tx: &Transaction, public_key: Vec<u8>) -> bool {
    // Verify the transaction's signature using quantum-safe methods
    let serialized_tx = serialize_transaction(&tx);
    verify_signature(public_key, &serialized_tx, tx.signature.clone())
}
