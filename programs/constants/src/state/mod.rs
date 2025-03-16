use anchor_lang::prelude::*;

pub use slots::*;
pub use transaction::*;
pub use status::*;
pub use subscription::*;
pub use subscriptionv2::*;
pub use tiers::*;

pub mod slots;
pub mod transaction;
pub mod status;
pub mod subscription;
pub mod subscriptionv2;
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
