use crate::constants::*;

pub const fn lamports_to_sol(lamports: u64) -> u64 {
    lamports / 1_000_000_000
}

pub const fn tokens_to_rewards(token_amount: u64) -> u64 {
    token_amount / REWARD_FACTOR
}

pub const fn lamports_to_rewards(lamports: u64) -> u64 {
    lamports_to_sol(lamports) / REWARD_FACTOR
}
