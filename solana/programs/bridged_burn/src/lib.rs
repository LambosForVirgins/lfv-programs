use anchor_lang::prelude::*;
use anchor_spl::token::{Burn, CloseAccount};
use solana_program::clock::Clock;
use borsh::{BorshDeserialize, BorshSerialize};
use error::*;
use state::*;
use events::*;
use instructions::*;

mod error;
mod state;
mod events;
mod instructions;

declare_id!("4rGdLkQDuZcJhCM85wwcpcyM7t5GxtpjAapV2LR6buiK");

/// Wormhole Chain ID for Sui Network
pub const SUI_CHAIN_ID: u16 = 21;

/// Message type identifier for burn confirmation
pub const BURN_CONFIRMATION_MESSAGE_TYPE: u8 = 1;

/// Wormhole Core Bridge program ID (mainnet)
pub const WORMHOLE_PROGRAM_ID: &str = "3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5";

/// Config account seed for PDA derivation
pub const CONFIG_SEED: &[u8] = b"config";

/// Proposal account seed for PDA derivation
pub const PROPOSAL_SEED: &[u8] = b"proposal";

#[cfg(test)]
mod tests {
    mod bridge_tests;
    mod integration_tests;
    mod governance_tests;
    // mod refund_tests;
}

#[program]
pub mod bridged_burn {
    use super::*;

    /// Initialize the program configuration
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        owner: Pubkey,
        vault: Pubkey,
        wormhole_program: Pubkey,
        multisig_threshold: u8,
        multisig_signers: Vec<Pubkey>,
        burn_fee: u64,
    ) -> Result<()> {
        // Validate parameters
        require!(
            multisig_threshold > 0 && multisig_threshold <= multisig_signers.len() as u8,
            GovernanceErrorCode::InvalidMultisigThreshold
        );
        require!(
            multisig_signers.len() <= Config::MAX_MULTISIG_SIGNERS,
            GovernanceErrorCode::TooManyMultisigSigners
        );
        
        // Check for duplicate signers
        for (i, signer) in multisig_signers.iter().enumerate() {
            for (j, other_signer) in multisig_signers.iter().enumerate() {
                if i != j && signer == other_signer {
                    return Err(GovernanceErrorCode::DuplicateMultisigSigner.into());
                }
            }
        }

        let config = &mut ctx.accounts.config;
        config.owner = owner;
        config.multisig_threshold = multisig_threshold;
        config.multisig_signers = multisig_signers.clone();
        config.vault = vault;
        config.wormhole_program = wormhole_program;
        config.burn_fee = burn_fee;
        config.is_paused = false;
        config.supported_mints = vec![];
        config.wormhole_consistency_level = 1;
        config.config_version = 1;
        config.last_updated = Clock::get()?.unix_timestamp;
        config.reserved = [0; 256];

        emit!(ConfigInitializedEvent {
            owner,
            vault,
            multisig_threshold,
            multisig_signers,
        });

        Ok(())
    }

    /// Create a proposal for configuration changes
    pub fn create_config_proposal(
        ctx: Context<CreateConfigProposal>,
        proposal_id: u64,
        proposed_config: Config,
        expiration_duration: i64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let proposer = &ctx.accounts.proposer;
        
        // Verify proposer is an authorized multisig signer
        require!(
            config.is_multisig_signer(&proposer.key()),
            GovernanceErrorCode::Unauthorized
        );
        
        // Validate proposed config
        require!(
            proposed_config.multisig_threshold > 0 && 
            proposed_config.multisig_threshold <= proposed_config.multisig_signers.len() as u8,
            GovernanceErrorCode::InvalidMultisigThreshold
        );
        require!(
            proposed_config.multisig_signers.len() <= Config::MAX_MULTISIG_SIGNERS,
            GovernanceErrorCode::TooManyMultisigSigners
        );
        require!(
            proposed_config.supported_mints.len() <= Config::MAX_SUPPORTED_MINTS,
            GovernanceErrorCode::TooManySupportedMints
        );

        let proposal = &mut ctx.accounts.proposal;
        proposal.proposal_id = proposal_id;
        proposal.proposed_config = proposed_config.clone();
        proposal.approved_signers = vec![proposer.key()];
        proposal.approval_count = 1;
        proposal.created_at = Clock::get()?.unix_timestamp;
        proposal.expires_at = Clock::get()?.unix_timestamp + expiration_duration;
        proposal.executed = false;
        proposal.reserved = [0; 64];

        emit!(ConfigProposalCreatedEvent {
            proposal_id,
            proposer: proposer.key(),
            expires_at: proposal.expires_at,
        });

        Ok(())
    }

    /// Approve a configuration proposal
    pub fn approve_config_proposal(
        ctx: Context<ApproveConfigProposal>,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let proposal = &mut ctx.accounts.proposal;
        let approver = &ctx.accounts.approver;
        
        // Verify approver is an authorized multisig signer
        require!(
            config.is_multisig_signer(&approver.key()),
            GovernanceErrorCode::Unauthorized
        );
        
        // Check proposal hasn't expired
        require!(
            !proposal.is_expired(),
            GovernanceErrorCode::ProposalExpired
        );
        
        // Check proposal hasn't been executed
        require!(
            !proposal.executed,
            GovernanceErrorCode::ProposalAlreadyExecuted
        );
        
        // Check signer hasn't already approved
        require!(
            !proposal.has_signer_approved(&approver.key()),
            GovernanceErrorCode::SignerAlreadyApproved
        );
        
        // Add approval
        proposal.approved_signers.push(approver.key());
        proposal.approval_count += 1;

        emit!(ConfigProposalApprovedEvent {
            proposal_id: proposal.proposal_id,
            approver: approver.key(),
            approval_count: proposal.approval_count,
        });

        Ok(())
    }

    /// Execute a configuration proposal
    pub fn execute_config_proposal(
        ctx: Context<ExecuteConfigProposal>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let proposal = &mut ctx.accounts.proposal;
        let executor = &ctx.accounts.executor;
        
        // Verify executor is an authorized multisig signer
        require!(
            config.is_multisig_signer(&executor.key()),
            GovernanceErrorCode::Unauthorized
        );
        
        // Check proposal hasn't expired
        require!(
            !proposal.is_expired(),
            GovernanceErrorCode::ProposalExpired
        );
        
        // Check proposal hasn't been executed
        require!(
            !proposal.executed,
            GovernanceErrorCode::ProposalAlreadyExecuted
        );
        
        // Check proposal has enough approvals
        require!(
            proposal.has_enough_approvals(config.multisig_threshold),
            GovernanceErrorCode::InsufficientMultisigApprovals
        );
        
        // Execute the proposal by updating config
        let proposed_config = proposal.proposed_config.clone();
        config.owner = proposed_config.owner;
        config.multisig_threshold = proposed_config.multisig_threshold;
        config.multisig_signers = proposed_config.multisig_signers;
        config.vault = proposed_config.vault;
        config.wormhole_program = proposed_config.wormhole_program;
        config.burn_fee = proposed_config.burn_fee;
        config.is_paused = proposed_config.is_paused;
        config.supported_mints = proposed_config.supported_mints;
        config.wormhole_consistency_level = proposed_config.wormhole_consistency_level;
        config.config_version = proposed_config.config_version;
        config.last_updated = Clock::get()?.unix_timestamp;
        
        // Mark proposal as executed
        proposal.executed = true;

        emit!(ConfigProposalExecutedEvent {
            proposal_id: proposal.proposal_id,
            executor: executor.key(),
        });

        Ok(())
    }

    /// Emergency pause (can be called by owner without multisig)
    pub fn emergency_pause(ctx: Context<EmergencyPause>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        // Only owner can call emergency pause
        require!(
            ctx.accounts.owner.key() == config.owner,
            GovernanceErrorCode::Unauthorized
        );
        
        config.is_paused = true;
        config.last_updated = Clock::get()?.unix_timestamp;

        emit!(EmergencyPauseEvent {
            paused_by: ctx.accounts.owner.key(),
        });

        Ok(())
    }

    /// Burns tokens and closes the token account,
    /// then sends a confirmation message to Sui via Wormhole
    pub fn burn_and_close(
        ctx: Context<BurnAndClose>,
        sui_address: [u8; 32],
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        
        // Check if program is paused
        config.check_not_paused()?;
        
        // Check if mint is supported
        require!(
            config.is_mint_supported(&ctx.accounts.mint.key()),
            GovernanceErrorCode::UnsupportedMint
        );
        
        // Validate token amount
        let burn_amount = ctx.accounts.token_account.amount;
        require!(burn_amount > 0, BridgeErrorCode::NothingToBurn);
        
        // Check if vault matches config
        require!(
            ctx.accounts.vault.key() == config.vault,
            GovernanceErrorCode::InvalidVaultAddress
        );

        // Burn all tokens
        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        );
        anchor_spl::token::burn(burn_ctx, burn_amount)?;
        
        // Close token account and refund rent to vault
        let close_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.token_account.to_account_info(),
                destination: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        );
        anchor_spl::token::close_account(close_ctx)?;

        // Send confirmation message to Sui via Wormhole
        send_burn_confirmation_message(
            &ctx,
            ctx.accounts.owner.key(),
            sui_address,
            ctx.accounts.mint.key(),
            burn_amount,
        )?;

        // Emit event for local tracking
        emit!(BridgeBurnEvent {
            sui_receiver: sui_address,
            sol_sender: ctx.accounts.owner.key(),
            mint: ctx.accounts.mint.key(),
            amount: burn_amount,
        });

        Ok(())
    }
}

/// Private function to send burn confirmation message to Sui via Wormhole
fn send_burn_confirmation_message(
    _ctx: &Context<BurnAndClose>,
    solana_sender: Pubkey,
    sui_receiver: [u8; 32],
    mint: Pubkey,
    amount: u64,
) -> Result<()> {
    let timestamp = Clock::get()?.unix_timestamp as u64;
    
    // Create structured payload
    let payload = BurnConfirmationPayload {
        message_type: BURN_CONFIRMATION_MESSAGE_TYPE,
        sui_receiver,
        solana_sender: solana_sender.to_bytes(),
        mint: mint.to_bytes(),
        amount,
        timestamp,
    };

    // Serialize payload using Borsh
    let serialized_payload = payload.try_to_vec()
        .map_err(|_| BridgeErrorCode::PayloadSerializationFailed)?;

    // Validate payload size (should be 113 bytes: 1 + 32 + 32 + 32 + 8 + 8)
    require!(serialized_payload.len() == 113, BridgeErrorCode::InvalidPayloadSize);

    // For now, we'll emit a custom event with the Wormhole payload
    // In production, this would use proper Wormhole CPI
    emit!(WormholeMessageEvent {
        target_chain: SUI_CHAIN_ID,
        payload: serialized_payload.clone(),
        consistency_level: 1,
    });

    msg!("Burn confirmation message prepared for Wormhole bridge");
    msg!("Target chain: {} (Sui)", SUI_CHAIN_ID);
    msg!("Solana sender: {}", solana_sender);
    msg!("Sui receiver: {:?}", sui_receiver);
    msg!("Amount: {}", amount);
    msg!("Timestamp: {}", timestamp);
    msg!("Payload size: {} bytes", serialized_payload.len());

    Ok(())
}