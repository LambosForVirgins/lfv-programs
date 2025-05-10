use anchor_lang::prelude::*;

use crate::{constants::MATURATION_PERIOD, errors::RewardError, utils::lamports_to_rewards};

#[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, Debug)]
pub enum Slot {
    V1 {
        amount: u64,
        time_processed: u64,
        time_matured: u64,
        rewards_accrued: u64,
    }
}

impl Default for Slot {
    fn default() -> Self {
        Slot::V1 {
            amount: 0,
            time_processed: 0,
            time_matured: 0,
            rewards_accrued: 0,
        }
    }
}

impl Slot {
    /// Calculates the outstanding rewards for this slot given the current time.
    ///
    /// For a slot to earn rewards, the current time must be at or after the `time_matured`.
    /// The number of full epoch cycles is computed as:
    ///
    ///  cycles = (now - time_processed) / MATURATION_PERIOD
    ///
    /// Each cycle grants a reward for every 1000 tokens in the `amount` field.
    ///
    /// # Parameters
    /// - `time_now`: The current Unix timestamp.
    ///
    /// # Returns
    /// The calculated rewards for this slot, or an error if the epoch duration
    /// is invalid or if arithmetic overflows.
    pub fn rewards_outstanding_since(&self, time_now: u64) -> Result<u64> {
        match self {
            Slot::V1 {
                amount,
                time_processed,
                rewards_accrued,
                ..
            } => {
                if !self.is_initialized() {
                    return Ok(0);
                }
                // Calculate the number of complete cycles.
                let cycles = (time_now - *time_processed) / MATURATION_PERIOD;
                // Reward multiplier is 1 reward per 1000 tokens in the `amount`.
                let reward_multiplier = lamports_to_rewards(*amount);
                // Calculate rewards ensuring no overflow occurs.
                let rewards = cycles
                    .checked_mul(reward_multiplier)
                    .ok_or(RewardError::RewardOverflow)?;
                // Add the accrued rewards to the total.
                let total_rewards = rewards
                    .checked_add(*rewards_accrued)
                    .ok_or(RewardError::RewardOverflow)?;
                Ok(total_rewards)
            }
        }
    }

    pub fn is_initialized(&self) -> bool {
        match self {
            Slot::V1 {
                time_processed,
                ..
            } => {
                if time_processed == &0 {
                    return false;
                }
                true
            }
        }
    }
}
