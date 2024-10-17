// src/consensus.rs

use crate::enhanced_pos::Validator;
use rand::Rng;
use rayon::prelude::*;

pub fn select_validator_parallel(validators: &[Validator]) -> Option<&Validator> {
    // Calculate total stake of non-slashed validators
    let total_stake: u64 = validators
        .par_iter()
        .filter(|v| !v.slashed)
        .map(|v| v.stake)
        .sum();

    if total_stake == 0 {
        return None;
    }

    // Generate a random stake chance
    let mut rng = rand::thread_rng();
    let stake_chance: u64 = rng.gen_range(0..total_stake);

    // Iterate through validators to find the selected one
    let mut cumulative_stake = 0;
    for validator in validators {
        if validator.slashed {
            continue;
        }
        cumulative_stake += validator.stake;
        if stake_chance < cumulative_stake {
            return Some(validator);
        }
    }

    None
}
