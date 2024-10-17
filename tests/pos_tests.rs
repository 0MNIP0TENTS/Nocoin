#[cfg(test)]
mod tests {
    use crate::enhanced_pos::{Validator, select_validator, slash_validator};

    #[test]
    fn test_validator_selection() {
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
                reputation_score: 1.0,
                carbon_score: 0.3,
                slashed: false,
            },
        ];
        let selected = select_validator(&validators);
        assert!(selected.is_some());
    }

    #[test]
    fn test_validator_slashing() {
        let mut validator = Validator {
            public_key: vec![1, 2, 3],
            stake: 1000,
            reputation_score: 1.0,
            carbon_score: 0.2,
            slashed: false,
        };
        slash_validator(&mut validator);
        assert!(validator.slashed);
    }
}
