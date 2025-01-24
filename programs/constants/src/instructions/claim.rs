use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, MintTo, Token, TokenAccount},
};

use crate::{constants::*, errors::*, state::subscription::SubscriptionAccount};

#[derive(Accounts)]
pub struct ClaimAccounts<'info> {
    #[account(mut,
        seeds = [SUBSCRIPTION_SEED_PREFIX, signer.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, SubscriptionAccount>,

    // #[account(mut,
    //     // owner = spl_token_2022::id(),
    //     mint::token_program = token_program
    // )]
    // pub mint: Account<'info, Mint>,
    // #[account(mut,
    //     token::mint = mint
    // )]
    // pub token_account: Account<'info, TokenAccount>,
    // #[account(signer)]
    // pub authority: AccountInfo<'info>, // Authority of the mint
    #[account(mut)]
    pub signer: Signer<'info>,
    // #[account(address = spl_token_2022::id())]
    // pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

// impl<'info> ClaimAccounts<'info> {
//     pub fn initialize_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
//         let cpi_accounts = MintTo {
//             mint: self.mint.to_account_info(),
//             to: self.token_account.to_account_info(),
//             authority: self.authority.to_account_info(),
//         };

//         let cpi_program = self.token_program.to_account_info();

//         CpiContext::new(cpi_program, cpi_accounts)
//     }
// }
