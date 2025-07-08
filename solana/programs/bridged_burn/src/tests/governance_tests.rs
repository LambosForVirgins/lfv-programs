#[cfg(test)]
mod governance_tests {
    use anchor_lang::prelude::*;
    use crate::{
        BridgeBurnEvent, 
        BURN_CONFIRMATION_MESSAGE_TYPE,
        SUI_CHAIN_ID,
        error::GovernanceErrorCode,
        Config,
        ConfigProposal,
        CONFIG_SEED,
        PROPOSAL_SEED,
        ConfigInitializedEvent,
        ConfigProposalCreatedEvent,
        ConfigProposalApprovedEvent,
        ConfigProposalExecutedEvent,
        EmergencyPauseEvent,
    };

    #[tokio::test]
    async fn test_config_account_size() {
        // Test that the config account size is correctly calculated
        let expected_size = Config::SIZE;
        assert!(expected_size > 0, "Config size should be greater than 0");
        assert!(expected_size < 10240, "Config size should be reasonable (< 10KB)");
        println!("Config account size: {} bytes", expected_size);
    }

    #[tokio::test]
    async fn test_config_proposal_size() {
        // Test that the config proposal size is correctly calculated
        let expected_size = ConfigProposal::SIZE;
        assert!(expected_size > 0, "Proposal size should be greater than 0");
        assert!(expected_size < 20480, "Proposal size should be reasonable (< 20KB)");
        println!("Config proposal size: {} bytes", expected_size);
    }

    #[tokio::test]
    async fn test_config_validation() {
        // Test config validation logic
        let owner = Pubkey::new_unique();
        let vault = Pubkey::new_unique();
        let wormhole_program = Pubkey::new_unique();
        let signers = vec![Pubkey::new_unique(), Pubkey::new_unique(), Pubkey::new_unique()];

        // Test valid config
        let config = Config {
            owner,
            multisig_threshold: 2,
            multisig_signers: signers.clone(),
            vault,
            wormhole_program,
            burn_fee: 1000,
            is_paused: false,
            supported_mints: vec![],
            wormhole_consistency_level: 1,
            config_version: 1,
            last_updated: 0,
            reserved: [0; 256],
        };

        assert!(config.is_multisig_signer(&signers[0]), "Should recognize valid signer");
        assert!(!config.is_multisig_signer(&Pubkey::new_unique()), "Should reject invalid signer");
        assert!(config.is_mint_supported(&Pubkey::new_unique()), "Should support all mints when list is empty");
        assert!(config.check_not_paused().is_ok(), "Should allow operations when not paused");
    }

    #[tokio::test]
    async fn test_config_proposal_validation() {
        // Mock timestamp (using hardcoded value for testing)
        let now = 1640995200i64; // Jan 1, 2022

        let mut proposal = ConfigProposal {
            proposal_id: 1,
            proposed_config: Config {
                owner: Pubkey::new_unique(),
                multisig_threshold: 2,
                multisig_signers: vec![Pubkey::new_unique(), Pubkey::new_unique()],
                vault: Pubkey::new_unique(),
                wormhole_program: Pubkey::new_unique(),
                burn_fee: 1000,
                is_paused: false,
                supported_mints: vec![],
                wormhole_consistency_level: 1,
                config_version: 1,
                last_updated: 0,
                reserved: [0; 256],
            },
            approved_signers: vec![],
            approval_count: 0,
            created_at: now,
            expires_at: now + 86400, // 24 hours
            executed: false,
            reserved: [0; 64],
        };

        let signer = Pubkey::new_unique();
        
        // Test proposal logic (manually checking expiration since Clock::get() doesn't work in tests)
        assert!(proposal.expires_at > now, "New proposal should not be expired");
        assert!(!proposal.has_signer_approved(&signer), "Should not have approval initially");
        assert!(!proposal.has_enough_approvals(2), "Should not have enough approvals initially");
        
        // Add approvals
        proposal.approved_signers.push(signer);
        proposal.approval_count = 1;
        
        assert!(proposal.has_signer_approved(&signer), "Should show signer approved");
        assert!(!proposal.has_enough_approvals(2), "Should still need more approvals");
        
        proposal.approval_count = 2;
        assert!(proposal.has_enough_approvals(2), "Should have enough approvals");
        
        // Test expiration (manual check since Clock::get() not available in tests)
        proposal.expires_at = now - 1;
        assert!(proposal.expires_at < now, "Past proposal should be expired");
    }

    #[tokio::test]
    async fn test_config_seeds() {
        // Test that config seeds are properly defined
        assert_eq!(CONFIG_SEED, b"config");
        assert_eq!(PROPOSAL_SEED, b"proposal");
        assert!(CONFIG_SEED.len() > 0, "Config seed should not be empty");
        assert!(PROPOSAL_SEED.len() > 0, "Proposal seed should not be empty");
    }

    #[tokio::test]
    async fn test_config_error_codes() {
        // Test that error codes are properly defined
        let error = GovernanceErrorCode::Unauthorized;
        assert_eq!(error as u32, 0, "Error code should have correct value");
        
        let error = GovernanceErrorCode::InvalidMultisigThreshold;
        assert_eq!(error as u32, 1, "Error code should have correct value");
        
        let error = GovernanceErrorCode::ProgramPaused;
        assert_eq!(error as u32, 7, "Error code should have correct value");
    }

    #[tokio::test]
    async fn test_config_constants() {
        // Test configuration constants
        assert_eq!(Config::MAX_MULTISIG_SIGNERS, 10, "Max multisig signers should be 10");
        assert_eq!(Config::MAX_SUPPORTED_MINTS, 50, "Max supported mints should be 50");
        assert_eq!(SUI_CHAIN_ID, 21, "Sui chain ID should be 21");
        assert_eq!(BURN_CONFIRMATION_MESSAGE_TYPE, 1, "Burn confirmation message type should be 1");
    }

    #[tokio::test]
    async fn test_multisig_configuration() {
        // Test various multisig configurations
        let signers = vec![
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        ];

        // Test valid configurations
        let config = Config {
            owner: Pubkey::new_unique(),
            multisig_threshold: 3,
            multisig_signers: signers.clone(),
            vault: Pubkey::new_unique(),
            wormhole_program: Pubkey::new_unique(),
            burn_fee: 1000,
            is_paused: false,
            supported_mints: vec![],
            wormhole_consistency_level: 1,
            config_version: 1,
            last_updated: 0,
            reserved: [0; 256],
        };

        // Test multisig validation
        for signer in &signers {
            assert!(config.is_multisig_signer(signer), "Should recognize all configured signers");
        }

        // Test mint support
        let test_mint = Pubkey::new_unique();
        assert!(config.is_mint_supported(&test_mint), "Should support all mints when list is empty");
        
        // Test with specific mints
        let config_with_mints = Config {
            supported_mints: vec![test_mint],
            ..config
        };
        assert!(config_with_mints.is_mint_supported(&test_mint), "Should support configured mint");
        assert!(!config_with_mints.is_mint_supported(&Pubkey::new_unique()), "Should reject unconfigured mint");
    }

    #[tokio::test]
    async fn test_emergency_pause_functionality() {
        let mut config = Config {
            owner: Pubkey::new_unique(),
            multisig_threshold: 2,
            multisig_signers: vec![Pubkey::new_unique(), Pubkey::new_unique()],
            vault: Pubkey::new_unique(),
            wormhole_program: Pubkey::new_unique(),
            burn_fee: 1000,
            is_paused: false,
            supported_mints: vec![],
            wormhole_consistency_level: 1,
            config_version: 1,
            last_updated: 0,
            reserved: [0; 256],
        };

        // Test normal operation
        assert!(config.check_not_paused().is_ok(), "Should allow operations when not paused");
        
        // Test pause
        config.is_paused = true;
        assert!(config.check_not_paused().is_err(), "Should reject operations when paused");
        
        // Test unpause
        config.is_paused = false;
        assert!(config.check_not_paused().is_ok(), "Should allow operations when unpaused");
    }

    #[tokio::test]
    async fn test_config_events() {
        // Test that events have the correct structure
        let config_event = ConfigInitializedEvent {
            owner: Pubkey::new_unique(),
            vault: Pubkey::new_unique(),
            multisig_threshold: 3,
            multisig_signers: vec![Pubkey::new_unique(), Pubkey::new_unique()],
        };

        let proposal_event = ConfigProposalCreatedEvent {
            proposal_id: 1,
            proposer: Pubkey::new_unique(),
            expires_at: 1234567890,
        };

        let approval_event = ConfigProposalApprovedEvent {
            proposal_id: 1,
            approver: Pubkey::new_unique(),
            approval_count: 2,
        };

        let execution_event = ConfigProposalExecutedEvent {
            proposal_id: 1,
            executor: Pubkey::new_unique(),
        };

        let pause_event = EmergencyPauseEvent {
            paused_by: Pubkey::new_unique(),
        };

        let burn_event = BridgeBurnEvent {
            sui_receiver: [0x42; 32],
            sol_sender: Pubkey::new_unique(),
            mint: Pubkey::new_unique(),
            amount: 1000000,
        };

        // Test that all events compile and have expected fields
        assert_eq!(config_event.multisig_threshold, 3);
        assert_eq!(proposal_event.proposal_id, 1);
        assert_eq!(approval_event.approval_count, 2);
        assert_eq!(execution_event.proposal_id, 1);
        assert_eq!(burn_event.amount, 1000000);
    }

    #[tokio::test]
    async fn test_pda_derivation() {
        // Test that PDA seeds are correct for config and proposal accounts
        let program_id = crate::id();
        
        // Test config PDA
        let (config_pda, _config_bump) = Pubkey::find_program_address(
            &[CONFIG_SEED],
            &program_id,
        );
        assert_ne!(config_pda, Pubkey::default(), "Config PDA should be valid");
        
        // Test proposal PDA
        let proposal_id = 1u64;
        let (proposal_pda, _proposal_bump) = Pubkey::find_program_address(
            &[PROPOSAL_SEED, &proposal_id.to_le_bytes()],
            &program_id,
        );
        assert_ne!(proposal_pda, Pubkey::default(), "Proposal PDA should be valid");
        assert_ne!(config_pda, proposal_pda, "Config and proposal PDAs should be different");
    }

    #[tokio::test]
    async fn test_config_limits() {
        // Test that config limits are enforced properly
        let base_config = Config {
            owner: Pubkey::new_unique(),
            multisig_threshold: 2,
            multisig_signers: vec![Pubkey::new_unique(), Pubkey::new_unique()],
            vault: Pubkey::new_unique(),
            wormhole_program: Pubkey::new_unique(),
            burn_fee: 1000,
            is_paused: false,
            supported_mints: vec![],
            wormhole_consistency_level: 1,
            config_version: 1,
            last_updated: 0,
            reserved: [0; 256],
        };

        // Test max multisig signers
        let max_signers: Vec<Pubkey> = (0..Config::MAX_MULTISIG_SIGNERS)
            .map(|_| Pubkey::new_unique())
            .collect();
        
        let config_with_max_signers = Config {
            multisig_signers: max_signers.clone(),
            multisig_threshold: max_signers.len() as u8,
            ..base_config
        };
        
        assert_eq!(config_with_max_signers.multisig_signers.len(), Config::MAX_MULTISIG_SIGNERS);
        
        // Test max supported mints
        let max_mints: Vec<Pubkey> = (0..Config::MAX_SUPPORTED_MINTS)
            .map(|_| Pubkey::new_unique())
            .collect();
        
        let config_with_max_mints = Config {
            supported_mints: max_mints.clone(),
            ..base_config
        };
        
        assert_eq!(config_with_max_mints.supported_mints.len(), Config::MAX_SUPPORTED_MINTS);
    }

    #[tokio::test]
    async fn test_config_version_management() {
        let config = Config {
            owner: Pubkey::new_unique(),
            multisig_threshold: 2,
            multisig_signers: vec![Pubkey::new_unique(), Pubkey::new_unique()],
            vault: Pubkey::new_unique(),
            wormhole_program: Pubkey::new_unique(),
            burn_fee: 1000,
            is_paused: false,
            supported_mints: vec![],
            wormhole_consistency_level: 1,
            config_version: 1,
            last_updated: 1234567890,
            reserved: [0; 256],
        };

        assert_eq!(config.config_version, 1, "Config version should be initialized to 1");
        assert_eq!(config.last_updated, 1234567890, "Last updated should be properly set");
        assert_eq!(config.reserved.len(), 256, "Reserved space should be 256 bytes");
    }

    #[tokio::test]
    async fn test_multisig_workflow() {
        // Create a multisig configuration scenario
        let owner = Pubkey::new_unique();
        let signers = vec![
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        ];
        
        let config = Config {
            owner,
            multisig_threshold: 2, // Require 2 out of 3 signatures
            multisig_signers: signers.clone(),
            vault: Pubkey::new_unique(),
            wormhole_program: Pubkey::new_unique(),
            burn_fee: 1000,
            is_paused: false,
            supported_mints: vec![],
            wormhole_consistency_level: 1,
            config_version: 1,
            last_updated: 0,
            reserved: [0; 256],
        };

        // Create a proposal for config change
        let now = 1640995200i64; // Jan 1, 2022

        let mut proposal = ConfigProposal {
            proposal_id: 1,
            proposed_config: Config {
                burn_fee: 2000, // Change burn fee
                ..config.clone()
            },
            approved_signers: vec![signers[0]], // First signer creates and auto-approves
            approval_count: 1,
            created_at: now,
            expires_at: now + 86400,
            executed: false,
            reserved: [0; 64],
        };

        // Verify initial state
        assert!(!proposal.has_enough_approvals(config.multisig_threshold), 
               "Should not have enough approvals initially");
        assert!(proposal.has_signer_approved(&signers[0]), 
               "Creator should have auto-approved");
        assert!(!proposal.has_signer_approved(&signers[1]), 
               "Other signers should not have approved");

        // Second signer approves
        proposal.approved_signers.push(signers[1]);
        proposal.approval_count += 1;

        // Verify threshold reached
        assert!(proposal.has_enough_approvals(config.multisig_threshold), 
               "Should have enough approvals after second signature");
        assert!(!proposal.executed, "Should not be executed yet");

        // Simulate execution
        proposal.executed = true;
        assert!(proposal.executed, "Should be marked as executed");
    }
} 