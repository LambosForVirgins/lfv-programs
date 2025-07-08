use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::error::GovernanceErrorCode;

/// Configuration account for the bridged burn program
#[account]
pub struct Config {
    /// Program owner/authority that can update configuration
    pub owner: Pubkey,
    
    /// Multisig threshold for configuration changes
    pub multisig_threshold: u8,
    
    /// List of authorized multisig signers (max 10)
    pub multisig_signers: Vec<Pubkey>,
    
    /// Vault address where lamports are collected
    pub vault: Pubkey,
    
    /// Wormhole program ID
    pub wormhole_program: Pubkey,
    
    /// Fee for burning tokens (in lamports)
    pub burn_fee: u64,
    
    /// Emergency pause flag
    pub is_paused: bool,
    
    /// Supported token mints (empty means all mints are supported)
    pub supported_mints: Vec<Pubkey>,
    
    /// Wormhole consistency level
    pub wormhole_consistency_level: u8,
    
    /// Configuration version for upgrades
    pub config_version: u8,
    
    /// Timestamp when config was last updated
    pub last_updated: i64,
    
    /// Reserved space for future upgrades
    pub reserved: [u8; 256],
}

impl Config {
    /// Size of the config account in bytes
    pub const SIZE: usize = 8 + // discriminator
        32 + // owner
        1 + // multisig_threshold
        (4 + 32 * 10) + // multisig_signers (max 10)
        32 + // vault
        32 + // wormhole_program
        8 + // burn_fee
        1 + // is_paused
        (4 + 32 * 50) + // supported_mints (max 50)
        1 + // wormhole_consistency_level
        1 + // config_version
        8 + // last_updated
        256; // reserved
    
    /// Maximum number of multisig signers
    pub const MAX_MULTISIG_SIGNERS: usize = 10;
    
    /// Maximum number of supported mints
    pub const MAX_SUPPORTED_MINTS: usize = 50;
    
    /// Check if a pubkey is an authorized multisig signer
    pub fn is_multisig_signer(&self, pubkey: &Pubkey) -> bool {
        self.multisig_signers.contains(pubkey)
    }
    
    /// Check if a mint is supported (empty list means all mints are supported)
    pub fn is_mint_supported(&self, mint: &Pubkey) -> bool {
        self.supported_mints.is_empty() || self.supported_mints.contains(mint)
    }
    
    /// Check if the program is paused
    pub fn check_not_paused(&self) -> Result<()> {
        require!(!self.is_paused, GovernanceErrorCode::ProgramPaused);
        Ok(())
    }
}

/// Proposal for configuration changes (for multisig governance)
#[account]
pub struct ConfigProposal {
    /// Unique identifier for the proposal
    pub proposal_id: u64,
    
    /// The proposed configuration changes
    pub proposed_config: Config,
    
    /// Signers who have approved this proposal
    pub approved_signers: Vec<Pubkey>,
    
    /// Number of approvals received
    pub approval_count: u8,
    
    /// Proposal creation timestamp
    pub created_at: i64,
    
    /// Proposal expiration timestamp
    pub expires_at: i64,
    
    /// Whether the proposal has been executed
    pub executed: bool,
    
    /// Reserved space for future upgrades
    pub reserved: [u8; 64],
}

impl ConfigProposal {
    /// Size of the proposal account in bytes
    pub const SIZE: usize = 8 + // discriminator
        8 + // proposal_id
        Config::SIZE + // proposed_config
        (4 + 32 * 10) + // approved_signers (max 10)
        1 + // approval_count
        8 + // created_at
        8 + // expires_at
        1 + // executed
        64; // reserved
    
    /// Check if proposal has expired
    pub fn is_expired(&self) -> bool {
        Clock::get().unwrap().unix_timestamp > self.expires_at
    }
    
    /// Check if signer has already approved
    pub fn has_signer_approved(&self, signer: &Pubkey) -> bool {
        self.approved_signers.contains(signer)
    }
    
    /// Check if proposal has enough approvals
    pub fn has_enough_approvals(&self, threshold: u8) -> bool {
        self.approval_count >= threshold
    }
}

/// Structured payload for burn confirmation message
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct BurnConfirmationPayload {
    /// Message type identifier (1 byte)
    pub message_type: u8,
    /// Sui receiver address (32 bytes)
    pub sui_receiver: [u8; 32],
    /// Solana sender address (32 bytes) 
    pub solana_sender: [u8; 32],
    /// Token mint address (32 bytes)
    pub mint: [u8; 32],
    /// Amount burned (8 bytes)
    pub amount: u64,
    /// Timestamp when burn occurred (8 bytes)
    pub timestamp: u64,
}

