// transactions.rs

pub struct Transaction {
    pub sender: Vec<u8>,       // Public key of the sender
    pub receiver: Vec<u8>,     // Public key of the receiver
    pub amount: u64,           // Transaction amount
    pub signature: Vec<u8>,    // Quantum-safe signature of the sender
    pub timestamp: u64,        // Transaction timestamp
}

pub fn create_transaction(sender: Vec<u8>, receiver: Vec<u8>, amount: u64, private_key: Vec<u8>) -> Transaction {
    let tx = Transaction {
        sender,
        receiver,
        amount,
        signature: vec![],
        timestamp: current_timestamp(),
    };

    let serialized_tx = serialize_transaction(&tx);
    let signature = sign_message(private_key, &serialized_tx);
    Transaction {
        signature,
        ..tx
    }
}

pub fn verify_transaction(tx: &Transaction, public_key: Vec<u8>) -> bool {
    // Verify the transaction's quantum-safe signature
    let serialized_tx = serialize_transaction(&tx);
    verify_signature(public_key, &serialized_tx, tx.signature.clone())
}
