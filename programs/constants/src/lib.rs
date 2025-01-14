use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    log::sol_log_compute_units, program::invoke, program::invoke_signed, system_instruction::*,
};
use anchor_spl::{
    associated_token::get_associated_token_address,
    token::{TokenAccount, Transfer},
};

use constants::*;
use errors::*;
use instructions::*;
use state::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("9JuJwm7kUPkCzHikuvMQozfrfbdt5nJZ1Wg9FPcQ4HeX");

// TODO: Convert total amounts to struct with sub components
// TODO: Flag tokens for unlocking and subtract from matured
// TODO: Prevent rewards for unlocked tokens
// TODO: Prevent withdrawals to only unlocked tokens
// TODO: Consolidate slots
// TODO: Probably need to swap timestamps from recording past into indicating future times, for the frontend

#[program]
mod subscription {
    use super::*;

    pub fn initialize(ctx: Context<InitializeAccounts>) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Update timestamps
        ctx.accounts.member_account.time_created = time_now as u64;
        ctx.accounts.member_account.time_rewarded = time_now as u64;
        // Output logs
        sol_log_compute_units();
        Ok(())
    }

    pub fn exclude(ctx: Context<ExcludeAccounts>) -> Result<()> {
        let member: &mut Account<MemberAccount> = &mut ctx.accounts.member_account;
        member.update_status(AccountStatus::Excluded);
        msg!("Member self excluded");
        sol_log_compute_units();
        Ok(())
    }

    pub fn deposit(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        // Ensure the correct token mint was used
        require_keys_eq!(
            ctx.accounts.mint.key(),
            MINT_KEY,
            TransferError::InvalidMint
        );

        let time_now: i64 = Clock::get()?.unix_timestamp;
        let source: &mut Account<TokenAccount> = &mut ctx.accounts.source_token_account;
        let member: &mut Account<MemberAccount> = &mut ctx.accounts.member_account;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Check the source has enough tokens to deposit
        require!(amount <= source.amount, TransferError::InsufficientBalance);
        // Construct transfer instruction
        let transfer_instruction: Transfer = Transfer {
            from: ctx.accounts.source_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };
        // Initialize the transfer context
        let context: CpiContext<Transfer> = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
        );
        // Execute transfer instruction
        anchor_spl::token::transfer(context, amount)?;
        // Update member account with deposit and reallocate account memory
        let rewards: u64 = member.on_deposit(amount, time_now as u64).unwrap();
        member.grant_rewards(rewards);

        ctx.accounts.member_account.realloc(
            &ctx.accounts.member_account.to_account_info(),
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
        let member: &mut Account<MemberAccount> = &mut ctx.accounts.member_account;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);

        let rewards = member.on_claim(time_now as u64).unwrap();
        member.grant_rewards(rewards);

        msg!("Rewarded {} entries", rewards);
        sol_log_compute_units();
        Ok(())
    }

    pub fn withdraw(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        let time_now: i64 = Clock::get()?.unix_timestamp;
        let signer_key: Pubkey = ctx.accounts.signer.key();
        let mint_key: Pubkey = ctx.accounts.mint.key();
        let vault: &mut Account<TokenAccount> = &mut ctx.accounts.source_token_account;
        let member: &mut Account<MemberAccount> = &mut ctx.accounts.member_account;
        // Validate time before reassinging to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Ensure the correct token mint was used
        require_keys_eq!(mint_key, MINT_KEY, TransferError::InvalidMint);
        // Check vault token account has sufficient balance
        require!(amount <= vault.amount, TransferError::InvalidBalance);
        // Update or remove locked slots for amount
        // Subtract amount from the total locked amount
        // Construct transfer instruction
        let transfer_instruction: Transfer = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.source_token_account.to_account_info(),
            authority: member.to_account_info(),
        };
        // Derive program signature
        // TODO: Replace this with impl function
        let bump: u8 = ctx.bumps.member_account;
        let seeds: &[&[u8]; 3] = &[
            MemberAccount::SEED_PREFIX.as_ref(),
            signer_key.as_ref(),
            &[bump],
        ];
        let signature: &[&[&[u8]]; 1] = &[&seeds[..]];
        // Initialize the transfer context
        let context: CpiContext<Transfer> = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            transfer_instruction,
            signature,
        );
        // Execute transfer instruction
        anchor_spl::token::transfer(context, amount)?;
        // Update pool attributes
        let rewards = member.on_withdraw(amount, time_now as u64).unwrap();
        member.grant_rewards(rewards);
        // Update stake system balance
        // stake_system.on_deposit(amount);
        sol_log_compute_units();
        Ok(())
    }
}
