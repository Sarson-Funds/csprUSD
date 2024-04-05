use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    MINIMUM_ACCOUNT_CREATION_BALANCE, PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_types::{
    runtime_args, system::mint, ApiError, ContractHash, Key, PublicKey, RuntimeArgs, U256,
};

use crate::utility::constants::{
    ACCOUNT_1_ADDR, ACCOUNT_1_PUBLIC_KEY, ACCOUNT_2_ADDR, AMOUNT, ARG_CURRENCY, ARG_DECIMALS,
    ARG_MASTER_MINTER, ARG_NAME, ARG_SYMBOL, BLACKLISTER, CONFIGURE_MINTER_ENTRY_POINT_NAME,
    CONTRACT_HASH, ERROR_EXCEEDS_MINT_ALLOWANCE, METHOD_MINT, MINTER, MINTER_ALLOWED, OWNER,
    PAUSER, RECIPIENT, TOKEN_CURRENCY, TOKEN_DECIMALS, TOKEN_NAME, TOKEN_SYMBOL,
};

use casper_execution_engine::core::{
    engine_state::Error as CoreError, execution::Error as ExecError,
};

/// Test assuring us that contract upgrades work as expected
/// Expectations after upgrade:
///     1. Old version of contract is inaccessible
///     2. New contract has same named-keys
///     3. New contract has access to the data left by previous contract
///     4. What is named keys set shrinks/grows? How would this function?
///
/// contract v0: blacklister is a Key::Account(AccountHash)
/// contract v1: blacklister is a PublicKey
#[test]
fn test_contract_upgrades() {
    let account_1_key: Key = Key::Account(*ACCOUNT_1_ADDR);
    let _account_2_key: Key = Key::Account(*ACCOUNT_2_ADDR);
    // install v0 of contract
    let mut builder = InMemoryWasmTestBuilder::default();
    builder
        .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
        .commit();

    // Install the first version of the contract
    let contract_v0_file: &str = "./../contract_versions/v0/csprusd_v0.wasm"; // searches given file in: tests/wasm/
    let install_request_1 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        contract_v0_file,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_SYMBOL => TOKEN_SYMBOL,
            ARG_CURRENCY => TOKEN_CURRENCY,
            ARG_DECIMALS => TOKEN_DECIMALS,
            ARG_MASTER_MINTER => account_1_key,
            PAUSER => account_1_key,
            BLACKLISTER => account_1_key,
            OWNER => account_1_key,
        },
    )
    .build();

    builder.exec(install_request_1).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");
    let csprusd_token = account
        .named_keys()
        .get(CONTRACT_HASH)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let id: Option<u64> = None;
    let transfer_1_args = runtime_args! {
        mint::ARG_TARGET => *ACCOUNT_1_ADDR,
        mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
        mint::ARG_ID => id,
    };
    let transfer_request_1 =
        ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_1_args).build();
    builder.exec(transfer_request_1).expect_success().commit();

    // prove that blacklister is a Key::Account
    let blacklister: Key = builder.get_value(csprusd_token, BLACKLISTER);

    assert_eq!(blacklister, account_1_key);

    // do some side effects: name a minter, mint some money, blacklist someone, etc
    // configure minter
    let configure_minter_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        CONFIGURE_MINTER_ENTRY_POINT_NAME,
        runtime_args! {MINTER => account_1_key, MINTER_ALLOWED => U256::from(10)},
    )
    .build();
    builder
        .exec(configure_minter_request)
        .expect_success()
        .commit();

    let mint1_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => account_1_key, AMOUNT => U256::from(5)},
    )
    .build();
    builder.exec(mint1_request).expect_success().commit();

    // upgrade contract to v1
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    let contract_v1_file: &str = "./../contract_versions/v1/csprusd.wasm"; // searches given file in: tests/wasm/
                                                                           // let contract_v0_file: &str = "./../contract_versions/v0/csprusd_v0.wasm"; // searches given
                                                                           // file in: tests/wasm/

    let contract_v2_installation_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        contract_v1_file,
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_SYMBOL => TOKEN_SYMBOL,
            ARG_CURRENCY => TOKEN_CURRENCY,
            ARG_DECIMALS => TOKEN_DECIMALS,
            ARG_MASTER_MINTER => account_1_key,
            PAUSER => account_1_key,
            BLACKLISTER => ACCOUNT_1_PUBLIC_KEY.clone(),
            OWNER => account_1_key,
        },
    )
    .build();

    builder
        .exec(contract_v2_installation_request)
        .expect_success()
        .commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");
    let csprusd_token1 = account
        .named_keys()
        .get(CONTRACT_HASH)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let blacklister: Key = builder.get_value(csprusd_token1, BLACKLISTER);

    // proof that contract was updated: there is data under "random_key"
    let random_v1_data: PublicKey = builder.get_value(csprusd_token1, "random_key");
    assert_eq!(random_v1_data, ACCOUNT_1_PUBLIC_KEY.clone());

    // proof that you can't change a named key if you also change its type
    assert_eq!(blacklister, account_1_key);

    // prove that new version still has access to the same data: minting allowance changed, etc
    //  do this by methods, not only by just simply querying global state e.g.: who is owner, etc?
    //   e.g.: try minting some money which exceeds the minting allowance left from operations in v0
    // let minter_allowed = builder.query(
    //     None,
    //     Key::Hash(csprusd_token1.to_bytes().unwrap().try_into().unwrap()),
    //     &["minter_allowed".to_string()],
    // );
    // println!("minter allowed uref: {:?}", minter_allowed);

    let mint1_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => account_1_key, AMOUNT => U256::from(6)},
    )
    .build();
    builder.exec(mint1_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_EXCEEDS_MINT_ALLOWANCE),
        "{:?}",
        error
    );

    // TODO: disable old version and prove that it no longer works
}
