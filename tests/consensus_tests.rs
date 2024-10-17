// tests/consensus_tests.rs

#[cfg(test)]
mod tests {
    use crate::consensus::select_validator_parallel;
    use crate::enhanced_pos::Validator;

    #[test]
    fn test_parallel_validator_selection() {
        // Create a set of validators with different stakes
        let validators = vec![
            Validator {
                public_key: vec![1, 2, 3],
                stake: 1000,
                reputation_score: 1.0,
                carbon_score: 0.2,
                slashed: false,
            },
            Validator {
                public_key: vec![4, 5, 6],
                stake: 500,
                reputation_score: 0.8,
                carbon_score: 0.3,
                slashed: false,
            },
            Validator {
                public_key: vec![7, 8, 9],
                stake: 2000,
                reputation_score: 1.2,
                carbon_score: 0.1,
                slashed: false,
            },
        ];

        // Test that a validator is selected
        let selected = select_validator_parallel(&validators);
        assert!(selected.is_some());

        // Test that the validator has a non-zero public key
        if let Some(validator) = selected {
            assert!(validator.public_key.len() > 0);
        }
    }

    #[test]
    fn test_slashed_validator_exclusion() {
        // Create a set of validators with one slashed validator
        let validators = vec![
            Validator {
                public_key: vec![1, 2, 3],
                stake: 1000,
                reputation_score: 1.0,
                carbon_score: 0.2,
                slashed: true,  // This validator is slashed
            },
            Validator {
                public_key: vec![4, 5, 6],
                stake: 500,
                reputation_score: 0.8,
                carbon_score: 0.3,
                slashed: false,
            },
            Validator {
                public_key: vec![7, 8, 9],
                stake: 2000,
                reputation_score: 1.2,
                carbon_score: 0.1,
                slashed: false,
            },
        ];

        // Ensure that the slashed validator is not selected
        let selected = select_validator_parallel(&validators);
        assert!(selected.is_some());

        // Ensure that the selected validator is not the slashed one
        if let Some(validator) = selected {
            assert!(!validator.slashed);
        }
    }

    #[test]
    fn test_validator_selection_with_equal_stakes() {
        // Create validators with equal stakes
        let validators = vec![
            Validator {
                public_key: vec![1, 2, 3],
                stake: 1000,
                reputation_score: 1.0,
                carbon_score: 0.2,
                slashed: false,
            },
            Validator {
                public_key: vec![4, 5, 6],
                stake: 1000,
                reputation_score: 1.0,
                carbon_score: 0.3,
                slashed: false,
            },
        ];

        // Ensure that one of the validators is selected
        let selected = select_validator_parallel(&validators);
        assert!(selected.is_some());

        // Validate that the selection is consistent with stake
        if let Some(validator) = selected {
            assert!(
                validator.public_key == vec![1, 2, 3] || validator.public_key == vec![4, 5, 6]
            );
        }
    }

    #[test]
    fn test_no_validators_case() {
        // Test when there are no validators
        let validators: Vec<Validator> = vec![];
        let selected = select_validator_parallel(&validators);
        // Should return None since no validators are available
        assert!(selected.is_none());
    }

    #[test]
    fn test_all_validators_slashed() {
        // Test when all validators are slashed
        let validators = vec![
            Validator {
                public_key: vec![1, 2, 3],
                stake: 1000,
                reputation_score: 1.0,
                carbon_score: 0.2,
                slashed: true,
            },
            Validator {
                public_key: vec![4, 5, 6],
                stake: 500,
                reputation_score: 0.8,
                carbon_score: 0.3,
                slashed: true,
            },
        ];

        let selected = select_validator_parallel(&validators);
        assert!(selected.is_none());
    }
}
