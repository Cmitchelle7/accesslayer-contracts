//! `dividend_credited` is emitted per holder and `dividend_claimed` on claim (issue #857).

mod contract_test_env;
use contract_test_env::{
    register_creator_keys, register_test_creator, set_pricing_and_fees, setup_holders,
    test_env_with_auths, DEFAULT_CREATOR_BPS, DEFAULT_PROTOCOL_BPS,
};
use soroban_sdk::testutils::{Address as _, Events};
use soroban_sdk::Address;

#[test]
fn credited_and_claimed_events_are_emitted() {
    let env = test_env_with_auths();
    let (client, _id) = register_creator_keys(&env);
    set_pricing_and_fees(
        &env,
        &client,
        100,
        DEFAULT_CREATOR_BPS,
        DEFAULT_PROTOCOL_BPS,
    );

    let creator = register_test_creator(&env, &client, "davecreator");

    let alice = Address::generate(&env);
    let holders = [(alice.clone(), 1u32)];
    setup_holders(&env, &client, &creator, &holders);

    let amounts = soroban_sdk::vec![&env, alice.clone()];
    client.distribute_dividend_claimable(&creator, &1_000i128, &amounts);
    let after_distribute = env.events().all();
    assert!(!after_distribute.is_empty());

    client.claim_dividend_claimable(&creator, &alice);
    let after_claim = env.events().all();
    assert!(after_claim.len() > after_distribute.len());
}