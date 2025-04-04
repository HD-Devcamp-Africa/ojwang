#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Address, Symbol, Vec, Map};

#[contract]
pub struct DaoVoting;

#[derive(Clone)]
pub struct Proposal {
    creator: Address,
    description: Symbol,
    votes_for: u32,
    votes_against: u32,
    end_time: u64,
    executed: bool,
}

#[contractimpl]
impl DaoVoting {
    pub fn create_proposal(env: Env, creator: Address, description: Symbol, duration: u64) -> Symbol {
        let end_time = env.ledger().timestamp() + duration;
        let proposal = Proposal { creator, description, votes_for: 0, votes_against: 0, end_time, executed: false };
        env.storage().set(&Symbol::new("proposal"), &proposal);
        Symbol::new("ProposalCreated")
    }

    pub fn vote(env: Env, voter: Address, support: bool) -> Symbol {
        let mut proposal: Proposal = env.storage().get(&Symbol::new("proposal")).unwrap();

        if env.ledger().timestamp() > proposal.end_time {
            return Symbol::new("VotingClosed");
        }

        if support {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }

        env.storage().set(&Symbol::new("proposal"), &proposal);
        Symbol::new("VoteCast")
    }

    pub fn execute_proposal(env: Env) -> Symbol {
        let mut proposal: Proposal = env.storage().get(&Symbol::new("proposal")).unwrap();

        if env.ledger().timestamp() <= proposal.end_time {
            return Symbol::new("VotingStillOpen");
        }

        if proposal.executed {
            return Symbol::new("AlreadyExecuted");
        }

        if proposal.votes_for > proposal.votes_against {
            proposal.executed = true;
            env.storage().set(&Symbol::new("proposal"), &proposal);
            Symbol::new("ProposalExecuted")
        } else {
            Symbol::new("ProposalRejected")
        }
    }
}
