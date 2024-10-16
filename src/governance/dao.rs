// dao.rs

pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: ProposalStatus,
}

pub fn create_proposal(description: &str) -> Proposal {
    Proposal {
        id: generate_proposal_id(),
        description: description.to_string(),
        votes_for: 0,
        votes_against: 0,
        status: ProposalStatus::Pending,
    }
}

pub fn vote_on_proposal(proposal_id: u64, vote: Vote) {
    update_vote_count(proposal_id, vote);
}
