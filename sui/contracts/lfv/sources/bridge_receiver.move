module lfv::bridge_receiver;

use lfv::entry::{Self, ENTRY};
use sui::address;
use sui::clock::Clock;
use sui::coin::TreasuryCap;
use sui::table::{Self, Table};
use wormhole::external_address;
use wormhole::state::State;
use wormhole::vaa::{Self, VAA};

/// Error codes
const EInvalidEmitter: u64 = 1;
const EInvalidPayload: u64 = 2;
const EAlreadyProcessed: u64 = 3;
const EInvalidChain: u64 = 4;

/// The Solana chain ID in Wormhole
const SOLANA_CHAIN_ID: u16 = 42; //1;

/// The Solana LFV program address that emits burn events
const SOLANA_LFV_PROGRAM: vector<u8> =
    x"7ce62138044ad06a9438874eb9566b73d6995eb6bc85c9af98d10b4514c4864f"; // TODO: Replace with actual program address

/// Tracks processed VAAs to prevent double-minting
public struct ProcessedVAAs has key, store {
    id: UID,
    processed: Table<vector<u8>, bool>,
}

/// Initialize the ProcessedVAAs table
fun init(ctx: &mut TxContext) {
    let processed_vaas: ProcessedVAAs = ProcessedVAAs {
        id: object::new(ctx),
        processed: table::new(ctx),
    };
    transfer::share_object(processed_vaas);
}

/// Receives a Wormhole VAA and mints an ENTRY token to the specified Sui address.
/// Only processes VAAs from the Solana LFV burn event.
///
/// # Parameters
/// - `wormhole_state`: reference to the shared Wormhole State object
/// - `processed_vaas`: reference to the shared ProcessedVAAs object
/// - `treasury_cap`: mutable reference to the ENTRY TreasuryCap
/// - `clock`: reference to the Sui Clock resource for verifying guardians are active
/// - `vaa_bytes`: the BCS-encoded VAA submitted by the off-chain relayer
/// - `ctx`: transaction context for minting
public entry fun receive_entry_token(
    wormhole_state: &State,
    // processed_vaas: &mut ProcessedVAAs,
    treasury_cap: &mut TreasuryCap<ENTRY>,
    clock: &Clock,
    vaa_bytes: vector<u8>,
    ctx: &mut TxContext,
) {
    // Parse and verify the VAA signatures against the guardian set
    let verified_vaa: VAA = vaa::parse_and_verify(wormhole_state, vaa_bytes, clock);

    // Verify the VAA is from Solana
    assert!(vaa::emitter_chain(&verified_vaa) == SOLANA_CHAIN_ID, EInvalidChain);

    // Verify the VAA is from the Solana LFV program
    let emitter_address = vaa::emitter_address(&verified_vaa);
    let emitter_bytes = external_address::to_bytes(emitter_address);
    let expected_emitter_bytes = SOLANA_LFV_PROGRAM;
    let mut is_valid_emitter = true;
    let len = vector::length<u8>(&emitter_bytes);
    let mut idx = 0;
    while (idx < len) {
        if (
            *vector::borrow<u8>(&emitter_bytes, idx) != *vector::borrow<u8>(&expected_emitter_bytes, idx)
        ) {
            is_valid_emitter = false;
            break
        };
        idx = idx + 1;
    };
    std::debug::print(&is_valid_emitter);
    assert!(is_valid_emitter, EInvalidEmitter);

    // // Get the VAA sequence number for finality check
    // let sequence = vaa::sequence(&verified_vaa);
    // let mut seq_bytes: vector<u8> = vector::empty<u8>();
    // let mut j: u64 = 0;
    // let mut seq = sequence;
    // while (j < 8) {
    //     vector::push_back<u8>(&mut seq_bytes, (seq & 0xFF) as u8);
    //     seq = seq >> 8;
    //     j = j + 1;
    // };

    // // // Check if this VAA has already been processed
    // // assert!(
    // //     !table::contains<vector<u8>, bool>(&processed_vaas.processed, seq_bytes),
    // //     EAlreadyProcessed,
    // // );

    // Access the payload field
    let payload: vector<u8> = vaa::take_payload(verified_vaa);
    std::debug::print(&payload);

    // // The payload should contain:
    // // 1. The Sui recipient address (32 bytes)
    // assert!(vector::length<u8>(&payload) >= 40, EInvalidPayload);

    // // Extract the Sui recipient address (first 32 bytes)
    // let mut addr_bytes: vector<u8> = vector::empty<u8>();
    // let mut k: u64 = 0;
    // while (k < 32) {
    //     vector::push_back<u8>(&mut addr_bytes, *vector::borrow<u8>(&payload, k));
    //     k = k + 1;
    // };
    // // Convert to address
    // let sui_recipient = address::from_bytes(addr_bytes);

    // // Mark this VAA as processed
    // // table::add<vector<u8>, bool>(&mut processed_vaas.processed, seq_bytes, true);

    // // Mint ENTRY tokens to the recipient
    // entry::reward_bridged_trade(treasury_cap, sui_recipient, ctx);
}

/// Clean up a processed VAA from the table (only callable by the package owner)
public entry fun cleanup_processed_vaa(
    processed_vaas: &mut ProcessedVAAs,
    vaa_key: vector<u8>,
    _ctx: &mut TxContext,
) {
    assert!(table::contains<vector<u8>, bool>(&processed_vaas.processed, vaa_key), 0);
    table::remove<vector<u8>, bool>(&mut processed_vaas.processed, vaa_key);
}

#[test_only]
public fun init_for_testing(ctx: &mut TxContext) {
    let processed_vaas: ProcessedVAAs = ProcessedVAAs {
        id: object::new(ctx),
        processed: table::new(ctx),
    };
    transfer::share_object(processed_vaas);
}
