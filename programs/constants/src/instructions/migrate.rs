use anchor_lang::prelude::*;
use crate::constants::SUBSCRIPTION_SEED_PREFIX;

#[derive(Accounts)]
pub struct MigrateAccounts<'info> {
    /// CHECK: Deserialization checks occur within the function
    #[account(mut,
        seeds = [SUBSCRIPTION_SEED_PREFIX, signer.key().as_ref()],
        bump
    )]
    pub subscription: AccountInfo<'info>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}
