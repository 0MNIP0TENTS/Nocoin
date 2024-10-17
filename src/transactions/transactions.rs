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
        signature: sign_transaction(private_key, &format!("{}{}{}", sender, receiver, amount).as_bytes()),
        timestamp: current_timestamp(),
    };
    transaction
}

pub fn adapt_transaction_fee(transaction: &mut Transaction, network_load: u64) {
    if network_load < 500 {
        transaction.fee = 0;
    } else if network_load < 1000 {
        transaction.fee = transaction.amount / 1000;
    } else {
        transaction.fee = transaction.amount / 500;
    }
}

pub fn broadcast_transaction(transaction: &Transaction) {
    p2p::broadcast(transaction);
}
