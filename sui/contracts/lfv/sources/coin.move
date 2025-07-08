module lfv::lfv;

use std::string;
use sui::coin::{Self, TreasuryCap};
use sui::url;

public struct LFV has drop {}

const INITIAL_SUPPLY: u64 = 1_000_000_000; // 1 billion units

fun init(witness: LFV, ctx: &mut TxContext) {
    let icon_url = string::utf8(
        b"https://peach-metadata.s3.ap-southeast-2.amazonaws.com/peach%403x.png",
    );

    let (mut treasury, metadata) = coin::create_currency(
        witness,
        9,
        b"LFV",
        b"LFV",
        b"LFV is a token",
        option::some(url::new_unsafe(icon_url.to_ascii())),
        ctx,
    );

    // Mint initial supply to treasury
    let initial_coins = coin::mint(&mut treasury, INITIAL_SUPPLY, ctx);
    transfer::public_transfer(initial_coins, tx_context::sender(ctx));
    transfer::public_transfer(treasury, tx_context::sender(ctx));
    transfer::public_transfer(metadata, tx_context::sender(ctx));
}

#[test_only]
public fun init_for_testing(ctx: &mut TxContext) {
    init(LFV {}, ctx);
}

public entry fun mint(
    treasury: &mut TreasuryCap<LFV>,
    amount: u64,
    recipient: address,
    ctx: &mut TxContext,
) {
    let coin = coin::mint(treasury, amount, ctx);
    transfer::public_transfer(coin, recipient)
}

#[test_only]
public fun get_initial_supply(): u64 {
    INITIAL_SUPPLY
}
