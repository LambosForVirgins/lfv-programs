module lfv::membership_system;

use lfv::lfv::LFV;
use sui::coin::{Self, Coin};

const BRONZE_THRESHOLD: u64 = 1000;
const SILVER_THRESHOLD: u64 = 5000;
const GOLD_THRESHOLD: u64 = 10000;

/// Membership tiers
public enum Tier {
    Bronze,
    Silver,
    Gold,
    None,
}

/// Represents a user's stake
public struct Stake has key, store {
    id: UID,
    owner: address,
    amount: u64,
}

/// Global state to keep track of all stakes
public struct StakePool has key {
    id: UID,
    stakes: vector<Stake>,
    total_stake: u64,
    coins: Coin<LFV>,
}

/// Initialize the stake pool (once)
public fun new(ctx: &mut TxContext) {
    let pool = StakePool {
        id: object::new(ctx),
        stakes: vector::empty(),
        total_stake: 0,
        coins: coin::zero<LFV>(ctx),
    };

    transfer::share_object(pool)
}

/// Stake tokens to the pool
public fun deposit_bond(pool: &mut StakePool, tokens: Coin<LFV>, ctx: &mut TxContext) {
    let amount = coin::value(&tokens);
    assert!(amount > 0, 0);

    let owner = tx_context::sender(ctx);

    vector::push_back(
        &mut pool.stakes,
        Stake {
            id: object::new(ctx),
            owner,
            amount,
        },
    );

    pool.total_stake = pool.total_stake + amount;

    // Merge the tokens into the pool's coin balance
    coin::join(&mut pool.coins, tokens);
}

/// Determine user's membership tier based on their stake
public fun get_membership_tier(amount: u64): Tier {
    if (amount >= GOLD_THRESHOLD) {
        Tier::Gold
    } else if (amount >= SILVER_THRESHOLD) {
        Tier::Silver
    } else if (amount >= BRONZE_THRESHOLD) {
        Tier::Bronze
    } else {
        Tier::None
    }
}
