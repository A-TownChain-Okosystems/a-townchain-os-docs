use crate::{Delegation, Proposal, ProposalStatus, Timelock, Treasury, VoteType, VotingSystem};
use std::collections::BTreeMap;

pub struct Dao {
    pub proposals: BTreeMap<u64, Proposal>,
    pub voting: VotingSystem,
    pub treasury: Treasury,
    pub timelock: Timelock,
    pub delegation: Delegation,
    pub quorum: u64,
    pub approval_bps: u16,
}
impl Dao {
    pub fn new(
        initial_treasury: u64,
        timelock_blocks: u64,
        quorum: u64,
        approval_bps: u16,
    ) -> Result<Self, String> {
        if quorum == 0 {
            return Err("quorum must be non-zero".into());
        }
        if approval_bps > 10_000 {
            return Err("approval threshold exceeds 10000 bps".into());
        }
        Ok(Self {
            proposals: BTreeMap::new(),
            voting: VotingSystem::new(),
            treasury: Treasury::new(initial_treasury.into()),
            timelock: Timelock::new(timelock_blocks),
            delegation: Delegation::new(),
            quorum,
            approval_bps,
        })
    }
    pub fn create_proposal(&mut self, p: Proposal) -> Result<(), String> {
        if self.proposals.contains_key(&p.id) {
            return Err("proposal id already exists".into());
        }
        self.proposals.insert(p.id, p);
        Ok(())
    }
    pub fn activate(&mut self, id: u64, block: u64) -> Result<(), String> {
        self.proposals
            .get_mut(&id)
            .ok_or("proposal not found".to_string())?
            .activate(block)
    }
    pub fn vote(
        &mut self,
        id: u64,
        voter: &str,
        kind: VoteType,
        weight: u64,
    ) -> Result<(), String> {
        let p = self
            .proposals
            .get(&id)
            .ok_or("proposal not found".to_string())?;
        if p.status != ProposalStatus::Active {
            return Err("proposal is not active".into());
        }
        self.voting.cast_vote(voter, id, kind, weight)
    }
    pub fn close(&mut self, id: u64, block: u64) -> Result<ProposalStatus, String> {
        let p = self
            .proposals
            .get_mut(&id)
            .ok_or("proposal not found".to_string())?;
        if block < p.end_block {
            return Err("voting window is still open".into());
        }
        let (f, a, x) = self.voting.tally(id);
        p.for_votes = f;
        p.against_votes = a;
        p.abstain_votes = x;
        p.tally(self.quorum, self.approval_bps)?;
        if p.status == ProposalStatus::Passed {
            self.timelock.queue(id, block);
            p.status = ProposalStatus::Queued
        }
        Ok(p.status)
    }
    pub fn execute(&mut self, id: u64, block: u64) -> Result<(), String> {
        if !self.timelock.is_ready(id, block) {
            return Err("timelock not expired".into());
        }
        let p = self
            .proposals
            .get_mut(&id)
            .ok_or("proposal not found".to_string())?;
        if p.status != ProposalStatus::Queued {
            return Err("proposal is not queued".into());
        }
        self.timelock.execute(id, block)?;
        p.status = ProposalStatus::Executed;
        Ok(())
    }
}
