// src/governance.rs
use std::collections::HashMap;

pub struct Proposal {
    pub id: u64,
    pub proposer: String,
    pub description: String,
    pub votes_for: u64,
    pub votes_against: u64,
    pub executed: bool,
}

pub struct DAO {
    pub proposals: HashMap<u64, Proposal>,
    pub next_proposal_id: u64,
}

impl DAO {
    pub fn new() -> Self {
        Self {
            proposals: HashMap::new(),
            next_proposal_id: 1,
        }
    }

    pub fn create_proposal(&mut self, proposer: String, description: String) -> u64 {
        let proposal_id = self.next_proposal_id;
        let proposal = Proposal {
            id: proposal_id,
            proposer,
            description,
            votes_for: 0,
            votes_against: 0,
            executed: false,
        };
        self.proposals.insert(proposal_id, proposal);
        self.next_proposal_id += 1;
        proposal_id
    }

    pub fn vote_on_proposal(&mut self, proposal_id: u64, vote_for: bool) {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if vote_for {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }
        }
    }

    pub fn execute_proposal(&mut self, proposal_id: u64) {
        if let Some(proposal) = self.proposals.get_mut(&proposal_id) {
            if proposal.votes_for > proposal.votes_against && !proposal.executed {
                // Logic for executing the proposal
                proposal.executed = true;
                println!("Proposal {} executed.", proposal_id);
            }
        }
    }
}
