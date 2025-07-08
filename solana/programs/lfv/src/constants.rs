use solana_program::{pubkey, pubkey::Pubkey};

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

pub const ANCHOR_VECTOR_SIZE: usize = 4;

pub const MAX_PENDING_SLOTS: usize = 8;

pub const MINT_KEY: Pubkey = pubkey!("7kB8ZkSBJr2uiBWfveqkVBN7EpZMFom5PqeWUB62DCRD");

pub const ADMIN_KEY: Pubkey = pubkey!("DEV4MxokMrwCXpnJPjWREazY4sbw37fVPPaWtuf559Qp");
/** Monthly slot locking period */
pub const MATURATION_PERIOD: u64 = 2629800;
/** Ratio of entries granted to amount locked */
pub const REWARD_FACTOR: u64 = 1000;
/** Reward granted when member account is first activated */
pub const ACTIVATION_REWARD: u64 = 0;

pub const SUBSCRIPTION_SEED_PREFIX: &'static [u8] = b"subscription";

pub const VAULT_SEED_PREFIX: &'static [u8] = b"vault";

pub const REWARDS_SEED_PREFIX: &'static [u8] = b"reward";
