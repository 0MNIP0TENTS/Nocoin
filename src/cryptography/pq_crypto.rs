// pq_crypto.rs

extern crate oqs;  // Open Quantum Safe library for post-quantum algorithms

pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    // Generate a quantum-safe public/private keypair using Kyber512
    let kem = oqs::kem::Kem::new(oqs::kem::Algorithm::Kyber512).unwrap();
    kem.keypair().unwrap()
}

pub fn sign_message(private_key: Vec<u8>, message: &[u8]) -> Vec<u8> {
    // Sign a message using a quantum-safe signature scheme (e.g., Dilithium2)
    let sig_alg = oqs::sig::Sig::new(oqs::sig::Algorithm::Dilithium2).unwrap();
    sig_alg.sign(message, &private_key).unwrap()
}

pub fn verify_signature(public_key: Vec<u8>, message: &[u8], signature: Vec<u8>) -> bool {
    // Verify a quantum-safe signature
    let sig_alg = oqs::sig::Sig::new(oqs::sig::Algorithm::Dilithium2).unwrap();
    sig_alg.verify(message, &signature, &public_key).is_ok()
}
