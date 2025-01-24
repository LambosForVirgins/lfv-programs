use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    log::sol_log_compute_units, program::invoke, program::invoke_signed, system_instruction::*,
};
use anchor_spl::{
    associated_token::get_associated_token_address,
    token::{self, TokenAccount, Transfer},
};

use constants::*;
use errors::*;
use instructions::*;
use macros::*;
use state::*;
use utils::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod macros;
pub mod state;
pub mod utils;

declare_id!("5FMGC6UGHY62NsitCmbkqwn5hJ7533mGRoxogf8JU8sM");

#[program]
mod reward_program {
    use super::*;

    pub fn create_account(ctx: Context<InitializeAccounts>) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Update timestamps
        ctx.accounts.subscription.time_created = time_now as u64;
        ctx.accounts.subscription.time_rewarded = time_now as u64;
        // Output logs
        sol_log_compute_units();
        Ok(())
    }

    pub fn exclude(ctx: Context<ExcludeAccounts>) -> Result<()> {
        let subscription: &mut Account<SubscriptionAccount> = &mut ctx.accounts.subscription;
        // Validate the account status
        require_valid_status!(subscription.status);
        // Update the status
        subscription.update_status(AccountStatus::Excluded);
        msg!("Subscription self excluded");
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
        // Validate the account status
        require_valid_status!(ctx.accounts.subscription.status);
        // Execute transfer instruction
        anchor_spl::token::transfer(ctx.accounts.initialize_deposit_context(), amount)?;
        // Update subscription account with deposit and reallocate account memory
        let rewards: u64 = ctx
            .accounts
            .subscription
            .on_deposit(amount, time_now as u64)
            .unwrap();
        // subscription.grant_rewards(rewards);

        ctx.accounts.subscription.realloc(
            &ctx.accounts.subscription.to_account_info(),
            &ctx.accounts.signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        );

        msg!("Rewarded {} entries", rewards);
        sol_log_compute_units();
        Ok(())
    }

    // pub fn redeem(ctx: Context<RedeemAccounts>) -> Result<()> {
    //     // Should redeem the entries into the giveaway draw
    // }

    pub fn claim(ctx: Context<ClaimAccounts>) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        let subscription: &mut Account<SubscriptionAccount> = &mut ctx.accounts.subscription;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);

        let rewards = subscription.on_claim(time_now as u64).unwrap();
        // subscription.grant_rewards(rewards);
        // Mint reward tokens to the member's associated token account
        // token::mint_to(ctx.accounts.initialize_mint_context(), 10)?;

        msg!("Rewarded {} entries", rewards);
        sol_log_compute_units();
        Ok(())
    }

    pub fn release(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        let vault: &mut Account<TokenAccount> = &mut ctx.accounts.source_token_account;
        let subscription: &mut Account<SubscriptionAccount> = &mut ctx.accounts.subscription;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Check vault token account has sufficient balance
        require!(amount <= vault.amount, TransferError::InvalidBalance);
        // Update pool attributes
        let rewards = subscription.on_release(amount, time_now as u64).unwrap();
        // subscription.grant_rewards(rewards);

        ctx.accounts.subscription.realloc(
            &ctx.accounts.subscription.to_account_info(),
            &ctx.accounts.signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        )?;

        sol_log_compute_units();
        Ok(())
    }

    pub fn withdraw(ctx: Context<TransferAccounts>) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        let mint_key: Pubkey = ctx.accounts.mint.key();
        let subscription: &mut Account<SubscriptionAccount> = &mut ctx.accounts.subscription;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Ensure the correct token mint was used
        require_keys_eq!(mint_key, MINT_KEY, TransferError::InvalidMint);
        // Request the transfer amount
        let amount = subscription.on_withdraw(time_now as u64)?;
        let bump: u8 = ctx.bumps.subscription;
        // Construct withdraw transfer instruction
        let transfer_instruction: Transfer = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.source_token_account.to_account_info(),
            authority: ctx.accounts.subscription.to_account_info(),
        };
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
        let transfer_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signature,
        );
        // Execute transfer instruction
        anchor_spl::token::transfer(transfer_context, amount)?;

        sol_log_compute_units();
        Ok(())
    }

    pub fn cancel(ctx: Context<CancelAccounts>) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        let subscription: &mut Account<SubscriptionAccount> = &mut ctx.accounts.subscription;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);

        // Should schedule all tokens for release

        // Should claim back rent

        sol_log_compute_units();
        Ok(())
    }
}
