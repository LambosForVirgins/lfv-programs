use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{constants::*, state::members::MemberAccount};

/**
 Self exclusion is the process of preventing oneself from
 further activity within the system. This mechanism is a
 safety mechanism for those feeling at risk of negative
 behaviour on the platform or simply wish to be excluded
 from member benefits or giveaways, such as the employee
 basis of the organisation.
 */
#[derive(Accounts)]
pub struct ExcludeAccounts<'info> {
    #[account(mut,
        seeds = [MemberAccount::SEED_PREFIX, signer.key().as_ref()],
        bump
    )]
    pub member_account: Account<'info, MemberAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,
}
