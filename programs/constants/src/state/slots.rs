use anchor_lang::prelude::*;
use crate::{constants::*, errors::*};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct LockedSlot {
    /** Amount of tokens deposited */
    pub amount: u64,
    /** Unix timestamp of the initial deposit */
    pub time_created: u64,
    /** Unix timestamp of the last reward granted */
    pub time_rewarded: u64,
    /** Indicates recurring lock period */
    pub enabled: bool,
}

impl LockedSlot {
    const MAX_AMOUNT: u64 = 100000000;

    pub fn new(amount: u64, time_now: u64) -> Self {
        Self {
            amount,
            time_created: time_now,
            time_rewarded: time_now,
            enabled: true,
        }
    }

    pub const fn space() -> usize {
        ANCHOR_DISCRIMINATOR_SIZE + std::mem::size_of::<LockedSlot>()
    }

    pub fn on_reward_granted(&mut self, time_now: u64) {
        self.time_rewarded = time_now;
    }

    /** Disables the recurring lock period when the member
     * intends to withdraw funds after the next reward
     * allocation event */
    pub fn on_cancel_subscription(&mut self) {
        self.enabled = false;
    }

    pub fn on_enable_subscription(&mut self, time_now: u64) {
        if self.is_reward_available(time_now) {
            self.enabled = true;
        }
    }

    pub fn on_release(&self) -> u64 {
        // Check current `amount` exceeds slot bounds
        // Release any excess locked tokens when the slot token bounds change to be less than current `amount`.
        return 0;
    }

    /** Checks if the slot has matured past the first epoch
     by checking the `time_created + MATURATION_PERIOD` exceeds
     the current time `now` */
    pub fn is_matured(&self, time_now: u64) -> bool {
        time_now >= self.time_created + MATURATION_PERIOD
    }

    pub fn is_lockout_expired(&self, time_now: u64) -> bool {
        time_now > self.time_rewarded + MATURATION_PERIOD
    }

    /** Alternative entry for `is_withdraw_available` for
     * improved readability in some use cases */
    pub fn is_reward_available(&self, time_now: u64) -> bool {
        self.enabled && self.is_lockout_expired(time_now)
    }

    pub fn is_withdraw_available(&self, time_now: u64) -> bool {
        !self.enabled && self.is_lockout_expired(time_now)
    }

    pub fn get_amount_outstanding(&self) -> u64 {
        // TODO: Make this respect the current tier constraints
        let outstanding = Self::MAX_AMOUNT - self.amount;
        return outstanding;
    }
}

// impl Default for LockedSlot {
//     fn default() -> Self {
//         LockedSlot {
//             amount: 0,
//             time_created: 0,
//             time_rewarded: 0,
//             enabled: true,
//         }
//     }
// }
