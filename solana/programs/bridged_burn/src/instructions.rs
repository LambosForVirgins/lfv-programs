use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Mint};
use crate::{
    Config,
    ConfigProposal,
    CONFIG_SEED,
    PROPOSAL_SEED,
    error::BridgeErrorCode,
};

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = payer,
        space = Config::SIZE,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CreateConfigProposal<'info> {
    #[account(
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    
    #[account(
        init,
        payer = payer,
        space = ConfigProposal::SIZE,
        seeds = [PROPOSAL_SEED, &proposal_id.to_le_bytes()],
        bump
    )]
    pub proposal: Account<'info, ConfigProposal>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub proposer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveConfigProposal<'info> {
    #[account(
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal.proposal_id.to_le_bytes()],
        bump
    )]
    pub proposal: Account<'info, ConfigProposal>,
    
    pub approver: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteConfigProposal<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, &proposal.proposal_id.to_le_bytes()],
        bump
    )]
    pub proposal: Account<'info, ConfigProposal>,
    
    pub executor: Signer<'info>,
}

#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct BurnAndClose<'info> {
    // ——— Configuration ———
    #[account(
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, Config>,
    
    // ——— Token accounts ———
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = owner,
        constraint = token_account.amount > 0 @ BridgeErrorCode::NothingToBurn
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    /// CHECK: Safe as only receiving lamports from closed token account
    /// Must match the vault address in config
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,

    // ——— System accounts ———
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}