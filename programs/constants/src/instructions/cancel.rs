use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount},
};

use crate::{constants::*, errors::*, state::subscription::SubscriptionAccount};

#[derive(Accounts)]
pub struct CancelAccounts<'info> {
    #[account(mut,
        seeds = [SUBSCRIPTION_SEED_PREFIX, signer.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, SubscriptionAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}
