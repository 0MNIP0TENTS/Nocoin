// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract NocoinGovernance {
    struct Proposal {
        string description;
        uint256 voteCount;
        mapping(address => bool) voters;
    }

    Proposal[] public proposals;
    address public chairperson;

    constructor() {
        chairperson = msg.sender;
    }

    function createProposal(string memory description) public {
        require(msg.sender == chairperson, "Only chairperson can create proposals");
        proposals.push(Proposal({description: description, voteCount: 0}));
    }

    function vote(uint256 proposalIndex) public {
        Proposal storage proposal = proposals[proposalIndex];
        require(!proposal.voters[msg.sender], "You have already voted");
        proposal.voters[msg.sender] = true;
        proposal.voteCount++;
    }

    function getProposal(uint256 proposalIndex) public view returns (string memory, uint256) {
        Proposal storage proposal = proposals[proposalIndex];
        return (proposal.description, proposal.voteCount);
    }
}
