use anchor_lang::prelude::*;
use crate::{
    constants::{ANCHOR_DISCRIMINATOR_SIZE, SUBSCRIPTION_SEED_PREFIX},
    state::subscription::SubscriptionAccount,
};

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [SUBSCRIPTION_SEED_PREFIX, signer.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR_SIZE + std::mem::size_of::<SubscriptionAccount>(),
    )]
    pub subscription: Account<'info, SubscriptionAccount>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}
