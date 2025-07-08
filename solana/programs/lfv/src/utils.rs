use crate::constants::*;

pub const fn lamports_to_sol(lamports: u64) -> u64 {
    match lamports.checked_div(10u64.pow(9)) {
        Some(result) => result,
        None => panic!("Multiplication overflowed"),
    }
}

pub const fn lamports_to_rewards(lamports: u64) -> u64 {
    match lamports_to_sol(lamports).checked_div(REWARD_FACTOR) {
        Some(result) => result,
        None => panic!("Multiplication overflowed"),
    }
}

pub const fn rewards_to_lamports(lamports: u64) -> u64 {
    match lamports.checked_mul(10u64.pow(4)) {
        Some(result) => result,
        None => panic!("Multiplication overflowed"),
    }
}
