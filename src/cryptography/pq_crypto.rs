  // cryptography.rs

extern crate oqs;  // Open Quantum Safe library

pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    // Generate quantum-resistant public/private keypair using Kyber512
    let kem = oqs::kem::Kem::new(oqs::kem::Algorithm::Kyber512).unwrap();
    let (public_key, private_key) = kem.keypair().unwrap();
    (public_key, private_key)
}

pub fn sign_transaction(private_key: Vec<u8>, message: &[u8]) -> Vec<u8> {
    // Use a quantum-safe signature scheme for signing transactions
    let sig_alg = oqs::sig::Sig::new(oqs::sig::Algorithm::Dilithium2).unwrap();
    let signature = sig_alg.sign(message, &private_key).unwrap();
    signature
}

pub fn verify_signature(public_key: Vec<u8>, message: &[u8], signature: Vec<u8>) -> bool {
    // Verify a post-quantum signature
    let sig_alg = oqs::sig::Sig::new(oqs::sig::Algorithm::Dilithium2).unwrap();
    sig_alg.verify(message, &signature, &public_key).is_ok()
}
