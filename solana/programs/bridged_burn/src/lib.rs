use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint, Burn, CloseAccount};
use solana_program::clock::Clock;
use borsh::{BorshDeserialize, BorshSerialize};
use error::*;
use state::*;

mod error;
mod state;

declare_id!("4rGdLkQDuZcJhCM85wwcpcyM7t5GxtpjAapV2LR6buiK");

/// Wormhole Chain ID for Sui Network
pub const SUI_CHAIN_ID: u16 = 21;

/// Message type identifier for burn confirmation
pub const BURN_CONFIRMATION_MESSAGE_TYPE: u8 = 1;

/// Wormhole Core Bridge program ID (mainnet)
pub const WORMHOLE_PROGRAM_ID: &str = "3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5";

#[cfg(test)]
mod tests {
    mod bridge_tests;
    mod integration_tests;
    mod refund_tests;
}

#[program]
pub mod bridged_burn {
    use super::*;

    /// Burns tokens and closes the token account,
    /// then sends a confirmation message to Sui via Wormhole
    pub fn burn_and_close(
        ctx: Context<BurnAndClose>,
        sui_address: [u8; 32],
    ) -> Result<()> {
        // Validate token amount
        let burn_amount = ctx.accounts.token_account.amount;
        require!(burn_amount > 0, BridgeErrorCode::NothingToBurn);

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
    ctx: &Context<BurnAndClose>,
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

#[derive(Accounts)]
pub struct BurnAndClose<'info> {
    // ——— Token accounts ———
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = owner,
        constraint = token_account.amount > 0 @ BridgeErrorCode::NothingToBurn
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// CHECK: Safe as only receiving lamports from closed token account
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,

    // ——— System accounts ———
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct BridgeBurnEvent {
    pub sui_receiver: [u8; 32],
    pub sol_sender: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WormholeMessageEvent {
    pub target_chain: u16,
    pub payload: Vec<u8>,
    pub consistency_level: u8,
}