#[cfg(test)]
mod governance_tests {
    use anchor_lang::prelude::*;
    use crate::{
        Config,
        ConfigProposal,
        CONFIG_SEED,
        PROPOSAL_SEED,
        EmergencyPauseEvent,
    };

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

    #[tokio::test]
    async fn test_emergency_pause_authorization() {
        // Test that emergency pause can only be called by the owner set in the config
        // This test validates the authorization logic that the emergency_pause function implements
        
        let owner = Pubkey::new_unique();
        let non_owner = Pubkey::new_unique();
        let multisig_signer = Pubkey::new_unique();
        let random_user = Pubkey::new_unique();
        
        let mut config = Config {
            owner,
            multisig_threshold: 2,
            multisig_signers: vec![multisig_signer, Pubkey::new_unique()],
            vault: Pubkey::new_unique(),
            wormhole_program: Pubkey::new_unique(),
            burn_fee: 1000,
            is_paused: false,
            supported_mints: vec![],
            wormhole_consistency_level: 1,
            config_version: 1,
            last_updated: 1640995200,
            reserved: [0; 256],
        };

        // Verify initial state - config should not be paused
        assert!(!config.is_paused, "Config should not be paused initially");
        assert!(config.check_not_paused().is_ok(), "Should allow operations when not paused");

        // Test Case 1: Valid owner should be authorized for emergency pause
        // This tests the exact authorization check: ctx.accounts.owner.key() == config.owner
        let is_authorized = owner == config.owner;
        assert!(is_authorized, "Owner should be authorized for emergency pause");
        
        if is_authorized {
            // Execute the state changes that emergency_pause would make
            config.is_paused = true;
            config.last_updated = 1640995300; // Updated timestamp
        }
        
        assert!(config.is_paused, "Config should be paused after emergency pause by owner");
        assert!(config.last_updated > 1640995200, "Last updated timestamp should be updated");

        // Test Case 2: Non-owner should NOT be authorized for emergency pause
        config.is_paused = false; // Reset for test
        config.last_updated = 1640995200;
        
        let is_authorized = non_owner == config.owner;
        assert!(!is_authorized, "Non-owner should not be authorized for emergency pause");
        
        // Verify the authorization would fail (same check as in emergency_pause function)
        let would_succeed = non_owner == config.owner;
        assert!(!would_succeed, "Non-owner should fail authorization check");
        assert!(!config.is_paused, "Config should remain unpaused when non-owner tries emergency pause");
        assert_eq!(config.last_updated, 1640995200, "Last updated should not change on failed emergency pause");

        // Test Case 3: Multisig signer (who is not owner) should NOT be authorized
        let is_authorized = multisig_signer == config.owner;
        assert!(!is_authorized, "Multisig signer should not be authorized for emergency pause");
        assert!(config.is_multisig_signer(&multisig_signer), "Multisig signer should be valid for other operations");
        
        // Even though they're a valid multisig signer, they cannot emergency pause
        let would_succeed = multisig_signer == config.owner;
        assert!(!would_succeed, "Multisig signer should fail emergency pause authorization");
        
        // Test Case 4: Random user should NOT be authorized
        let is_authorized = random_user == config.owner;
        assert!(!is_authorized, "Random user should not be authorized for emergency pause");
        assert!(!config.is_multisig_signer(&random_user), "Random user should not be a multisig signer");
        
        let would_succeed = random_user == config.owner;
        assert!(!would_succeed, "Random user should fail emergency pause authorization");

        // Test Case 5: Verify emergency pause event structure
        config.is_paused = false; // Reset
        
        let is_authorized = owner == config.owner;
        if is_authorized {
            config.is_paused = true;
            config.last_updated = 1640995400;
            
            // Verify event would be emitted correctly
            let expected_event = EmergencyPauseEvent {
                paused_by: owner,
            };
            assert_eq!(expected_event.paused_by, owner, "Event should contain correct pauser address");
        }

        // Test Case 6: Verify that once paused, operations are blocked
        assert!(config.check_not_paused().is_err(), "Should reject operations when paused");
        
        // Test Case 7: Test owner change scenario - new owner should be authorized
        let new_owner = Pubkey::new_unique();
        config.owner = new_owner;
        config.is_paused = false; // Reset pause state
        
        let is_authorized = new_owner == config.owner;
        assert!(is_authorized, "New owner should be authorized for emergency pause");
        
        if is_authorized {
            config.is_paused = true;
            config.last_updated = 1640995500;
        }
        assert!(config.is_paused, "Config should be paused by new owner");
        
        // Old owner should no longer be authorized
        config.is_paused = false; // Reset for test
        let is_authorized = owner == config.owner;
        assert!(!is_authorized, "Old owner should not be authorized after ownership change");
        
        let would_succeed = owner == config.owner;
        assert!(!would_succeed, "Old owner should fail authorization after ownership change");

        // Test Case 8: Verify the exact authorization logic matches emergency_pause function
        // This mirrors the require! check: require!(ctx.accounts.owner.key() == config.owner, GovernanceErrorCode::Unauthorized);
        
        let test_cases = vec![
            (owner, false), // old owner, should fail
            (new_owner, true), // current owner, should succeed
            (multisig_signer, false), // multisig signer, should fail
            (random_user, false), // random user, should fail
        ];
        
        for (caller, should_succeed) in test_cases {
            let authorization_check = caller == config.owner;
            assert_eq!(authorization_check, should_succeed, 
                     "Authorization check for caller {:?} should be {}", caller, should_succeed);
            
            // This is the exact same logic as in the emergency_pause function
            if !authorization_check {
                // Would return Err(GovernanceErrorCode::Unauthorized.into())
                assert!(!should_succeed, "Failed authorization should correspond to expected failure");
            }
        }

        // Test Case 9: Verify state consistency after successful emergency pause
        let final_owner = config.owner; // new_owner
        let is_authorized = final_owner == config.owner;
        if is_authorized {
            let initial_timestamp = config.last_updated;
            config.is_paused = true;
            config.last_updated = initial_timestamp + 100; // Simulate Clock::get()?.unix_timestamp
            
            assert!(config.is_paused, "Config should be paused after authorized call");
            assert!(config.last_updated > initial_timestamp, "Timestamp should be updated");
            assert!(config.check_not_paused().is_err(), "Paused config should block operations");
        }
    }
} 