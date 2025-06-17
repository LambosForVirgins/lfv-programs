#[test_only]
module bridge::receiver_test;

use lfv::bridge_receiver::receive_entry_token;
use sui::clock::{Self, Clock};
use sui::object::{Self, UID};
use sui::test_scenario;
use sui::tx_context;
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
    x"01000000000d009bafff633087a9587d9afb6d29bd74a3483b7a8d5619323a416fe9ca43b482cd5526fabe953157cfd42eea9ffa544babc0f3a025a8a6159217b96fc9ff586d560002c9367884940a43a1a4d86531ea33ccb41e52bd7d1679c106fdff756f3da2ca743f7c181fcf40d19151d0a8397335c1b71709279b6e4fa97b6e3de90824e841c801035a493b65bf12ab9b98aa4db3bfcb73df20ab854d8e5998a1552f3b3e57ea7cd3546187c62cd450d12d430cae0fb48124ae68034dae602fa3e2232b55257961f90104758e265101353923661f6df67cec3c38528ed1b68825099b5bb2ce3fb2e735c5073d90223bebd00cc10406a60413a6089b5fb9acee0a1b04a63a8d7db24c0bbc000587777306dd174e266c313f711e881086355b6ce66cf2bf1f5da58273a10be77813b5ffcafc1ba6b83645e326a7c1a3751496f279ba307a6cd554f2709c2f1eda0108ed23ba8264146c3e3cc0601c93260c25058bcdd25213a7834e51679afdc4b50104e3f3a3079ba45115e703096c7e0700354cd48348bbf686dcbc58be896c35a20009c2352cb46ef1d2ef9e185764650403aee87a1be071555b31cdcee0c346991da858defb8d5e164a293ce4377b54fc74b65e3acbdedcbb53c2bcc2688a0b5bd1c9010ae470b1573989f387f7c54a86325cc05978bbcbc13267e90e2fa2efb0e18bccb772252bd6d13ebf908f7f3f2caf20a45c17dec7168122a2535ea93d300fae7063000ba0e8770298d4e3567488f455455a33f1e723e1e629ba4f87928016aeaa5875561ec38bde5d934389dc657d80a927cd9d06a9d9c7ce910c98d77a576e3f31735c000eeeedc956cff4489ac55b52ca38233cdc11e88767e5cc82f664bd1d7c28dfb5a12d7d17620725aae08e499b021200919f42c50c05916cf425dcd6e59f24b4b233000f18d447c9608a076c066b30ee770910e3c133087d33e329ad0101f08f88d88e142623df87aa3842edcf34e10fd36271b49f7af73ff2a7bcf4a65a4306d59586f20111905fc99dc650d9b1b33c313e9b31dfdbc85ce57e9f31abc4841d5791a239f20e5f28e4e612db96aee2f49ae712f724466007aaf27309d0385005fe0264d33dd100127b46f2fbbbf12efb10c2e662b4449de404f6a408ad7f38c7ea40a46300930e9a3b1e02ce00b97e33fa8a87221c1fd9064ce966dc4772658b98f2ec1e28d13e7400000023280000000c002adeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef000000000000092a20416c6c20796f75722062617365206172652062656c6f6e6720746f207573";

#[test]
fun test_receive_entry_token() {
    let caller = person();
    let mut ctx = tx_context::dummy();
    let mut my_scenario = test_scenario::begin(caller);
    let scenario = &mut my_scenario;

    // Initialize Wormhole with 19 guardians.
    let wormhole_fee = 350;
    set_up_wormhole_with_guardians(scenario, wormhole_fee, guardians());

    // Prepare test to execute `parse_and_verify`.
    test_scenario::next_tx(scenario, caller);

    let worm_state = take_state(scenario);
    let the_clock = take_clock(scenario);

    // Call the bridge function
    receive_entry_token(&worm_state, &the_clock, VAA_MESSAGE, &mut ctx);

    // Optionally check that the token exists (if you store or log ENTRY)
    // This requires extending the module to emit or store minted tokens

    // Clean up.
    return_state(worm_state);
    return_clock(the_clock);
    test_scenario::end(my_scenario);
}

#[test]
fun test_bridge_receiver() {}

#[test, expected_failure(abort_code = ::bridge::receiver_test::ENotImplemented)]
fun test_bridge_receiver_fail() {
    abort ENotImplemented
}
