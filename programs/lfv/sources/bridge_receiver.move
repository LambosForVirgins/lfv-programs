module lfv::bridge_receiver;

use lfv::entry::reward_bridged_trade;
use std::debug;
use sui::bcs;
use sui::clock::Clock;
use wormhole::state::State;
use wormhole::vaa::{Self, VAA};

/// Receives a Wormhole VAA and mints an ENTRY token to the specified Sui address.
///
/// # Parameters
/// - `wormhole_state`: reference to the shared Wormhole State object
/// - `clock`: reference to the Sui Clock resource for verifying guardians are active
/// - `vaa_bytes`: the BCS-encoded VAA submitted by the off-chain relayer
/// - `ctx`: transaction context for minting
public entry fun receive_entry_token(
    wormhole_state: &State,
    clock: &Clock,
    vaa_bytes: vector<u8>,
    ctx: &mut TxContext,
) {
    // Parse and verify the VAA signatures against the guardian set
    let verified_vaa: VAA = vaa::parse_and_verify(wormhole_state, vaa_bytes, clock);

    // Access the payload field directly
    let payload: vector<u8> = vaa::take_payload(verified_vaa);
    debug::print(&payload);

    // Extract the first 32 bytes as the recipient address
    // let addr_bytes: vector<u8> = payload[0, 32];

    // // Deserialize to Move `address`
    // let sui_recipient: address = bcs::from_bytes(&addr_bytes);

    // // Mint a single ENTRY token to the verified recipient
    // entry_token::mint(sui_recipient, ctx);
}
