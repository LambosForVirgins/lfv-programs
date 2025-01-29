use anchor_lang::prelude::*;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Debug)]
pub enum Transaction {
    Deposit {
        amount: u64,
        time_created: u64,
        time_matured: u64,
    },
    Withdraw {
        amount: u64,
        time_released: u64,
    },
}

impl Transaction {
    pub const fn space() -> usize {
        ANCHOR_DISCRIMINATOR_SIZE + std::mem::size_of::<Transaction>()
    }

    /** Checks if the slot has matured past the first epoch
     by checking the `time_matured + MATURATION_PERIOD` exceeds
     the current time `now` */
    pub fn is_matured(&self, time_now: u64) -> bool {
        match self {
            Transaction::Deposit { time_matured, .. } => time_now >= *time_matured,
            Transaction::Withdraw { time_released, .. } => time_now >= *time_released,
        }
    }
}
