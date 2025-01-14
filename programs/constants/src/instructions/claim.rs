use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount},
};

use crate::{constants::*, errors::*, state::members::MemberAccount};

#[derive(Accounts)]
pub struct ClaimAccounts<'info> {
    #[account(mut,
        seeds = [MemberAccount::SEED_PREFIX, signer.key().as_ref()],
        bump
    )]
    pub member_account: Account<'info, MemberAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}
