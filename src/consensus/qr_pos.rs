pub struct Validator {
    pub public_key: Vec<u8>,
    pub stake: u64,
    pub reputation_score: f64,
    pub carbon_score: f64,
    pub slashed: bool,
}

pub struct Block {
    pub previous_hash: String,
    pub transactions: Vec<Transaction>,
    pub validator_signature: Vec<u8>,
    pub timestamp: u64,
}

pub fn select_validator(validators: &[Validator]) -> Option<&Validator> {
    let total_stake: u64 = validators.iter().filter(|v| !v.slashed).map(|v| v.stake).sum();
    let mut rng = rand::thread_rng();
    let random_value = rng.gen_range(0..total_stake);

    let mut cumulative_stake = 0;
    for validator in validators.iter().filter(|v| !v.slashed) {
        cumulative_stake += validator.stake;
        if cumulative_stake >= random_value {
            return Some(validator);
        }
    }
    None
}

pub fn validate_and_reward_block(block: &Block, validators: &mut [Validator], eco_factor: f64) -> bool {
    if let Some(validator) = get_validator_by_signature(&block.validator_signature, validators) {
        if verify_signature(&validator.public_key, &block.hash(), &block.validator_signature) {
            reward_validator(validator, eco_factor);
            return true;
        }
    }
    false
}

pub fn reward_validator(validator: &mut Validator, eco_factor: f64) {
    let base_reward: u64 = 100;
    let carbon_incentive = (eco_factor * validator.carbon_score * 0.1) as u64;
    let final_reward = base_reward + carbon_incentive;
    validator.stake += final_reward;
    validator.reputation_score += 0.05;
}

pub fn slash_validator(validator: &mut Validator, reason: &str) {
    validator.slashed = true;
    validator.stake /= 2;
    validator.reputation_score -= 0.5;
    println!("Validator {} slashed for: {}", validator.public_key.to_hex(), reason);
}
