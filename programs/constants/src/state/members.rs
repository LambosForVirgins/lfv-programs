use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction::*};
use crate::{
    constants::*, errors::*, slots::Transaction, status::AccountStatus, tiers::MemberTier,
};
use solana_program::program_pack::IsInitialized;

#[account]
pub struct MemberAccount {
    /** Schema version from v0 up to v255. Defaults to the `LATEST_VERSION` constant. */
    pub version: u8,
    pub status: u8,
    /** Persists the tier of the greatest filled subscription slot */
    pub tier: u8,
    /** Total amount of tokens managed by the account */
    pub total_amount: u64,
    /** Amount of tokens passed their first epoch */
    pub total_matured: u64,
    /** Amount of tokens pending release */
    pub total_pending: u64,
    /** Persists the total amount of vouchers accrued minus redeemed */
    pub total_entries: u64,
    /** Initial creation date of the members account */
    pub time_created: u64,
    /** Date of the last reward granted to matured tokens */
    pub time_rewarded: u64,
    /** Collection of locked token deposit slots (one entry per deposit) */
    pub slots: Vec<Transaction>,
}

#[constant]
impl MemberAccount {
    pub const LATEST_VERSION: u8 = 1;
    pub const SEED_PREFIX: &'static [u8] = b"member_account";
    pub const SEED_PREFIX_VAULT: &'static [u8] = b"vault_token_account";
    pub const MAX_SLOTS: usize = 8;
    /** Reward granted when member account is first activated */
    const ACTIVATION_REWARD: u64 = 1;

    /// Generates the seeds array for PDA derivation
    // pub fn seeds<'a>(authority: &'a Pubkey, bump: u8) -> [&'a [u8]; 3] {
    //     let bump_vec = [bump];
    //     [Self::SEED_PREFIX, authority.as_ref(), &bump_vec]
    // }

    pub fn on_claim(&mut self, time_now: u64) -> Result<u64> {
        // Determine unclaimed rewards
        let mut rewards: u64 = self.get_unclaimed_rewards(time_now);
        // Mature existing slots and collect rewards
        rewards += tokens_to_vouchers(self.mature_slots(time_now));

        // TODO: Fetch the released count if the epoch has lapsed and release
        // Update the reward timestamp
        self.time_rewarded = time_now;
        // Return outstanding rewards
        Ok(rewards)
    }

    pub fn on_deposit(&mut self, amount: u64, time_now: u64) -> Result<u64> {
        require!(
            AccountStatus::not(AccountStatus::Suspended, self.status),
            MemberError::AccountSuspended
        );

        require!(
            self.slots.len() < Self::MAX_SLOTS,
            LockingError::MaxSlotsExceeded
        );
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
        // Grant entries and update locked amount counter
        self.total_amount += amount;
        rewards += tokens_to_vouchers(amount);
        // Update member tier
        self.tier = MemberTier::from_tier(MemberTier::get_tier(self.total_amount));
        // Return outstanding rewards
        Ok(rewards)
    }

    pub fn on_withdraw(&mut self, amount: u64, time_now: u64) -> Result<u64> {
        // Claim outstanding rewards
        let rewards: u64 = self.on_claim(time_now).unwrap();
        // Check sufficient token maturity
        require!(
            self.total_matured >= amount,
            TransferError::InsufficientBalance
        );
        // Create a pending withdrawal transaction
        let withdrawal = Transaction::Withdraw {
            amount: amount,
            time_released: time_now + MATURATION_PERIOD,
        };
        // Append the new locked slot if it can't be compress
        self.slots.push(withdrawal);
        // We shift balance from locked to liquidity
        self.total_matured -= amount;
        self.total_pending += amount;
        // Update member tier for new balance
        self.tier = MemberTier::from_tier(MemberTier::get_tier(self.total_amount));
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

    pub fn mature_slots(&mut self, time_now: u64) -> u64 {
        let mut matured_change: u64 = 0;
        let mut matured_rewards: u64 = 0;
        let mut released_change: u64 = 0;
        // Cleanup matured slots and capture amount
        self.slots.retain(|slot: &Transaction| {
            if slot.is_matured(time_now) {
                match slot {
                    Transaction::Deposit {
                        amount,
                        time_created,
                        time_matured,
                    } => {
                        // let epoch_length: u64 = time_matured - time_created;
                        // let outstanding_cycles: u64 = (time_now - time_matured) / epoch_length;
                        // matured_rewards += outstanding_cycles * amount;
                        matured_change += amount;
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

        msg!("Matured rewards {}", matured_rewards);
        msg!("Release {} tokens", released_change);
        msg!("Matured {} tokens", matured_change);
        // Update total matured balance
        self.total_matured += matured_change;
        matured_change
    }

    fn get_unclaimed_rewards(&self, time_now: u64) -> u64 {
        let total_epochs: u64 = self.get_unclaimed_epochs(time_now);
        tokens_to_vouchers(self.total_matured * total_epochs)
    }

    fn get_unclaimed_epochs(&self, time_now: u64) -> u64 {
        let time_elapsed: u64 = time_now - self.time_rewarded;
        return time_elapsed / MATURATION_PERIOD;
    }

    pub fn grant_rewards(&mut self, amount: u64) -> Result<()> {
        let status: AccountStatus = AccountStatus::from_u8(self.status);
        match status {
            AccountStatus::Excluded | AccountStatus::Suspended => {
                Err(LockingError::RewardsForbidden.into())
            }
            AccountStatus::Pending => {
                self.status = AccountStatus::Active.to_u8();
                self.total_entries += amount + Self::ACTIVATION_REWARD;
                msg!(
                    "Rewarded {} vouchers with {} activation reward",
                    amount,
                    Self::ACTIVATION_REWARD
                );
                Ok(())
            }
            AccountStatus::Active | AccountStatus::Paused => {
                self.total_entries += amount;
                msg!("Rewarded {} vouchers", amount);
                Ok(())
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.total_amount == 0
    }

    /**
     Derives the Program Account signature
     */
    // pub fn derive_pda_signature<'a>(&'a self, authority: &'a Pubkey, bump: u8) -> &[&[&[u8]]; 1] {
    //     let seeds = Self::seeds(authority, bump);
    //     [&seeds[..]]
    // }

    /** Updates the account status when current status is not `AccountStatus::Excluded` or `AccountStatus::Suspended` */
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
            + std::mem::size_of::<MemberAccount>()
            + ANCHOR_VECTOR_SIZE
            + (std::mem::size_of::<Transaction>() * total_slots)
    }
}

impl IsInitialized for MemberAccount {
    fn is_initialized(&self) -> bool {
        AccountStatus::not(AccountStatus::Pending, self.status)
    }
}

impl Default for MemberAccount {
    fn default() -> Self {
        MemberAccount {
            version: Self::LATEST_VERSION,
            status: AccountStatus::Pending.to_u8(),
            tier: MemberTier::from_tier(MemberTier::Pending),
            total_amount: 0,
            total_matured: 0,
            total_pending: 0,
            total_entries: 0,
            time_created: 0,
            time_rewarded: 0,
            slots: Vec::new(),
        }
    }
}
