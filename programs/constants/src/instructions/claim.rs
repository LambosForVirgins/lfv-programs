use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, MintTo, Token, TokenAccount};

use crate::{
    constants::{REWARDS_SEED_PREFIX, SUBSCRIPTION_SEED_PREFIX},
    state::SubscriptionAccountV2,
};

#[derive(Accounts)]
pub struct ClaimAccounts<'info> {
    #[account(mut,
        seeds = [SUBSCRIPTION_SEED_PREFIX, signer.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, SubscriptionAccountV2>,

    #[account(mut,
        seeds = [REWARDS_SEED_PREFIX],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut,
        token::mint = mint
    )]
    pub destination_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimAccounts<'info> {
    pub fn initialize_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                authority: self.mint.to_account_info(),
                to: self.destination_token_account.to_account_info(),
                mint: self.mint.to_account_info(),
            },
        )
    }
}
