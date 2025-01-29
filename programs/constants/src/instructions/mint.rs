use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{CreateMetadataAccountsV3, Metadata},
    token::{Mint, Token},
};
use crate::constants::{ADMIN_KEY, REWARDS_SEED_PREFIX};

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct InitializeRewardsParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub decimals: u8,
}

#[derive(Accounts)]
#[instruction(params: InitializeRewardsParams)]
pub struct InitializeRewards<'info> {
    /// CHECK: New Metaplex Account being created
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    // Create reward mint owned by the program
    #[account(
        init,
        seeds = [REWARDS_SEED_PREFIX],
        bump,
        payer = admin,
        mint::decimals = 4,
        mint::authority = mint.key(), // Grant program authority without additional PDA
        owner = token_program.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut,
        address = ADMIN_KEY
    )]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
}

impl<'info> InitializeRewards<'info> {
    pub fn initialize_metadata_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, CreateMetadataAccountsV3<'info>> {
        CpiContext::new(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                payer: self.admin.to_account_info(),
                update_authority: self.admin.to_account_info(),
                mint: self.mint.to_account_info(),
                metadata: self.metadata.to_account_info(),
                mint_authority: self.mint.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
        )
    }
}
