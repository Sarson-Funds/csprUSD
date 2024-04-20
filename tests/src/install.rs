use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{Key, PublicKey, U256};

use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ACCOUNT_1_PUBLIC_KEY, ALLOWANCES_KEY, ARG_CURRENCY, BALANCES_KEY,
        BLACKLISTER, DECIMALS_KEY, IS_PAUSED, MASTER_MINTER, NAME_KEY, OWNER, PAUSER, SYMBOL_KEY,
        TOKEN_CURRENCY, TOKEN_DECIMALS, TOKEN_NAME, TOKEN_SYMBOL, TOTAL_SUPPLY_KEY,
    },
    installer_request_builders::{
        csprusd_check_balance_of, invert_csprusd_address, setup, TestContext,
    },
};

#[test]
fn should_have_queryable_properties() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    let name: String = builder.get_value(csprusd_token, NAME_KEY);
    assert_eq!(name, TOKEN_NAME);

    let symbol: String = builder.get_value(csprusd_token, SYMBOL_KEY);
    assert_eq!(symbol, TOKEN_SYMBOL);

    let currency: String = builder.get_value(csprusd_token, ARG_CURRENCY);
    assert_eq!(currency, TOKEN_CURRENCY);

    let decimals: u8 = builder.get_value(csprusd_token, DECIMALS_KEY);
    assert_eq!(decimals, TOKEN_DECIMALS);

    let total_supply: U256 = builder.get_value(csprusd_token, TOTAL_SUPPLY_KEY);
    assert_eq!(total_supply, U256::zero());

    let paused: bool = builder.get_value(csprusd_token, IS_PAUSED);
    assert!(!paused);

    let account_1_key: Key = Key::Account(*ACCOUNT_1_ADDR);

    let master_minter: Key = builder.get_value(csprusd_token, MASTER_MINTER);
    assert_eq!(master_minter, account_1_key);

    let blacklister: PublicKey = builder.get_value(csprusd_token, BLACKLISTER);
    assert_eq!(blacklister, ACCOUNT_1_PUBLIC_KEY.clone());

    let owner: Key = builder.get_value(csprusd_token, OWNER);
    assert_eq!(owner, account_1_key);

    let pauser: PublicKey = builder.get_value(csprusd_token, PAUSER);
    assert_eq!(pauser, ACCOUNT_1_PUBLIC_KEY.clone());

    let owner_balance = csprusd_check_balance_of(&mut builder, &csprusd_token, account_1_key);
    assert_eq!(owner_balance, total_supply);

    let contract_balance = csprusd_check_balance_of(
        &mut builder,
        &csprusd_token,
        Key::Hash(csprusd_token.value()),
    );
    assert_eq!(contract_balance, U256::zero());

    // Ensures that Account and Contract ownership is respected and we're not keying ownership under
    // the raw bytes regardless of variant.
    let inverted_owner_key = invert_csprusd_address(account_1_key);
    let inverted_owner_balance =
        csprusd_check_balance_of(&mut builder, &csprusd_token, inverted_owner_key);
    assert_eq!(inverted_owner_balance, U256::zero());
}

#[test]
fn should_not_store_balances_or_allowances_under_account_after_install() {
    let (builder, _contract_hash) = setup();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let named_keys = account.named_keys();
    assert!(!named_keys.contains_key(BALANCES_KEY), "{:?}", named_keys);
    assert!(!named_keys.contains_key(ALLOWANCES_KEY), "{:?}", named_keys);
}
