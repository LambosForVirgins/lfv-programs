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
    #[msg("Account can't be deserialized")]
    AccountUnserializable,
    #[msg("Account suspended")]
    AccountSuspended,
    #[msg("Member account is disabled")]
    AccountDisabled,
    #[msg("Account status can't be changed")]
    AccountImmutable,
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
pub enum RewardError {
    #[msg("Invalid epoch duration: time_matured must be greater than time_processed.")]
    InvalidEpochDuration,
    #[msg("Reward calculation overflowed.")]
    RewardOverflow,
    #[msg("No rewards available.")]
    ZeroRewardsAvailable,
}

#[error_code]
pub enum HostError {
    #[msg("Invalid timestamp")]
    InvalidTimestamp,
    #[msg("No rewards available")]
    NoRewards,
    #[msg("Invalid Pause Authority")]
    InvalidPauseAuthority,
    #[msg("System Already Paused")]
    AlreadyPaused,
    #[msg("System Not Paused")]
    NotPaused,
}
