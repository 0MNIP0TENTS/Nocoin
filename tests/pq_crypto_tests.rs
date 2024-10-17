#[cfg(test)]
mod tests {
    use crate::pq_crypto;

    #[test]
    fn test_pq_keypair_generation() {
        let (public_key, private_key) = pq_crypto::generate_post_quantum_keypair();
        assert!(public_key.len() > 0);
        assert!(private_key.len() > 0);
    }

    #[test]
    fn test_pq_signature_verification() {
        let (public_key, private_key) = pq_crypto::generate_post_quantum_keypair();
        let message = b"Hello, Nocoin!";
        let signature = pq_crypto::sign_message(&private_key, message, b"nonce");
        assert!(pq_crypto::verify_signature(&[signature]));
    }
}
