use std::convert::TryInto;

use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ACCOUNT_2_ADDR, ACCOUNT_2_PUBLIC_KEY, ALREADY_BLACKLISTED, AMOUNT,
        BLACKLIST, BLACKLISTED_ACCOUNT, BLACKLISTED_ADDRESSES_COUNT,
        CONFIGURE_MINTER_ENTRY_POINT_NAME, KEY, METHOD_MINT, MINTER, MINTER_ALLOWED, NEW,
        NON_BLACKLISTER, NOT_BLACKLISTED, RECIPIENT, TOKEN_OWNER_AMOUNT_1, UN_BLACKLIST,
        UPDATE_BLACKLISTER_ENTRY_POINT,
    },
    installer_request_builders::{csprusd_check_balance_of, setup, TestContext},
};
use casper_engine_test_support::{ExecuteRequestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR};

use casper_types::{
    account::AccountHash, bytesrepr::ToBytes, runtime_args, ApiError, ContractHash, Key,
    RuntimeArgs, U256,
};

use casper_execution_engine::{
    core::{engine_state::Error as CoreError, execution::Error as ExecError},
    storage::global_state::in_memory::InMemoryGlobalState,
};

#[test]
fn test_blacklisting_keeps_track_correctly() {
    let k1: Key = Key::Account(AccountHash(vec![1; 32].try_into().unwrap()));
    let k2: Key = Key::Hash(vec![2; 32].try_into().unwrap());
    let k3: Key = Key::Account(AccountHash(vec![3; 32].try_into().unwrap()));
    let k4: Key = Key::Hash(vec![4; 32].try_into().unwrap());
    let k5: Key = Key::Account(AccountHash(vec![5; 32].try_into().unwrap()));
    let k6: Key = Key::Account(AccountHash(vec![6; 32].try_into().unwrap()));
    let k7: Key = Key::Hash(vec![7; 32].try_into().unwrap());
    let k8: Key = Key::Hash(vec![8; 32].try_into().unwrap());
    let k9: Key = Key::Account(AccountHash(vec![9; 32].try_into().unwrap()));

    // install contract
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    // write a long sequence of blacklisting and whitelisting
    whitelist(csprusd_token, k9, &mut builder, true); // -
    blacklist(csprusd_token, k1, &mut builder, false); // 1
    blacklist(csprusd_token, k1, &mut builder, true); // 1
    blacklist(csprusd_token, k1, &mut builder, true); // 1
    blacklist(csprusd_token, k2, &mut builder, false); // 1 2
    whitelist(csprusd_token, k2, &mut builder, false); // 1
    whitelist(csprusd_token, k1, &mut builder, false); // -
    whitelist(csprusd_token, k1, &mut builder, true); // -
    whitelist(csprusd_token, k1, &mut builder, true); // -
    blacklist(csprusd_token, k1, &mut builder, false); // 1
    blacklist(csprusd_token, k2, &mut builder, false); // 1 2
    blacklist(csprusd_token, k3, &mut builder, false); // 1 2 3
    blacklist(csprusd_token, k4, &mut builder, false); // 1 2 3 4
    blacklist(csprusd_token, k4, &mut builder, true); // 1 2 3 4
    blacklist(csprusd_token, k5, &mut builder, false); // 1 2 3 4 5
    whitelist(csprusd_token, k1, &mut builder, false); // 2 3 4 5
    whitelist(csprusd_token, k1, &mut builder, true); // 2 3 4 5
    whitelist(csprusd_token, k5, &mut builder, false); // 2 3 4
    blacklist(csprusd_token, k4, &mut builder, true); // 2 3 4
    blacklist(csprusd_token, k5, &mut builder, false); // 2 3 4 5
    blacklist(csprusd_token, k6, &mut builder, false); // 2 3 4 5 6
    blacklist(csprusd_token, k7, &mut builder, false); // 2 3 4 5 6 7
    blacklist(csprusd_token, k8, &mut builder, false); // 2 3 4 5 6 7 8
    whitelist(csprusd_token, k1, &mut builder, true); // 2 3 4 5 6 7 8
    whitelist(csprusd_token, k8, &mut builder, false); // 2 3 4 5 6 7
    whitelist(csprusd_token, k9, &mut builder, true); // 2 3 4 5 6 7

    let keys = get_blacklist(&mut builder, csprusd_token);

    assert_eq!(keys.len(), 6);
    assert!(keys.contains(&hex::encode(k2.to_bytes().unwrap())));
    assert!(keys.contains(&hex::encode(k3.to_bytes().unwrap())));
    assert!(keys.contains(&hex::encode(k4.to_bytes().unwrap())));
    assert!(keys.contains(&hex::encode(k5.to_bytes().unwrap())));
    assert!(keys.contains(&hex::encode(k6.to_bytes().unwrap())));
    assert!(keys.contains(&hex::encode(k7.to_bytes().unwrap())));
}

fn blacklist(
    csprusd_token: ContractHash,
    key_to_blacklist: Key,
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    should_fail: bool,
) {
    let blacklist_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        BLACKLIST,
        runtime_args! {KEY => key_to_blacklist},
    )
    .build();

    if should_fail {
        builder.exec(blacklist_request).commit();

        let error = builder.get_error().expect("should have error");
        assert!(
            matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ALREADY_BLACKLISTED),
            "{:?}",
            error
        );
    } else {
        builder.exec(blacklist_request).expect_success().commit();
    }
}

fn whitelist(
    csprusd_token: ContractHash,
    key_to_whitelist: Key,
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    should_fail: bool,
) {
    let un_blacklist_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        UN_BLACKLIST,
        runtime_args! {KEY => key_to_whitelist},
    )
    .build();

    if should_fail {
        builder.exec(un_blacklist_request).commit();

        let error = builder.get_error().expect("should have error");
        assert!(
            matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == NOT_BLACKLISTED),
            "{:?}",
            error
        );
    } else {
        builder.exec(un_blacklist_request).expect_success().commit();
    }
}

#[test]
fn test_blacklisting_prevents_minting() {
    let account_1_key: Key = Key::Account(*ACCOUNT_1_ADDR);
    let account_2_key: Key = Key::Account(*ACCOUNT_2_ADDR); // non-blacklister
    let account_3_key: Key = Key::Account(*DEFAULT_ACCOUNT_ADDR); // this account will be (un)blacklisted

    // install contract
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    // configure a minter: account_2_key
    let configure_minter_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        CONFIGURE_MINTER_ENTRY_POINT_NAME,
        runtime_args! {MINTER => account_2_key, MINTER_ALLOWED => U256::MAX},
    )
    .build();
    builder
        .exec(configure_minter_request)
        .expect_success()
        .commit();

    // prove that operations work for account account_3_key
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => account_3_key, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();

    assert_eq!(
        csprusd_check_balance_of(&mut builder, &csprusd_token, account_3_key),
        U256::from(TOKEN_OWNER_AMOUNT_1)
    );

    // non-blacklister tries to blacklist account_3_key
    let blacklist_account3_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        BLACKLIST,
        runtime_args! {KEY => account_3_key},
    )
    .build();
    builder.exec(blacklist_account3_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == NON_BLACKLISTER),
        "{:?}",
        error
    );

    // blacklister blacklists account_3_key
    let blacklist_account3_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        BLACKLIST,
        runtime_args! {KEY => account_3_key},
    )
    .build();
    builder
        .exec(blacklist_account3_request)
        .expect_success()
        .commit();

    // minting to account_3_key will fail because target is blacklisted
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => account_3_key, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == BLACKLISTED_ACCOUNT),
        "{:?}",
        error
    );

    // non-blacklister tries to un_blacklist -> fails
    let un_blacklist_account3_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        UN_BLACKLIST,
        runtime_args! {KEY => account_3_key},
    )
    .build();
    builder.exec(un_blacklist_account3_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == NON_BLACKLISTER),
        "{:?}",
        error
    );

    // blacklister un_blacklists
    let un_blacklist_account3_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        UN_BLACKLIST,
        runtime_args! {KEY => account_3_key},
    )
    .build();
    builder
        .exec(un_blacklist_account3_request)
        .expect_success()
        .commit();

    // prove that operations work for account
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => account_3_key, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();

    assert_eq!(
        csprusd_check_balance_of(&mut builder, &csprusd_token, account_3_key),
        U256::from(2 * TOKEN_OWNER_AMOUNT_1)
    );

    // blacklist account 1
    let blacklist_account1_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        BLACKLIST,
        runtime_args! {KEY => account_1_key},
    )
    .build();
    builder
        .exec(blacklist_account1_request)
        .expect_success()
        .commit();

    // get blacklist from contract
    let blacklist: Vec<String> = get_blacklist(&mut builder, csprusd_token);

    assert_eq!(
        hex::decode(blacklist.get(0).unwrap()).unwrap(),
        account_1_key.to_bytes().unwrap()
    );

    // update blacklister
    let update_blacklister_to_acc_2 = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        UPDATE_BLACKLISTER_ENTRY_POINT,
        runtime_args! {NEW => ACCOUNT_2_PUBLIC_KEY.clone()},
    )
    .build();
    builder
        .exec(update_blacklister_to_acc_2)
        .expect_success()
        .commit();
}

fn get_blacklist(
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    contract: ContractHash,
) -> Vec<String> {
    let blacklisted_count: u32 = builder.get_value(contract, BLACKLISTED_ADDRESSES_COUNT);
    let dict_uref = *builder
        .query(None, contract.into(), &[])
        .expect("must have nft contract")
        .as_contract()
        .expect("must convert contract")
        .named_keys()
        .get("index_to_blacklisted_addr")
        .expect("must have key")
        .as_uref()
        .expect("must convert to seed uref");

    let mut res = Vec::new();

    for i in 1..blacklisted_count {
        let a = builder
            .query_dictionary_item(None, dict_uref, &i.to_string())
            .unwrap();
        let b = a.as_cl_value().unwrap();
        let c: String = b.clone().into_t().unwrap();
        res.push(c);
    }
    res
}
