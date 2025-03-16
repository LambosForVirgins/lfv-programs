use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::*};
use crate::errors::RewardError;
use crate::{
    constants::{SLOT_COUNT,MATURATION_PERIOD,ANCHOR_DISCRIMINATOR_SIZE,ANCHOR_VECTOR_SIZE},
    slots::Slot,
    transaction::Transaction,
    status::AccountStatus,
    tiers::MemberTier, utils::*,
};

#[account]
pub struct SubscriptionAccountV2 {
    /** Schema version from v0 up to v255. Defaults to the `LATEST_VERSION` constant. */
    pub version: u8,
    /** Current membership tier according to the total locked tokens */
    pub tier: u8,
    /** Members account status */
    pub status: u8,
    /** Total amount of tokens managed by the account */
    pub total_amount: u64, // TODO: Not required as vault token account is source of truth
    /** Initial creation date of the members account */
    pub time_created: u64,
    /** Date the last reward claim was processed */
    pub time_processed: u64,
    /** Collection of locking slots */
    pub slots: [Slot; 4],
    /** Collection of pending transactions */
    pub queue: Vec<Transaction>,
}

/**
 * Determines the current slot index relative to the current `time_now`.
 * Assuming there's 4 slots within the maturation period.
 */
fn calculate_slot_index(subscription: &SubscriptionAccountV2, time_now: u64) -> usize {
    let time_elapsed = time_now - subscription.time_created;
    let cycles_elapsed = time_elapsed / (MATURATION_PERIOD / SLOT_COUNT);
    (cycles_elapsed % SLOT_COUNT) as usize
}

#[constant]
impl SubscriptionAccountV2 {
    pub const LATEST_VERSION: u8 = 2;
    pub const MAX_SLOTS: usize = 4;

    fn activate(&mut self, time_now: u64) -> () {
        self.time_created = time_now;
        self.time_processed = time_now;
        // Iterate over each slot and activate
        for slot in self.slots.iter_mut() {
            match slot {
                Slot::V1 {
                    time_processed,
                    time_matured,
                    ..
                } => {
                    *time_processed = time_now;
                    *time_matured = time_now + MATURATION_PERIOD;
                }
            }
        }
        // Update the account status
        self.status = AccountStatus::Active.to_u8();
    }

    pub fn process_rewards(&mut self, time_now: u64) -> Result<u64> {
        // Determine unclaimed rewards
        let rewards = self.get_unclaimed_rewards(time_now).unwrap();
        // Update the reward timestamp
        self.time_processed = time_now;
        // Return outstanding rewards
        Ok(rewards)
    }

    pub fn deposit_bond(&mut self, deposit_amount: u64, time_now: u64) -> () {
        // Update the total amount reference
        self.total_amount += deposit_amount;

        if self.status == AccountStatus::Pending.to_u8() {
            msg!("Activating subscription account");
            self.activate(time_now);
        } else {
            msg!("Subscription account active");
        }
        // Determine the vaulting slot index
        let slot_index = calculate_slot_index(self, time_now);
        // Create a new vaulted token slot
        match &mut self.slots[slot_index] {
            Slot::V1 {
                amount,
                time_processed,
                time_matured,
                rewards_accrued,
            } => {
                *amount += deposit_amount;
                *time_processed = time_now;
                *time_matured = time_now + MATURATION_PERIOD;
                *rewards_accrued += lamports_to_rewards(deposit_amount);
            }
        }
    }

    pub fn unlock(&mut self, amount: u64, time_now: u64) -> Result<u64> {

        let time_next_epoch: u64 = self.time_processed + MATURATION_PERIOD;
        // Create a pending withdrawal transaction
        let withdrawal = Transaction::Withdraw {
            time_released: time_next_epoch, // Released after next epoch
            amount,
        };
        
        self.queue.push(withdrawal);
        // Return amount released
        Ok(amount)
    }

    pub fn realloc<'a>(
        &self,
        target_account: &AccountInfo<'a>,
        payer_account: &AccountInfo<'a>,
        system_program: &AccountInfo<'a>,
    ) -> Result<()> {
        // Calculate rent balance adjustment required from the payer
        let rent = Rent::get()?; // TODO: Handle error mapping
        let current_rental_bond: u64 = target_account.get_lamports();
        let new_account_size: usize = self.get_packed_len();
        let rent_exempt_minimum: u64 = rent.minimum_balance(new_account_size);

        if current_rental_bond < rent_exempt_minimum {
            // Transfer additional lamports from payer to target (member) rent
            let additional_lamports: u64 = rent_exempt_minimum - current_rental_bond;
            let instruction = transfer(payer_account.key, target_account.key, additional_lamports);
            invoke(
                &instruction,
                &[
                    payer_account.clone(),
                    target_account.clone(),
                    system_program.clone(),
                ],
            )?; // TODO: Handle error mapping
        }

        target_account.realloc(new_account_size, false)?;
        Ok(())
    }

    /**
     Determines the total amount of tokens available for withdrawal.
     Tokens become available once `time_now` is greater-than or equal-to
     the `time_released` of items in the transaction `queue`.
     */
    pub fn process_withdrawals(&mut self, time_now:u64) -> Result<u64> {
        // Split the queue into released and pending withdrawals
        let (released, pending): (Vec<_>, Vec<_>) = self.queue
            .drain(..)
            .partition(|item| match item {
                Transaction::Withdraw { time_released, .. } => *time_released <= time_now,
                Transaction::Deposit { .. } => todo!() // TODO: This will panic!
            });
        // Store the pending withdrawals
        self.queue = pending;
        // Calculate the total released amount
        let total_released = released.into_iter().fold(0u64, |sum, item| {
            match item {
                Transaction::Withdraw { amount, .. } => {
                    sum.checked_add(amount).unwrap_or(sum)
                }
                Transaction::Deposit { .. } => todo!() // TODO: This will panic!
            }
        });

        Ok(total_released)
    }

    fn get_unclaimed_rewards(&self, time_now: u64) -> Result<u64> {
        let mut total_rewards = 0u64;
        // Iterate over each slot and sum the outstanding rewards
        for slot in self.slots.iter() {
            let rewards = slot.rewards_outstanding_since(time_now)?;
            total_rewards = total_rewards
                .checked_add(rewards)
                .ok_or(RewardError::RewardOverflow)?;
        }
        Ok(total_rewards)
    }

    pub fn get_packed_len(&self) -> usize {
        self.space(self.queue.len())
    }

    pub const fn space(&self, queue_size: usize) -> usize {
        ANCHOR_DISCRIMINATOR_SIZE
            + std::mem::size_of::<SubscriptionAccountV2>()
            + ANCHOR_VECTOR_SIZE
            + (std::mem::size_of::<Transaction>() * queue_size)
    }
}

impl Default for SubscriptionAccountV2 {
    fn default() -> Self {
        SubscriptionAccountV2 {
            version: Self::LATEST_VERSION,
            tier: MemberTier::from_tier(MemberTier::Pending),
            status: AccountStatus::Pending.to_u8(),
            total_amount: 0,
            time_created: 0,
            time_processed: 0,
            slots: [Slot::default(); Self::MAX_SLOTS],
            queue: Vec::new(),
        }
    }
}
