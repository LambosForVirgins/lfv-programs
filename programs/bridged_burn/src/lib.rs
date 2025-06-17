use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Burn, CloseAccount};
use wormhole_sdk::core::post_message;
use solana_program::clock::Clock;

declare_id!("4rGdLkQDuZcJhCM85wwcpcyM7t5GxtpjAapV2LR6buiK");

#[program]
pub mod bridged_burn {
    use super::*;

    pub fn burn_and_close(
        ctx: Context<BurnAndClose>,
        sui_address: [u8; 32],
    ) -> Result<()> {
        // Validate token amount
        let burn_amount = ctx.accounts.token_account.amount;
        require!(burn_amount > 0, ErrorCode::NothingToBurn);
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

        // Close account, send lamports to vault
        let close_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.token_account.to_account_info(),
                destination: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        );
        anchor_spl::token::close_account(close_ctx)?;

        // Emit event
        emit!(BridgeBurnEvent {
            sui_receiver: sui_address,
            sol_sender: ctx.accounts.owner.key(),
            mint: ctx.accounts.mint.key(),
            amount: burn_amount,
        });

        // Build your Wormhole payload: here we concatenate
        // [32-byte recipient │ 8-byte amount u64 │ …]
        let mut payload = Vec::with_capacity(32 + 8);
        payload.extend_from_slice(&sui_address);
        payload.extend_from_slice(&burn_amount.to_le_bytes());

        // CPI into Wormhole Core Bridge
        // This uses the `post_message` helper from `wormhole-sdk`
        let clock = Clock::get()?.unix_timestamp as u32;
        let bridge_ctx = CpiContext::new(
            ctx.accounts.wormhole_program.to_account_info(),
            PostMessage {
                payer:            ctx.accounts.owner.to_account_info(),
                config:           ctx.accounts.wormhole_config.to_account_info(),
                clock:            ctx.accounts.clock.to_account_info(),
                message:          ctx.accounts.message.to_account_info(),
                emitter:          ctx.accounts.emitter.to_account_info(),
                sequence:         ctx.accounts.sequence.to_account_info(),
                fee_collector:    ctx.accounts.fee_collector.to_account_info(),
                rent:             ctx.accounts.rent.to_account_info(),
                system_program:   ctx.accounts.system_program.to_account_info(),
            },
        );
        // consistency_level = 0 for guaranteed-once delivery
        post_message(bridge_ctx, payload, 0)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnAndClose<'info> {
    // ——— Wormhole CPI accounts ———
    /// CHECK: The Wormhole Core Bridge program ID
    pub wormhole_program: UncheckedAccount<'info>,
    /// CHECK: The on-chain Wormhole config account
    #[account(address = wormhole_sdk::config::CONFIG_ADDRESS)]
    pub wormhole_config: UncheckedAccount<'info>,
    /// CHECK: PDA for this emitter (derived from your program ID)
    #[account(mut)]
    pub emitter: UncheckedAccount<'info>,
    #[account(mut)]
    pub sequence: UncheckedAccount<'info>,
    #[account(mut)]
    pub message: UncheckedAccount<'info>,
    /// CHECK: collects Wormhole fees
    #[account(mut)]
    pub fee_collector: UncheckedAccount<'info>,



    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// CHECK: Safe as only receiving lamports
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct BridgeBurnEvent {
    pub sui_receiver: [u8; 32],
    pub sol_sender: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Nothing to burn")]
    NothingToBurn,
}