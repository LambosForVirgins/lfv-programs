module entry_token::entry;

use sui::coin::{Self, TreasuryCap};
use sui::token::{Self};

public struct AuthorCapability has key { id: UID }

// The one-time-witness (OTW) entry token
public struct ENTRY has drop {}

// This is the Rule requirement for the `GiveawayTickets`.
public struct GiveawayTickets has drop {}

public struct Ticket has key, store {
    id: UID,
}

/**
 * Create a new `ENTRY` token, create a `TokenPolicy` for it and
 * allow everyone to spend `Token's if they were reward`ed with
 * a `GiveawayTickets`.
 **/
fun init(otw: ENTRY, ctx: &mut TxContext) {
    let (treasury_cap, coin_metadata) = coin::create_currency(
        otw,
        0, // no decimals
        b"ENTRY", // symbol
        b"Giveaway Entry", // name
        b"Giveaway Entry Member Reward Token", // description
        option::none(), // url
        ctx,
    );

    let (mut policy, policy_cap) = token::new_policy(&treasury_cap, ctx);

    // Constrain the spend by this giveaway ticket
    token::add_rule_for_action<ENTRY, GiveawayTickets>(
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

