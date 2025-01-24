use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::*};
use crate::{
    constants::*, errors::*, slots::Transaction, status::AccountStatus, tiers::MemberTier,
};
use solana_program::program_pack::IsInitialized;

#[account]
pub struct MetaAccount {
    /** Schema version from v0 up to v255. Defaults to the `LATEST_VERSION` constant. */
    pub version: u8,
    pub name: u64,
}
