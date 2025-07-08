#[cfg(test)]
mod bridge_tests {
    use anchor_lang::prelude::*;
    use crate::{
        BurnConfirmationPayload, 
        BridgeBurnEvent, 
        WormholeMessageEvent,
        BURN_CONFIRMATION_MESSAGE_TYPE,
        SUI_CHAIN_ID,
        WORMHOLE_PROGRAM_ID
    };
    use std::str::FromStr;

    #[test]
    fn test_burn_confirmation_payload_serialization() {
        let solana_sender = Pubkey::from_str("11111111111111111111111111111112").unwrap();
        let sui_receiver = [1u8; 32];
        let mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
        let amount = 1000000u64; // 1 SOL in lamports
        let timestamp = 1640995200u64; // Example timestamp

        let payload = BurnConfirmationPayload {
            message_type: BURN_CONFIRMATION_MESSAGE_TYPE,
            sui_receiver,
            solana_sender: solana_sender.to_bytes(),
            mint: mint.to_bytes(),
            amount,
            timestamp,
        };

        // Test serialization
        let serialized = payload.try_to_vec().unwrap();
        
        // Verify payload size is exactly 113 bytes
        assert_eq!(serialized.len(), 113);
        
        // Test deserialization
        let deserialized = BurnConfirmationPayload::try_from_slice(&serialized).unwrap();
        assert_eq!(payload, deserialized);
        
        // Verify individual fields
        assert_eq!(deserialized.message_type, BURN_CONFIRMATION_MESSAGE_TYPE);
        assert_eq!(deserialized.sui_receiver, sui_receiver);
        assert_eq!(deserialized.solana_sender, solana_sender.to_bytes());
        assert_eq!(deserialized.mint, mint.to_bytes());
        assert_eq!(deserialized.amount, amount);
        assert_eq!(deserialized.timestamp, timestamp);
    }

    #[test]
    fn test_burn_confirmation_payload_structure() {
        let payload = BurnConfirmationPayload {
            message_type: BURN_CONFIRMATION_MESSAGE_TYPE,
            sui_receiver: [0u8; 32],
            solana_sender: [0u8; 32],
            mint: [0u8; 32],
            amount: 0,
            timestamp: 0,
        };

        let serialized = payload.try_to_vec().unwrap();
        
        // Verify the structure:
        // 1 byte message_type + 32 bytes sui_receiver + 32 bytes solana_sender 
        // + 32 bytes mint + 8 bytes amount + 8 bytes timestamp = 113 bytes
        assert_eq!(serialized.len(), 113);
        
        // Verify message type is first byte
        assert_eq!(serialized[0], BURN_CONFIRMATION_MESSAGE_TYPE);
    }

    #[test]
    fn test_different_amounts_and_addresses() {
        let test_cases = vec![
            (1u64, "Amount: 1"),
            (1000000u64, "Amount: 1 SOL"),
            (u64::MAX, "Amount: MAX"),
        ];

        for (amount, description) in test_cases {
            let solana_sender = Pubkey::new_unique();
            let sui_receiver = [42u8; 32];
            let mint = Pubkey::new_unique();
            let timestamp = 1640995200u64;

            let payload = BurnConfirmationPayload {
                message_type: BURN_CONFIRMATION_MESSAGE_TYPE,
                sui_receiver,
                solana_sender: solana_sender.to_bytes(),
                mint: mint.to_bytes(),
                amount,
                timestamp,
            };

            let serialized = payload.try_to_vec().unwrap();
            assert_eq!(serialized.len(), 113, "Failed for {}", description);
            
            let deserialized = BurnConfirmationPayload::try_from_slice(&serialized).unwrap();
            assert_eq!(payload.amount, deserialized.amount, "Amount mismatch for {}", description);
        }
    }

    #[test]
    fn test_sui_address_formats() {
        let test_addresses = vec![
            [0u8; 32],                    // All zeros
            [255u8; 32],                  // All ones
            [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
             0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
             0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18,
             0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20], // Sequential bytes
        ];

        for (i, sui_address) in test_addresses.iter().enumerate() {
            let payload = BurnConfirmationPayload {
                message_type: BURN_CONFIRMATION_MESSAGE_TYPE,
                sui_receiver: *sui_address,
                solana_sender: Pubkey::new_unique().to_bytes(),
                mint: Pubkey::new_unique().to_bytes(),
                amount: 1000000,
                timestamp: 1640995200,
            };

            let serialized = payload.try_to_vec().unwrap();
            assert_eq!(serialized.len(), 113, "Failed for address case {}", i);
            
            let deserialized = BurnConfirmationPayload::try_from_slice(&serialized).unwrap();
            assert_eq!(payload.sui_receiver, deserialized.sui_receiver, "Address mismatch for case {}", i);
        }
    }

    #[test]
    fn test_constants() {
        assert_eq!(SUI_CHAIN_ID, 21);
        assert_eq!(BURN_CONFIRMATION_MESSAGE_TYPE, 1);
        assert_eq!(WORMHOLE_PROGRAM_ID, "3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5");
    }

    #[test]
    fn test_payload_round_trip_with_real_addresses() {
        // Use real Solana addresses
        let solana_sender = Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(); // USDC mint
        let mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(); // SOL mint
        
        // Example Sui address (32 bytes)
        let sui_receiver = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        ];

        let payload = BurnConfirmationPayload {
            message_type: BURN_CONFIRMATION_MESSAGE_TYPE,
            sui_receiver,
            solana_sender: solana_sender.to_bytes(),
            mint: mint.to_bytes(),
            amount: 1_000_000_000, // 1 billion lamports
            timestamp: 1640995200,
        };

        let serialized = payload.try_to_vec().unwrap();
        let deserialized = BurnConfirmationPayload::try_from_slice(&serialized).unwrap();
        
        assert_eq!(payload, deserialized);
        assert_eq!(serialized.len(), 113);
    }
}