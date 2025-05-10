use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};
use crate::{
    constants::{MINT_KEY, SUBSCRIPTION_SEED_PREFIX, VAULT_SEED_PREFIX},
    state::SubscriptionAccountV2,
};

#[derive(Accounts)]
pub struct TransferAccounts<'info> {
    #[account(mut,
        seeds = [SUBSCRIPTION_SEED_PREFIX, signer.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, SubscriptionAccountV2>,

    #[account(
        init_if_needed,
        payer = signer,
        seeds = [VAULT_SEED_PREFIX, mint.key().as_ref(), signer.key().as_ref()],
        token::mint = mint,
        token::authority = subscription,
        bump,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = signer.as_ref(),
    )]
    pub source_token_account: Account<'info, TokenAccount>,

    #[account(
        address = MINT_KEY
    )]
    pub mint: Account<'info, Mint>, // TODO: Not needed for release/withdraw

    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> TransferAccounts<'info> {
    pub fn initialize_deposit_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        // Construct deposit transfer instruction
        let transfer_instruction: Transfer = Transfer {
            from: self.source_token_account.to_account_info(),
            to: self.vault_token_account.to_account_info(),
            authority: self.signer.to_account_info(),
        };
        // Initialize the transfer context
        CpiContext::new(self.token_program.to_account_info(), transfer_instruction)
    }

    pub fn initialize_withdraw_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        // Construct withdraw transfer instruction
        let transfer_instruction: Transfer = Transfer {
            from: self.vault_token_account.to_account_info(),
            to: self.source_token_account.to_account_info(),
            authority: self.subscription.to_account_info(),
        };
        // Initialize the transfer context
        CpiContext::new(self.token_program.to_account_info(), transfer_instruction)
    }
}
