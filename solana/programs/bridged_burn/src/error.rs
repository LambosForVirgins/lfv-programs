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