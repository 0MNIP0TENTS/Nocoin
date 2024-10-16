// zk_snarks.rs

extern crate snarkjs;

pub async fn create_zk_proof(input: &str) -> (String, String) {
    let (proof, public_signals) = snarkjs::groth16::full_prove(input, "circuit.wasm", "circuit_final.zkey").await;
    (proof, public_signals)
}

pub async fn verify_zk_proof(proof: &str, public_signals: &str) -> bool {
    let verification_key = std::fs::read_to_string("verification_key.json").unwrap();
    snarkjs::groth16::verify(&verification_key, public_signals, proof).await
}
