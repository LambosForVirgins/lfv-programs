use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::{constants::*, state::members::MemberAccount};

#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
    #[account(
        init, 
        payer = signer,
        seeds = [MemberAccount::SEED_PREFIX, signer.key().as_ref()],
        bump,
        space = ANCHOR_DISCRIMINATOR_SIZE + std::mem::size_of::<MemberAccount>(),
    )]
    pub member_account: Account<'info, MemberAccount>,

    /**
     * Derive an associated token account between the signers unique
     * program member account and the token mint. This will hold the
     * mint tokens under management of the program with respect
     * to the signing member and mint, much like a token vault.
     */
    #[account(
        init,
        payer = signer,
        seeds = [MemberAccount::SEED_PREFIX_VAULT,  mint.key().as_ref(), signer.key().as_ref()],
        token::mint = mint,
        token::authority = member_account,
        bump
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(
        mint::token_program = token_program
    )]
    mint: Account<'info, Mint>,

    #[account(mut)]
    signer: Signer<'info>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}
