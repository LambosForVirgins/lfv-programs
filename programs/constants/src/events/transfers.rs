use anchor_lan::prelude::*;

#[event]
pub struct DepositEvent {
    pub sender: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WithdrawEvent {
    pub sender: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ReleaseEvent {
    pub sender: Pubkey,
    pub amount: u64,
}

#[event]
pub struct ClaimEvent {
    pub sender: Pubkey,
    pub amount: u64,
}
