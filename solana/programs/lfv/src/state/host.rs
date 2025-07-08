use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::*};
use crate::{constants::*, errors::*};
use solana_program::{program_pack::IsInitialized, pubkey};

#[account]
pub struct HostAccount {
    /** Schema version from v0 up to v255. Defaults to the `LATEST_VERSION` constant. */
    pub version: u8,
    /** Monthly slot locking period */
    pub maturation_period: u64,
    /** Ratio of entries granted to tokens locked */
    pub reward_factor: u64,

    pub total_members: u64,

    pub total_locked: u64,
}

impl HostAccount {
    pub const LATEST_VERSION: u8 = 1;

    pub const MINT: Pubkey = pubkey!("M1NTCgX3PG6hjpf4RAa7gjGrxz8rv4WeKzpW5cu31f8");
}

impl Default for HostAccount {
    fn default() -> Self {
        HostAccount {
            version: Self::LATEST_VERSION,
            maturation_period: 3, // 31557600 / 12;
            reward_factor: 100,
            total_members: 0,
            total_locked: 0,
        }
    }
}
