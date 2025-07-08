/// This Closed Loop Loyalty Token `$ENTRY` is sent to subscribed
/// members as a reward for their bound loyalty. The `$ENTRY` token
/// can be used to enter into an open `Draw` of a `Giveaway`.
///
/// Actions:
/// - reward_member - reward a member with `$ENTRY` tokens
/// - redeem_entry - redeem the `$ENTRY` token to enter into a `Draw`
module lfv::entry;

use sui::coin::{Self, TreasuryCap};
use sui::token;

public struct AuthorCapability has key { id: UID }

// The one-time-witness (OTW) entry token
public struct ENTRY has drop {}

// Rule requirement for rewarding member loyalty.
public struct LoyaltyReward has drop {}

public struct Ticket has key, store {
    id: UID,
}

/// Create a new `$ENTRY` token with `LoyaltyReward` policy to allow
/// holders to redeem for entries into a `Draw`.
fun init(otw: ENTRY, ctx: &mut TxContext) {
    let (treasury_cap, coin_metadata) = coin::create_currency(
        otw,
        0,
        b"ENTRY",
        b"Giveaway Entry",
        b"Member Loyalty Tokens",
        option::none(),
        ctx,
    );

    let (mut policy, policy_cap) = token::new_policy(&treasury_cap, ctx);

    // Constrain the spend within this ecosystem
    token::add_rule_for_action<ENTRY, LoyaltyReward>(
        &mut policy,
        &policy_cap,
        token::spend_action(),
        ctx,
    );

    token::share_policy(policy);

    transfer::public_freeze_object(coin_metadata);
    transfer::public_transfer(policy_cap, tx_context::sender(ctx));
    transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
}

#[test_only]
public fun init_for_testing(ctx: &mut TxContext) {
    init(ENTRY {}, ctx);
}

public fun reward_member(
    cap: &mut TreasuryCap<ENTRY>,
    amount: u64,
    recipient: address,
    ctx: &mut TxContext,
) {
    let token = token::mint(cap, amount, ctx);
    let request = token::transfer(token, recipient, ctx);

    token::confirm_with_treasury_cap(cap, request, ctx);
}

public fun reward_bridged_trade(
    cap: &mut TreasuryCap<ENTRY>,
    recipient: address,
    ctx: &mut TxContext,
) {
    let token = token::mint(cap, 1, ctx);
    let request = token::transfer(token, recipient, ctx);

    token::confirm_with_treasury_cap(cap, request, ctx);
}

/**
 * Redeem the ENTRY token to enter into the giveaway.
 **/
public fun redeem_entry(
    _cap: &mut TreasuryCap<ENTRY>,
    _amount: u64,
    _recipient: address,
    _ctx: &mut TxContext,
) {}
