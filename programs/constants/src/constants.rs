use solana_program::{pubkey, pubkey::Pubkey};

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

pub const ANCHOR_VECTOR_SIZE: usize = 4;

pub const MINT_KEY: Pubkey = pubkey!("LFVqPrRGnwYdCwFcDzShBxN2GMFmD4AoCMrjxjq4xdz");

const MONTHLY: u64 = 31557600 / 12;

const QUARTER_DAY: u64 = 86400 / 4;

const TESTING_PERIOD: u64 = 600;
/** Monthly slot locking period */
pub const MATURATION_PERIOD: u64 = TESTING_PERIOD;
/** Ratio of entries granted to amount locked */
pub const REWARD_FACTOR: u64 = 1000;
/** Reward granted when member account is first activated */
pub const ACTIVATION_REWARD: u64 = 0;

pub const SUBSCRIPTION_SEED_PREFIX: &'static [u8] = b"subscription";

pub const VAULT_SEED_PREFIX: &'static [u8] = b"vault";

pub const REWARDS_SEED_PREFIX: &'static [u8] = b"reward";
