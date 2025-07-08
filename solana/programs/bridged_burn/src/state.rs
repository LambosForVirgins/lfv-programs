use anchor_lang::prelude::*;
use solana_program::clock::Clock;
use borsh::{BorshDeserialize, BorshSerialize};

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