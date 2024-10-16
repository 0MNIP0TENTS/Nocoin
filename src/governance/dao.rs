// dao.rs

pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: ProposalStatus,
}

pub fn create_proposal(description: String) -> Proposal {
    Proposal {
        id: generate_proposal_id(),
        description,
        votes_for: 0,
        votes_against: 0,
        status: ProposalStatus::Pending,
    }
}

pub fn vote_on_proposal(proposal: &mut Proposal, vote_for: bool) {
    if vote_for {
        proposal.votes_for += 1;
    } else {
        proposal.votes_against += 1;
    }

    if proposal.votes_for > proposal.votes_against {
        proposal.status = ProposalStatus::Approved;
    } else {
        proposal.status = ProposalStatus::Rejected;
    }
}
