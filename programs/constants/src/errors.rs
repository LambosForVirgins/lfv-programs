use anchor_lang::prelude::*;

#[error_code]
pub enum TransferError {
    #[msg("Invalid Member Pool")]
    InvalidMemberPool,
    #[msg("Unauthorised Member Pool")]
    UnauthorisedMemberPool,
    #[msg("Invalid token balance")]
    InvalidBalance,
    #[msg("Insufficient token balance")]
    InsufficientBalance,
    #[msg("No Matching Entry to Withdraw")]
    InvalidEntryAddress,
    #[msg("Entry Owner Key Mismatch")]
    InvalidOwner,
    #[msg("Withdrawal period not reached")]
    InvalidWithdrawTime,
    #[msg("Withdraw Entry Index OverFlow")]
    IndexOverflow,
    #[msg("Insufficient Lamports")]
    LackLamports,
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
