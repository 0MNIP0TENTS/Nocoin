pub fn generate_post_quantum_keypair(algorithm: PQAlgorithm) -> (Vec<u8>, Vec<u8>) {
    match algorithm {
        PQAlgorithm::Dilithium => dilithium2::keypair(),
        PQAlgorithm::Falcon => falcon::keypair(),
    }
}

pub fn sign_message(private_key: &[u8], message: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut message_with_nonce = vec![];
    message_with_nonce.extend_from_slice(message);
    message_with_nonce.extend_from_slice(nonce);
    dilithium2::sign(&message_with_nonce, private_key)
}

pub fn verify_signature(public_key: &[u8], message: &[u8], signature: &[u8], nonce: &[u8]) -> bool {
    let mut message_with_nonce = vec![];
    message_with_nonce.extend_from_slice(message);
    message_with_nonce.extend_from_slice(nonce);
    dilithium2::verify(&message_with_nonce, signature, public_key).is_ok()
}

pub fn generate_nonce() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8; 16];
    rng.fill_bytes(&mut nonce);
    nonce.to_vec()
}
