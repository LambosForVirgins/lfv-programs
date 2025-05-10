use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::*};
use crate::{
    constants::*,
    errors::TransferError,
    transaction::Transaction,
    status::AccountStatus,
    tiers::MemberTier, utils::*,
};
// use solana_program::program_pack::IsInitialized;

#[account]
pub struct SubscriptionAccount {
    /** Schema version from v0 up to v255. Defaults to the `LATEST_VERSION` constant. */
    pub version: u8,
    /** Current membership tier according to the total locked tokens */
    pub tier: u8,
    /** Members account status */
    pub status: u8,
    /** Total amount of tokens managed by the account */
    pub total_amount: u64,
    /** Amount of tokens passed their first epoch */
    pub total_matured: u64,
    /** Amount of tokens pending release */
    pub total_released: u64,
    /** Amount of unclaimed entry tokens */
    pub total_rewards: u64,
    /** Initial creation date of the members account */
    pub time_created: u64,
    /** Date of the last reward granted to matured tokens */
    pub time_rewarded: u64,
    /** Collection of locked slots */
    pub slots: Vec<Transaction>,
}

#[constant]
impl SubscriptionAccount {
    pub const LATEST_VERSION: u8 = 1;
    pub const MAX_SLOTS: usize = 8;

    pub fn claim(&mut self, time_now: u64) -> Result<u64> {
        // Determine unclaimed rewards
        let mut rewards: u64 = self.get_unclaimed_rewards(time_now);
        // Mature existing slots and collect rewards
        rewards += self.mature_slots(time_now).unwrap();
        // Update the reward timestamp
        self.time_rewarded = time_now;
        self.total_rewards = 0;
        // Return outstanding rewards
        Ok(rewards)
    }

    pub fn lock(&mut self, amount: u64, time_now: u64) -> Result<()> {
        // Mature existing slots
        self.mature_slots(time_now)?;
        // Update locked amount counter
        self.total_amount += amount;
        // Immediately grant entries
        self.total_rewards += lamports_to_rewards(amount);
        // Allocate a new deposit slot and store
        let new_slot = Transaction::Deposit {
            amount: amount,
            time_created: time_now,
            time_matured: time_now + MATURATION_PERIOD,
        }; // TODO: Compress slots
           // Append the deposit slot if not compressed
        self.slots.push(new_slot);
        // Return outstanding rewards
        Ok(())
    }

    pub fn unlock(&mut self, amount: u64, time_now: u64) -> Result<u64> {
        // Claim outstanding rewards
        self.claim(time_now).unwrap();
        let time_next_epoch: u64 = self.time_rewarded + MATURATION_PERIOD;
        // Check sufficient token maturity
        require!(
            self.total_matured >= amount,
            TransferError::InsufficientBalance
        );
        // Create a pending withdrawal transaction
        let withdrawal = Transaction::Withdraw {
            time_released: time_next_epoch, // Released after next epoch
            amount,
        };
        // Append the new locked slot if it can't be compress
        self.slots.push(withdrawal);
        // We shift balance from locked to liquidity
        self.total_matured -= amount;
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
        let target_balance: u64 = target_account.get_lamports();
        let new_account_size: usize = self.get_packed_len();
        let rent_exempt_minimum: u64 = rent.minimum_balance(new_account_size);

        if target_balance < rent_exempt_minimum {
            // Transfer additional lamports from payer to target (member) rent
            let additional_lamports: u64 = rent_exempt_minimum - target_balance;
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

    pub fn on_withdraw(&mut self, time_now: u64) -> Result<u64> {
        // Mature withdrawal slots for release
        self.mature_slots(time_now)?; // TODO: Handle error
        // Get the updated release amount
        let amount = self.total_released;

        self.total_released = 0;
        self.total_amount -= amount;

        Ok(amount)
    }

    pub fn mature_slots(&mut self, time_now: u64) -> Result<u64> {
        let mut matured_change: u64 = 0;
        let mut matured_rewards: u64 = 0;
        let mut released_change: u64 = 0;
        // Cleanup matured slots and update balances
        self.slots.retain(|slot: &Transaction| {
            if slot.is_matured(time_now) {
                match slot {
                    Transaction::Deposit {
                        amount,
                        time_created,
                        time_matured,
                    } => {
                        matured_change += amount;
                        // Determine how many reward cycles have elapsed since maturity
                        let epoch_length: u64 = time_matured - time_created;
                        // Adds an additional cycle because the maturation has completed (1 cycle)
                        let outstanding_cycles: u64 = (time_now - time_matured) / epoch_length + 1;
                        matured_rewards += lamports_to_rewards(outstanding_cycles * amount);
                    }
                    Transaction::Withdraw { amount, .. } => {
                        released_change += amount;
                    }
                }

                false
            } else {
                true
            }
        });
        // Update total matured and released balances
        self.total_matured += matured_change;
        self.total_released += released_change;
        // Update the accrued rewards for claiming
        self.total_rewards += matured_rewards;
        // Update member tier for new balances
        self.tier = MemberTier::from_tier(MemberTier::get_tier(
            self.total_amount - self.total_released,
        ));

        Ok(matured_rewards)
    }

    fn get_unclaimed_rewards(&self, time_now: u64) -> u64 {
        let unclaimed_rewards = self.total_rewards;
        let total_epochs: u64 = self.get_unclaimed_epochs(time_now);
        lamports_to_rewards(self.total_matured * total_epochs) + unclaimed_rewards
    }

    fn get_unclaimed_epochs(&self, time_now: u64) -> u64 {
        let time_elapsed: u64 = time_now - self.time_rewarded;
        return time_elapsed / MATURATION_PERIOD;
    }

    pub fn get_packed_len(&self) -> usize {
        self.space(self.slots.len())
    }

    pub const fn space(&self, total_slots: usize) -> usize {
        ANCHOR_DISCRIMINATOR_SIZE
            + std::mem::size_of::<SubscriptionAccount>()
            + ANCHOR_VECTOR_SIZE
            + (std::mem::size_of::<Transaction>() * total_slots)
    }
}


impl Default for SubscriptionAccount {
    fn default() -> Self {
        SubscriptionAccount {
            version: Self::LATEST_VERSION,
            tier: MemberTier::from_tier(MemberTier::Pending),
            status: AccountStatus::Pending.to_u8(),
            total_amount: 0,
            total_matured: 0,
            total_released: 0,
            total_rewards: 0,
            time_created: 0,
            time_rewarded: 0,
            slots: Vec::new(),
        }
    }
}
