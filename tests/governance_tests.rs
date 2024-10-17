#[cfg(test)]
mod tests {
    use crate::governance::DAO;

    #[test]
    fn test_proposal_creation() {
        let mut dao = DAO::new();
        let proposal_id = dao.create_proposal("Alice".to_string(), "Increase block size".to_string());
        assert_eq!(proposal_id, 1);
        assert!(dao.proposals.contains_key(&proposal_id));
    }

    #[test]
    fn test_voting_on_proposal() {
        let mut dao = DAO::new();
        let proposal_id = dao.create_proposal("Alice".to_string(), "Increase block size".to_string());
        dao.vote_on_proposal(proposal_id, true);
        assert_eq!(dao.proposals.get(&proposal_id).unwrap().votes_for, 1);
    }

    #[test]
    fn test_proposal_execution() {
        let mut dao = DAO::new();
        let proposal_id = dao.create_proposal("Alice".to_string(), "Increase block size".to_string());
        dao.vote_on_proposal(proposal_id, true);
        dao.execute_proposal(proposal_id);
        assert!(dao.proposals.get(&proposal_id).unwrap().executed);
    }
}
