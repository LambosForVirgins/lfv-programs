use anchor_lang::AnchorDeserialize;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::log::sol_log_compute_units;
use anchor_spl::token::{self};
use solana_program::account_info::AccountInfo;

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
        // Output logs
        sol_log_compute_units();
        Ok(())
    }

    pub fn migrate(ctx: Context<MigrateAccounts>) -> Result<()> {
        let new_subscription:SubscriptionAccountV2;
        let sub = &mut ctx.accounts.subscription;

        {
            let data =  sub.data.borrow();
            msg!("Migrating subscription account to version 2");
            let mut data_slice: &[u8] = &data;
            // Deserialize the subscription account (skip the 8-byte discriminator)
            let old_data = SubscriptionAccount::try_deserialize_unchecked(&mut data_slice).map_err(|_| error!(MemberError::AccountUnserializable))?;
            msg!("Old data loaded");
            // Convert the old slots to a fixed array of 4 slots
            let mut new_slots: [Slot; 4] = [Slot::default(); 4];
            msg!("New slots created");
            // If the old vector has fewer than 4 slots, the remaining are set to default
            for (idx, slot) in old_data.slots.iter().enumerate().take(4) {
                // TODO: Map the transaction slot type into the correct slot week and values
                new_slots[idx] = Slot::V1 {
                    amount: 0,
                    time_processed: 0,
                    time_matured: 0,
                    rewards_accrued: 0,
                };
                msg!("Slot {} migrated", idx);
            }
            // Create an empty queue
            let new_queue: Vec<Transaction> = Vec::new();
            msg!("New queue created");
            // Create the new subscription account
            new_subscription = SubscriptionAccountV2 {
                version: SubscriptionAccountV2::LATEST_VERSION,
                tier: old_data.tier,
                status: old_data.status,
                total_amount: old_data.total_amount,
                time_created: old_data.time_created,
                time_processed: old_data.time_rewarded,
                slots: new_slots,
                queue: new_queue
            };
            msg!("New subscription account defined");
        }

        {
            let mut data_mut = sub.data.borrow_mut();
            data_mut.fill(0);
            // Overwrite the existing account data with the new data.
            new_subscription.try_serialize(&mut *data_mut).map_err(|_| error!(MemberError::AccountUnserializable))?;
            msg!("Overwrote old subscription account");
        }
        // Output logs
        sol_log_compute_units();
        Ok(())
    }

    pub fn deposit(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        let time_now = Clock::get()?.unix_timestamp;
        let source = &mut ctx.accounts.source_token_account;
        // Validate time before reassigning to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Validate the amount in valid range
        require!(amount > 0, TransferError::InvalidAmount);
        // Check the source has enough tokens to deposit
        require!(amount <= source.amount, TransferError::InsufficientBalance);
        // Execute transfer instruction
        token::transfer(ctx.accounts.initialize_deposit_context(), amount)?; // TODO: Handle result
        // Update subscription account with deposit and reallocate account memory
        ctx.accounts.subscription.deposit_bond(amount, time_now as u64);
        // Output logs
        sol_log_compute_units();
        Ok(())
    }

    pub fn claim(ctx: Context<ClaimAccounts>) -> Result<()> {
        let time_now = Clock::get()?.unix_timestamp;
        let subscription = &mut ctx.accounts.subscription;
        // Validate time before reassigning to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Calculate the number of rewards available
        let rewards = subscription.process_rewards(time_now as u64).unwrap();
        // Validate positive number of rewards
        require!(rewards > 0, RewardError::ZeroRewardsAvailable);
        // Derive the program signature
        let seeds = &[REWARDS_SEED_PREFIX, &[ctx.bumps.mint]];
        let signer = &[&seeds[..]];
        let mint_context = ctx.accounts.initialize_mint_context();
        // Mint reward tokens to the member's associated token account
        token::mint_to(
            mint_context.with_signer(signer),
            rewards_to_lamports(rewards),
        )?;

        sol_log_compute_units();
        Ok(())
    }

    pub fn release(ctx: Context<TransferAccounts>, amount: u64) -> Result<()> {
        let time_now = Clock::get()?.unix_timestamp;
        let vault = &mut ctx.accounts.vault_token_account;
        let subscription = &mut ctx.accounts.subscription;
        // Validate time before reassigning to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Check vault token account has sufficient balance
        require!(amount <= vault.amount, TransferError::InvalidBalance);
        // Update pool attributes
        let released = subscription.unlock(amount, time_now as u64).unwrap();
        // Resize allocated space for subscription account
        ctx.accounts.subscription.realloc(
            &ctx.accounts.subscription.to_account_info(),
            &ctx.accounts.signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        )?;
        
        sol_log_compute_units();
        Ok(())
    }

    pub fn withdraw(ctx: Context<TransferAccounts>) -> Result<()> {
        let time_now = Clock::get()?.unix_timestamp;
        let subscription= &mut ctx.accounts.subscription;
        // Validate time before reassigning to u64
        require!(time_now > 0, HostError::InvalidTimestamp);
        // Request the transfer amount
        let amount = subscription.process_withdrawals(time_now as u64)?;
        // Validate the amount is within a valid range
        require!(amount > 0 && amount <= ctx.accounts.vault_token_account.amount, TransferError::InvalidAmount);
        // Derive the program signature
        let bump = ctx.bumps.subscription;
        let signer_key = ctx.accounts.signer.key();
        let seeds = [
            SUBSCRIPTION_SEED_PREFIX.as_ref(),
            signer_key.as_ref(),
            &[bump],
        ];
        let signature = &[&seeds[..]];
        // Initialize the transfer context
        let transfer_context = ctx.accounts.initialize_withdraw_context();
        // Execute transfer instruction
        anchor_spl::token::transfer(transfer_context.with_signer(signature), amount)?;
        // Resize allocated space for subscription account
        ctx.accounts.subscription.realloc(
            &ctx.accounts.subscription.to_account_info(),
            &ctx.accounts.signer.to_account_info(),
            &ctx.accounts.system_program.to_account_info(),
        )?;
         // Output logs
        sol_log_compute_units();
        Ok(())
    }
}
