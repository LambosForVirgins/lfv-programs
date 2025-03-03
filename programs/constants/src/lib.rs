use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log_compute_units;
use anchor_spl::{
    metadata::{create_metadata_accounts_v3, mpl_token_metadata::types::DataV2},
    token::{self, TokenAccount},
};

use constants::*;
use errors::*;
use instructions::*;
use state::*;
use utils::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("LFV1t2uUvpEZuhduXTepyimVJ35ZANUThNPH8yp1w7o");

#[program]
mod reward_program {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        // Validate time before reassigning to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Update timestamps
        ctx.accounts.subscription.time_created = time_now as u64;
        ctx.accounts.subscription.time_rewarded = time_now as u64;
        // Output logs
        sol_log_compute_units();
        Ok(())
    }

    pub fn initialize_mint(
        ctx: Context<InitializeRewards>,
        params: InitializeRewardsParams,
    ) -> Result<()> {
        let seeds = &[REWARDS_SEED_PREFIX, &[ctx.bumps.mint]];
        let signer = &[&seeds[..]];
        let metadata_ctx = ctx.accounts.initialize_metadata_context();
        let token_data: DataV2 = DataV2 {
            name: params.name,
            symbol: params.symbol,
            uri: params.uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(
            metadata_ctx.with_signer(signer),
            token_data,
            true,
            true,
            None,
        )?;

        sol_log_compute_units();
        Ok(())
    }

    pub fn deposit(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        let source: &mut Account<TokenAccount> = &mut ctx.accounts.source_token_account;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Validate the amount in valid range
        require!(amount > 0, TransferError::InvalidAmount);
        // Check the source has enough tokens to deposit
        require!(amount <= source.amount, TransferError::InsufficientBalance);
        // Validate pending slots
        require!(
            ctx.accounts.subscription.slots.len() < MAX_PENDING_SLOTS,
            LockingError::MaxSlotsExceeded
        );
        // Execute transfer instruction
        token::transfer(ctx.accounts.initialize_deposit_context(), amount)?;
        // Update subscription account with deposit and reallocate account memory
        ctx.accounts.subscription.lock(amount, time_now as u64)?;

        ctx.accounts.subscription.realloc(
            &ctx.accounts.subscription.to_account_info(),
            &ctx.accounts.signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        );

        sol_log_compute_units();
        Ok(())
    }

    pub fn claim(ctx: Context<ClaimAccounts>) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        let subscription: &mut Account<SubscriptionAccount> = &mut ctx.accounts.subscription;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);

        let rewards = subscription.claim(time_now as u64).unwrap();

        if rewards > 0 {
            let seeds = &[REWARDS_SEED_PREFIX, &[ctx.bumps.mint]];
            let signer = &[&seeds[..]];
            let mint_context = ctx.accounts.initialize_mint_context();
            // Mint reward tokens to the member's associated token account
            token::mint_to(
                mint_context.with_signer(signer),
                rewards_to_lamports(rewards),
            )?;
        }

        msg!("Rewarded {} entries", rewards);
        sol_log_compute_units();
        Ok(())
    }

    pub fn release(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        let vault: &mut Account<TokenAccount> = &mut ctx.accounts.vault_token_account;
        let subscription: &mut Account<SubscriptionAccount> = &mut ctx.accounts.subscription;
        // Validate time before reassigning to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Check vault token account has sufficient balance
        require!(amount <= vault.amount, TransferError::InvalidBalance);
        // Update pool attributes
        let released = subscription.unlock(amount, time_now as u64).unwrap();
        // Resize allocated space for member account
        ctx.accounts.subscription.realloc(
            &ctx.accounts.subscription.to_account_info(),
            &ctx.accounts.signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        )?;

        msg!("Released {} tokens", released);
        sol_log_compute_units();
        Ok(())
    }

    pub fn withdraw(ctx: Context<TransferAccounts>) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        let subscription: &mut Account<SubscriptionAccount> = &mut ctx.accounts.subscription;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Request the transfer amount
        let amount = subscription.on_withdraw(time_now as u64)?;
        // Vlaidate the transfer amount
        if amount > 0 {
            let bump = ctx.bumps.subscription;
            let signer_key = ctx.accounts.signer.key();
            // Derive program signature
            let seeds = [
                SUBSCRIPTION_SEED_PREFIX.as_ref(),
                signer_key.as_ref(),
                &[bump],
            ];
            // Create signature with seeds
            let signature = &[&seeds[..]];
            // Initialize the transfer context
            let transfer_context = ctx.accounts.initialize_withdraw_context();
            // Execute transfer instruction
            anchor_spl::token::transfer(transfer_context.with_signer(signature), amount)?;
        }

        sol_log_compute_units();
        Ok(())
    }
}
