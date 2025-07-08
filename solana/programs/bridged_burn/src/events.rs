use anchor_lang::prelude::*;

#[event]
pub struct ConfigInitializedEvent {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub multisig_threshold: u8,
    pub multisig_signers: Vec<Pubkey>,
}

#[event]
pub struct ConfigProposalCreatedEvent {
    pub proposal_id: u64,
    pub proposer: Pubkey,
    pub expires_at: i64,
}

#[event]
pub struct ConfigProposalApprovedEvent {
    pub proposal_id: u64,
    pub approver: Pubkey,
    pub approval_count: u8,
}

#[event]
pub struct ConfigProposalExecutedEvent {
    pub proposal_id: u64,
    pub executor: Pubkey,
}

#[event]
pub struct EmergencyPauseEvent {
    pub paused_by: Pubkey,
}

#[event]
pub struct BridgeBurnEvent {
    pub sui_receiver: [u8; 32],
    pub sol_sender: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WormholeMessageEvent {
    pub target_chain: u16,
    pub payload: Vec<u8>,
    pub consistency_level: u8,
}