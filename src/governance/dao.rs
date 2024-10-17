pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub proposer: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub deadline: u64,
}

pub fn submit_proposal(proposer: String, description: String) -> Proposal {
    Proposal {
        id: generate_proposal_id(),
        description,
        proposer,
        votes_for: 0,
        votes_against: 0,
        deadline: current_timestamp() + 604800,
    }
}

pub fn vote_on_proposal(proposal: &mut Proposal, voter: &Voter, support: bool) {
    if support {
        proposal.votes_for += voter.voting_weight;
    } else {
        proposal.votes_against += voter.voting_weight;
    }
}

pub fn finalize_proposal(proposal: &Proposal) -> Result<&'static str, &'static str> {
    if current_timestamp() > proposal.deadline {
        if proposal.votes_for > proposal.votes_against {
            Ok("Proposal Approved")
        } else {
            Ok("Proposal Rejected")
        }
    } else {
        Err("Voting is still ongoing")
    }
}
