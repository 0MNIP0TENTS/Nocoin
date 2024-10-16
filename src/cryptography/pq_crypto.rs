// pq_crypto.rs

extern crate oqs;  // Open Quantum Safe library

pub fn generate_keypair() -> (Vec<u8>, Vec<u8>) {
    // Use Kyber512 for keypair generation, offering post-quantum security
    let kem = oqs::kem::Kem::new(oqs::kem::Algorithm::Kyber512).unwrap();
    kem.keypair().unwrap()
}

pub fn sign_message(private_key: Vec<u8>, message: &[u8]) -> Vec<u8> {
    // Sign message using a lattice-based scheme
    let sig_alg = oqs::sig::Sig::new(oqs::sig::Algorithm::Dilithium2).unwrap();
    sig_alg.sign(message, &private_key).unwrap()
}

pub fn verify_signature(public_key: Vec<u8>, message: &[u8], signature: Vec<u8>) -> bool {
    // Verify signature for authenticity and integrity
    let sig_alg = oqs::sig::Sig::new(oqs::sig::Algorithm::Dilithium2).unwrap();
    sig_alg.verify(message, &signature, &public_key).is_ok()
}
