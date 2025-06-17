#[test_only]
module lfv::lfv_tests;

use lfv::lfv::{Self, LFV};
use sui::coin::{Self, Coin, TreasuryCap};
use sui::test_scenario::{Self as ts, Scenario};
use sui::test_utils::assert_eq;

#[test]
fun test_lfv_coin_initialization() {
    let owner = @0x123;
    let scenario = ts::begin(owner);

    // Initialize the LFV token
    {
        ts::next_tx(&mut scenario, owner);
        lfv::init(LFV {}, ts::ctx(&mut scenario));
    };

    // Verify treasury ownership and initial supply
    {
        ts::next_tx(&mut scenario, owner);
        let treasury = ts::take_from_sender<TreasuryCap<LFV>>(&scenario);
        let initial_coins = ts::take_from_sender<Coin<LFV>>(&scenario);
        assert_eq(coin::value(&initial_coins), lfv::get_initial_supply());
        ts::return_to_sender(&scenario, treasury);
        ts::return_to_sender(&scenario, initial_coins);
    };

    ts::end(scenario);
}

#[test]
fun test_lfv_coin_mint() {
    let owner = @0x123;
    let recipient = @0x456;
    let scenario = ts::begin(owner);

    // Initialize the LFV token
    {
        ts::next_tx(&mut scenario, owner);
        lfv::init(LFV {}, ts::ctx(&mut scenario));
    };

    // Test minting
    {
        ts::next_tx(&mut scenario, owner);
        let treasury = ts::take_from_sender<TreasuryCap<LFV>>(&scenario);
        lfv::mint(&mut treasury, 1000, recipient, ts::ctx(&mut scenario));
        ts::return_to_sender(&scenario, treasury);
    };

    // Verify recipient received the tokens
    {
        ts::next_tx(&mut scenario, recipient);
        let coin = ts::take_from_sender<Coin<LFV>>(&scenario);
        assert_eq(coin::value(&coin), 1000);
        ts::return_to_sender(&scenario, coin);
    };

    ts::end(scenario);
}
