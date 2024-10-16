// wallet.rs

pub struct Wallet {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

pub fn create_wallet() -> Wallet {
    let (public_key, private_key) = generate_keypair();
    Wallet {
        public_key,
        private_key,
    }
}

pub fn sign_transaction(wallet: &Wallet, transaction: &Transaction) -> Transaction {
    let serialized_tx = serialize_transaction(transaction);
    let signature = sign_message(wallet.private_key.clone(), &serialized_tx);
    Transaction {
        signature,
        ..transaction.clone()
    }
}
