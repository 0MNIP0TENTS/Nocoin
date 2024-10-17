// src/wallet.rs
use crate::pq_crypto::generate_post_quantum_keypair;
use crate::transactions::Transaction;

pub struct Wallet {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub balance: u64,
    pub decentralized_id: Option<String>,
    pub cold_storage: bool,
}

pub fn create_wallet(cold_storage: bool) -> Wallet {
    let (public_key, private_key) = generate_post_quantum_keypair();
    Wallet {
        public_key,
        private_key,
        balance: 0,
        decentralized_id: None,
        cold_storage,
    }
}

pub fn sign_transaction(wallet: &Wallet, transaction: &Transaction) -> Vec<u8> {
    crate::pq_crypto::sign_message(&wallet.private_key, &transaction.to_string().as_bytes(), b"nonce")
}
