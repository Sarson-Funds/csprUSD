use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ACCOUNT_1_PUBLIC_KEY, ACCOUNT_2_ADDR, ACCOUNT_2_PUBLIC_KEY, AMOUNT,
        BLACKLIST, BLACKLISTED_ACCOUNT, BLACKLISTED_LIST, CONFIGURE_MINTER_ENTRY_POINT_NAME, KEY,
        METHOD_MINT, MINTER, MINTER_ALLOWED, NEW, NON_BLACKLISTER, RECIPIENT, TOKEN_OWNER_AMOUNT_1,
        UN_BLACKLIST, UPDATE_BLACKLISTER_ENTRY_POINT,
    },
    installer_request_builders::{csprusd_check_balance_of, setup, TestContext},
};
use casper_engine_test_support::{
    ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_PUBLIC_KEY,
};

use casper_types::{bytesrepr::ToBytes, runtime_args, ApiError, Key, PublicKey, RuntimeArgs, U256};

use casper_execution_engine::core::{
    engine_state::Error as CoreError, execution::Error as ExecError,
};

#[test]
fn test_blacklisting() {
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
        runtime_args! {KEY => DEFAULT_ACCOUNT_PUBLIC_KEY.clone()},
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
        runtime_args! {KEY => DEFAULT_ACCOUNT_PUBLIC_KEY.clone()},
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

    // print list of blacklisted accounts
    let blacklist_account1_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        BLACKLIST,
        runtime_args! {KEY => ACCOUNT_1_PUBLIC_KEY.clone()},
    )
    .build();
    builder
        .exec(blacklist_account1_request)
        .expect_success()
        .commit();

    let blacklisted: Vec<PublicKey> = builder.get_value(csprusd_token, BLACKLISTED_LIST);
    for i in 0..blacklisted.len() {
        println!("key={:?}", *blacklisted.get(i).unwrap());
    }
    assert_eq!(
        blacklisted.get(0).unwrap().to_bytes(),
        ACCOUNT_1_PUBLIC_KEY.clone().to_bytes()
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
