// consensus.rs

pub struct Validator {
    pub stake: u64,           // Staked Nocoin™ tokens
    pub public_key: Vec<u8>,   // Quantum-resistant public key
    pub signature: Vec<u8>,    // Validator's quantum-safe signature
}

pub struct Block {
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub validator_signature: Vec<u8>, // Quantum-safe signature of the block
    pub timestamp: u64,
}

// Validator selection based on post-quantum randomness
pub fn select_validator(validators: &Vec<Validator>) -> Validator {
    // Use a post-quantum random number generator for unbiased selection
    let random_index = post_quantum_random(validators.len());
    validators[random_index].clone()
}

pub fn validate_block(block: &Block, validators: &Vec<Validator>) -> bool {
    // Ensure block is signed by the correct validator
    let validator = get_validator_by_signature(&block.validator_signature, validators);
    if validator.is_none() {
        return false;
    }

    // Verify the validator’s quantum-safe signature
    verify_signature(&block.validator_signature, &validator.unwrap().public_key)
}
