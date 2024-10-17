// src/hardware_wallet.rs

use crate::transactions::Transaction;
use crate::pq_crypto;

pub struct HardwareWallet {
    pub device_id: String,
}

impl HardwareWallet {
    pub fn new(device_id: String) -> Self {
        Self { device_id }
    }

    pub fn sign_transaction(&self, transaction: &Transaction) -> Vec<u8> {
        // Placeholder for communication with a hardware wallet device
        println!("Signing transaction with hardware wallet device: {}", self.device_id);
        pq_crypto::sign_message(&[1, 2, 3], &transaction.to_string().as_bytes(), b"nonce")
    }
}
