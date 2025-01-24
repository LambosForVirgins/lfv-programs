use anchor_lang::prelude::*;

#[error_code]
pub enum TransferError {
    #[msg("Invalid token balance")]
    InvalidBalance,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Insufficient token balance")]
    InsufficientBalance,
    #[msg("Invalid mint")]
    InvalidMint,
}

#[error_code]
pub enum MemberError {
    #[msg("Account suspended")]
    AccountSuspended,
    #[msg("Immutable account status")]
    ImmutableAccountStatus,
}

#[error_code]
pub enum LockingError {
    #[msg("Too many slots allocated")]
    MaxSlotsExceeded,
    #[msg("Slot index out of bounds")]
    IndexOutOfBounds,
    #[msg("Forbidden account status")]
    RewardsForbidden,
    #[msg("Insufficient lamport balance")]
    InsufficientFunds,
}

#[error_code]
pub enum HostError {
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    #[msg("Invalid Pause Authority")]
    InvalidPauseAuthority,
    #[msg("System Already Paused")]
    AlreadyPaused,
    #[msg("System Not Paused")]
    NotPaused,
}
