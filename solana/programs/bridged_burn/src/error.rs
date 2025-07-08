use anchor_lang::prelude::error_code;

#[error_code]
/// Errors relevant to this program's malfunction.
pub enum BridgeErrorCode {
    #[msg("Nothing to burn - token account is empty")]
    NothingToBurn,
    
    #[msg("Invalid payload size for Wormhole message")]
    InvalidPayloadSize,
    
    #[msg("Invalid Sui address format")]
    InvalidSuiAddress,
    
    #[msg("Wormhole message posting failed")]
    WormholePostFailed,
    
    #[msg("Failed to serialize payload")]
    PayloadSerializationFailed,
}

#[error_code]
/// Errors relevant to governance operations.
pub enum GovernanceErrorCode {
    // Config-related errors
    #[msg("Unauthorized - only config owner can perform this action")]
    Unauthorized,
    
    #[msg("Invalid multisig threshold - must be between 1 and number of signers")]
    InvalidMultisigThreshold,
    
    #[msg("Too many multisig signers - maximum is 10")]
    TooManyMultisigSigners,
    
    #[msg("Insufficient multisig approvals")]
    InsufficientMultisigApprovals,
    
    #[msg("Signer already approved this proposal")]
    SignerAlreadyApproved,
    
    #[msg("Proposal has expired")]
    ProposalExpired,
    
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    
    #[msg("Program is paused")]
    ProgramPaused,
    
    #[msg("Token mint is not supported")]
    UnsupportedMint,
    
    #[msg("Config account not found")]
    ConfigNotFound,
    
    #[msg("Invalid config version")]
    InvalidConfigVersion,
    
    #[msg("Too many supported mints - maximum is 50")]
    TooManySupportedMints,
    
    #[msg("Duplicate multisig signer")]
    DuplicateMultisigSigner,
    
    #[msg("Invalid vault address")]
    InvalidVaultAddress,
    
    #[msg("Invalid wormhole program address")]
    InvalidWormholeProgram,
    
    #[msg("Insufficient fees provided")]
    InsufficientFees,
}