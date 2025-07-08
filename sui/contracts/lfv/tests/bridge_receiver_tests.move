#[test_only]
module bridge::receiver_test;

use lfv::bridge_receiver::{Self, ProcessedVAAs};
use lfv::entry::{Self, ENTRY};
use sui::coin::TreasuryCap;
use sui::test_scenario;
use wormhole::wormhole_scenario::{
    guardians,
    person,
    return_clock,
    return_state,
    set_up_wormhole_with_guardians,
    take_clock,
    take_state
};

const ENotImplemented: u64 = 0;

const VAA_MESSAGE: vector<u8> =
    x"0100000000010049f9568dc65f0dc45064d0c5d9937df1bcf663efc29ee5ffd58a10ba8579d18f13126b378bd83efc1f982306cca73734c30bda6b89c9a4637b366765817dee930068516f2d0000000000017ce62138044ad06a9438874eb9566b73d6995eb6bc85c9af98d10b4514c4864f000000000000000000307831376535343262346366663062383064363137316335653036326230626330323961383865656664343663353639373034346539363864613766623565386365";

#[test]
fun test_receive_entry_token() {
    let caller = person();
    // let mut ctx = tx_context::dummy();
    let mut scenario_val = test_scenario::begin(caller);
    let scenario = &mut scenario_val;

    // Initialize Wormhole with 19 guardians.
    let wormhole_fee = 350;
    set_up_wormhole_with_guardians(scenario, wormhole_fee, guardians());

    // Initialize ENTRY token
    test_scenario::next_tx(scenario, caller);
    entry::init_for_testing(test_scenario::ctx(scenario));

    // Initialize ProcessedVAAs
    test_scenario::next_tx(scenario, caller);
    bridge_receiver::init_for_testing(test_scenario::ctx(scenario));
    std::debug::print(&b"here");

    // Get processed VAAs
    // let mut processed_vaas = test_scenario::take_shared<ProcessedVAAs>(scenario);

    // std::debug::print(&processed_vaas);

    // Get treasury cap for ENTRY token
    let mut treasury_cap = test_scenario::take_from_sender<TreasuryCap<ENTRY>>(scenario);

    // Get wormhole state and clock
    let worm_state = take_state(scenario);
    let the_clock = take_clock(scenario);

    // Call the bridge function
    bridge_receiver::receive_entry_token(
        &worm_state,
        // &mut processed_vaas,
        &mut treasury_cap,
        &the_clock,
        VAA_MESSAGE,
        test_scenario::ctx(scenario),
    );

    // Return treasury cap
    test_scenario::return_to_sender(scenario, treasury_cap);

    // Return processed VAAs
    // test_scenario::return_shared(processed_vaas);

    // Clean up.
    return_state(worm_state);
    return_clock(the_clock);
    test_scenario::end(scenario_val);
}

#[test, expected_failure(abort_code = ::bridge::receiver_test::ENotImplemented)]
fun test_bridge_receiver() {
    abort ENotImplemented
}

#[test, expected_failure(abort_code = ::bridge::receiver_test::ENotImplemented)]
fun test_bridge_receiver_fail() {
    abort ENotImplemented
}
