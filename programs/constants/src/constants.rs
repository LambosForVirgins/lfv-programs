use solana_program::{pubkey, pubkey::Pubkey};

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

pub const ANCHOR_VECTOR_SIZE: usize = 4;

pub const MINT_KEY: Pubkey = pubkey!("M1NTCgX3PG6hjpf4RAa7gjGrxz8rv4WeKzpW5cu31f8");

/** Monthly slot locking period */
pub const MATURATION_PERIOD: u64 = 3; // 31557600 / 12;
/** Ratio of entries granted to amount locked */
pub const REWARD_FACTOR: u64 = 100;

pub const fn lamports_to_sol(lamports: u64) -> u64 {
    lamports / 1_000_000_000
}

pub const fn tokens_to_vouchers(lamports: u64) -> u64 {
    lamports_to_sol(lamports) / REWARD_FACTOR
}
