use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ACCOUNT_2_ADDR, CONFIGURE_MINTER_ENTRY_POINT_NAME, MASTER_MINTER, MINTER,
        MINTER_ALLOWED, NEW, NOT_MASTER_MINTER, UPDATE_MASTER_MINTER_ENTRY_POINT_NAME,
    },
    installer_request_builders::{setup, TestContext},
};

use casper_execution_engine::core::{
    engine_state::Error as CoreError, execution::Error as ExecError,
};

#[test]
fn test_master_minter() {
    let account_1_key: Key = Key::Account(*ACCOUNT_1_ADDR); // owner, master-minter, blacklister
    let account_2_key: Key = Key::Account(*ACCOUNT_2_ADDR); // non-master-minter
    let account_3_key: Key = Key::Account(*DEFAULT_ACCOUNT_ADDR); // this account will be configured as minter

    // install contract
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    // assert master minter
    let master_minter: Key = builder.get_value(csprusd_token, MASTER_MINTER);
    assert_eq!(master_minter, account_1_key);

    // non-master minter tries to perform master minter operation
    let configure_minter_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        CONFIGURE_MINTER_ENTRY_POINT_NAME,
        runtime_args! {MINTER => account_3_key, MINTER_ALLOWED => U256::from(44)},
    )
    .build();
    builder.exec(configure_minter_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == NOT_MASTER_MINTER),
        "{:?}",
        error
    );

    // master-minter tries to perform master minter operation
    let configure_minter_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        CONFIGURE_MINTER_ENTRY_POINT_NAME,
        runtime_args! {MINTER => account_3_key, MINTER_ALLOWED => U256::from(44)},
    )
    .build();
    builder
        .exec(configure_minter_request)
        .expect_success()
        .commit();

    // transfer master-minter rights to another account
    let update_master_minter = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        UPDATE_MASTER_MINTER_ENTRY_POINT_NAME,
        runtime_args! {NEW => account_2_key},
    )
    .build();
    builder.exec(update_master_minter).expect_success().commit();

    // now new master minter account can do stuff
    let configure_minter_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        CONFIGURE_MINTER_ENTRY_POINT_NAME,
        runtime_args! {MINTER => account_3_key, MINTER_ALLOWED => U256::from(444)},
    )
    .build();
    builder
        .exec(configure_minter_request)
        .expect_success()
        .commit();
}
