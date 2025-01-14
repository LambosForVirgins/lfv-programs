use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{constants::*, state::members::MemberAccount};

#[derive(Accounts)]
pub struct TransferAccounts<'info> {
    #[account(mut,
        seeds = [MemberAccount::SEED_PREFIX, signer.key().as_ref()],
        bump
    )]
    pub member_account: Account<'info, MemberAccount>,

    #[account(mut,
        seeds = [MemberAccount::SEED_PREFIX_VAULT, mint.key().as_ref(), signer.key().as_ref()],
        token::mint = mint,
        token::authority = member_account,
        bump
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
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>, // SPL Token program
    pub system_program: Program<'info, System>,
    clock: Sysvar<'info, Clock>,
}
