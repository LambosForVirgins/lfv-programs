use anchor_lang::{
    prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, solana_program::program_pack::Pack,
};
use anchor_spl::token::spl_token;

pub use host::*;
pub use members::*;
pub use slots::*;
pub use status::*;
pub use tiers::*;

pub mod host;
pub mod members;
pub mod slots;
pub mod status;
pub mod tiers;

#[account]
#[derive(Debug)]
pub struct SystemAccount {
    /**
     Schema version from v0 up to v255. Defaults
     to the `LATEST_VERSION` constant.
     */
    pub version: u8, // 1 byte

    pub mint: Pubkey,

    pub reward_factor: u64,

    pub activation_reward: u64,
}

impl SystemAccount {
    pub const SEED_PREFIX: &'static [u8] = b"locker_system";

    pub const LATEST_VERSION: u8 = 1;
    /** Ratio of entries granted to amount deposited */
    const REWARD_FACTOR: u64 = 100;
    /** Reward granted when member account is first activated */
    const ACTIVATION_REWARD: u64 = 1;
}

impl Default for SystemAccount {
    fn default() -> Self {
        SystemAccount {
            version: Self::LATEST_VERSION,
            mint: Pubkey::default(),
            reward_factor: Self::REWARD_FACTOR,
            activation_reward: Self::ACTIVATION_REWARD,
        }
    }
}
