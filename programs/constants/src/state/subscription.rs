use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::*};
use crate::{
    constants::*, errors::*, slots::Transaction, status::AccountStatus, tiers::MemberTier, utils::*,
};
use solana_program::program_pack::IsInitialized;

#[account]
pub struct SubscriptionAccount {
    /** Schema version from v0 up to v255. Defaults to the `LATEST_VERSION` constant. */
    pub version: u8,
    /** Persists the tier of the greatest filled subscription slot */
    pub tier: u8,
    pub status: u8,
    pub entries: u64,
    /** Total amount of tokens managed by the account */
    pub total_amount: u64,
    /** Amount of tokens passed their first epoch */
    pub total_matured: u64,
    /** Amount of tokens pending release */
    pub total_released: u64,
    /** Initial creation date of the members account */
    pub time_created: u64,
    /** Date of the last reward granted to matured tokens */
    pub time_rewarded: u64,
    /** Collection of locked token deposit slots (one entry per deposit) */
    pub slots: Vec<Transaction>,
}

#[constant]
impl SubscriptionAccount {
    pub const LATEST_VERSION: u8 = 1;
    pub const MAX_SLOTS: usize = 8;

    pub fn on_claim(&mut self, time_now: u64) -> Result<u64> {
        // Determine unclaimed rewards
        let mut rewards: u64 = self.get_unclaimed_rewards(time_now);
        // Mature existing slots and collect rewards
        rewards += self.mature_slots(time_now);
        // Update the reward timestamp
        self.time_rewarded = time_now;
        // Return outstanding rewards
        Ok(rewards)
    }

    // on_lock
    pub fn on_deposit(&mut self, amount: u64, time_now: u64) -> Result<u64> {
        require!(
            self.slots.len() < Self::MAX_SLOTS,
            LockingError::MaxSlotsExceeded
        );
        // Grant entries and update locked amount counter
        self.total_amount += amount;
        // Claim outstanding rewards prior to everything
        let mut rewards: u64 = self.on_claim(time_now).unwrap();
        let new_slot = Transaction::Deposit {
            amount: amount,
            time_created: time_now,
            time_matured: time_now + MATURATION_PERIOD,
        };
        // TODO: Compress slots
        // Append the new locked slot if it can't be compress
        self.slots.push(new_slot);

        rewards += lamports_to_rewards(amount);
        // Return outstanding rewards
        Ok(rewards)
    }

    // on_unlock
    pub fn on_release(&mut self, amount: u64, time_now: u64) -> Result<u64> {
        // Claim outstanding rewards
        let rewards: u64 = self.on_claim(time_now).unwrap();
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
        // Return outstanding rewards
        Ok(rewards)
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
        let new_account_size: usize = self.get_space();
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
        // Mature withdarwal slots for release
        self.mature_slots(time_now);
        // Get the updated release amount
        let amount = self.total_released;

        self.total_released = 0;
        self.total_amount -= amount;

        Ok(amount)
    }

    pub fn mature_slots(&mut self, time_now: u64) -> u64 {
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
                        let outstanding_cycles: u64 = (time_now - time_matured) / epoch_length;
                        matured_rewards += outstanding_cycles * amount;
                    }
                    Transaction::Withdraw {
                        amount,
                        time_released,
                    } => {
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
        // Update member tier for new balances
        self.tier = MemberTier::from_tier(MemberTier::get_tier(
            self.total_amount - self.total_released,
        ));

        lamports_to_rewards(matured_rewards)
    }

    fn get_unclaimed_rewards(&self, time_now: u64) -> u64 {
        let total_epochs: u64 = self.get_unclaimed_epochs(time_now);
        lamports_to_rewards(self.total_matured * total_epochs)
    }

    fn get_unclaimed_epochs(&self, time_now: u64) -> u64 {
        let time_elapsed: u64 = time_now - self.time_rewarded;
        return time_elapsed / MATURATION_PERIOD;
    }

    /**
     Updates the account status when current status
     is not `AccountStatus::Excluded` or `AccountStatus::Suspended`
     */
    pub fn update_status(&mut self, new_status: AccountStatus) -> Result<()> {
        let status: AccountStatus = AccountStatus::from_u8(self.status);
        match status {
            AccountStatus::Pending | AccountStatus::Paused | AccountStatus::Active => {
                self.status = new_status.to_u8();
                Ok(())
            }
            _ => Err(MemberError::ImmutableAccountStatus.into()),
        }
    }

    pub fn get_space(&self) -> usize {
        self.space(self.slots.len())
    }

    pub const fn space(&self, total_slots: usize) -> usize {
        ANCHOR_DISCRIMINATOR_SIZE
            + std::mem::size_of::<SubscriptionAccount>()
            + ANCHOR_VECTOR_SIZE
            + (std::mem::size_of::<Transaction>() * total_slots)
    }
}

impl IsInitialized for SubscriptionAccount {
    fn is_initialized(&self) -> bool {
        AccountStatus::not(AccountStatus::Pending, self.status)
    }
}

impl Default for SubscriptionAccount {
    fn default() -> Self {
        SubscriptionAccount {
            version: Self::LATEST_VERSION,
            tier: MemberTier::from_tier(MemberTier::Pending),
            status: AccountStatus::Pending.to_u8(),
            entries: 0,
            total_amount: 0,
            total_matured: 0,
            total_released: 0,
            time_created: 0,
            time_rewarded: 0,
            slots: Vec::new(),
        }
    }
}
