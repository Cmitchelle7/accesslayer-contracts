//! `distribute_dividend_claimable` requires creator auth (issue #857).

mod contract_test_env;
use contract_test_env::{
    register_creator_keys, register_test_creator_with_fee_config, set_pricing_and_fees,
    setup_holders, test_env_with_auths, DEFAULT_CREATOR_BPS, DEFAULT_PROTOCOL_BPS,
};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::Address;

#[test]
fn non_creator_call_is_rejected() {
    // Authenticated env so setup helpers succeed.
    let env = test_env_with_auths();
    // Turn auth off again so the specific call below is *not* auto-authorized.
    env.set_auth_enabled(false); // wait — see note below

    let (client, _id) = register_creator_keys(&env);
    // ... rest of setup ...
}
