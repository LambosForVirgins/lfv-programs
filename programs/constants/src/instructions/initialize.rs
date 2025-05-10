use anchor_lang::prelude::*;
use crate::{
    constants::{ANCHOR_DISCRIMINATOR_SIZE, SUBSCRIPTION_SEED_PREFIX},
    state::SubscriptionAccountV2,
};

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(
        init,
        payer = signer,
        seeds = [SUBSCRIPTION_SEED_PREFIX, signer.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR_SIZE + std::mem::size_of::<SubscriptionAccountV2>(),
    )]
    pub subscription: Account<'info, SubscriptionAccountV2>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}
