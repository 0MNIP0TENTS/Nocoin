pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signature: Vec<u8>,
    pub fee: u64,
    pub timestamp: u64,
}

pub fn create_transaction(sender: String, receiver: String, amount: u64, private_key: &[u8], fee: u64) -> Transaction {
    let transaction = Transaction {
        sender: sender.clone(),
        receiver: receiver.clone(),
        amount,
        fee,
        signature: sign_transaction(private_key, &format!("{}{}{}", sender, receiver, amount)),
        timestamp: current_timestamp(),
    };
    transaction
}

pub struct MultiSigTransaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
    pub signatures: Vec<Vec<u8>>,  // Collects multiple signatures
    pub fee: u64,
    pub timestamp: u64,
}

pub fn create_multi_sig_transaction(sender: String, receiver: String, amount: u64, fee: u64) -> MultiSigTransaction {
    MultiSigTransaction {
        sender,
        receiver,
        amount,
        signatures: vec![],  // Initially empty, signatures will be added
        fee,
        timestamp: current_timestamp(),
    }
}

pub fn add_signature_to_multi_sig(transaction: &mut MultiSigTransaction, signature: Vec<u8>) {
    transaction.signatures.push(signature);
}

pub fn is_multi_sig_valid(transaction: &MultiSigTransaction, wallet: &MultiSigWallet) -> bool {
    wallet.threshold <= transaction.signatures.len() as u8
}

// Support for batching transactions
pub struct BatchTransaction {
    pub transactions: Vec<Transaction>,
}

pub fn create_batch_transaction(transactions: Vec<Transaction>) -> BatchTransaction {
    BatchTransaction { transactions }
}
