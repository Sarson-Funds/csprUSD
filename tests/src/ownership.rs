use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_PUBLIC_KEY};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs};

use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ACCOUNT_2_ADDR, METHOD_TRANSFER_OWNERSHIP, METHOD_UPDATE_PAUSER, NEW,
        NOT_OWNER, OWNER,
    },
    installer_request_builders::{setup, TestContext},
};

use casper_execution_engine::core::{
    engine_state::Error as CoreError, execution::Error as ExecError,
};

#[test]
fn test_ownership() {
    let account_1_key: Key = Key::Account(*ACCOUNT_1_ADDR); // owner, master-minter, blacklister
    let account_2_key: Key = Key::Account(*ACCOUNT_2_ADDR); // non-owner

    // install contract
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    // assert owner
    let owner: Key = builder.get_value(csprusd_token, OWNER);
    assert_eq!(owner, account_1_key);

    // non-owner tries to do some owner method -> fails
    let update_pauser_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        METHOD_UPDATE_PAUSER,
        runtime_args! {NEW => DEFAULT_ACCOUNT_PUBLIC_KEY.clone()},
    )
    .build();

    builder.exec(update_pauser_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == NOT_OWNER),
        "{:?}",
        error
    );

    // owner transfers ownership to non-owner account above
    let transfer_ownership_to_acc_2 = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_TRANSFER_OWNERSHIP,
        runtime_args! {NEW => account_2_key},
    )
    .build();
    builder
        .exec(transfer_ownership_to_acc_2)
        .expect_success()
        .commit();

    // prove that not this new owner can perform onlyOwner() methods
    let update_pauser_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        METHOD_UPDATE_PAUSER,
        runtime_args! {NEW => DEFAULT_ACCOUNT_PUBLIC_KEY.clone()},
    )
    .build();

    builder
        .exec(update_pauser_request)
        .expect_success()
        .commit();
}
