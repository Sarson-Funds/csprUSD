use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ALLOWANCE_AMOUNT_1, ALLOWANCE_AMOUNT_2, AMOUNT, ARG_AMOUNT, ARG_OWNER,
        ARG_RECIPIENT, ARG_SPENDER, CONFIGURE_MINTER_ENTRY_POINT_NAME, DECREASE_ALLOWANCE,
        ERROR_INSUFFICIENT_ALLOWANCE, INCREASE_ALLOWANCE, METHOD_APPROVE, METHOD_MINT,
        METHOD_TRANSFER_FROM, MINTER, MINTER_ALLOWED, RECIPIENT, TOKEN_OWNER_AMOUNT_1,
    },
    installer_request_builders::{
        csprusd_check_allowance_of, csprusd_check_balance_of, make_csprusd_approve_request, setup,
        test_approve_for, TestContext,
    },
};
use casper_execution_engine::core::{
    engine_state::Error as CoreError, execution::Error as ExecError,
};

#[test]
fn should_approve_funds_contract_to_account() {
    let (mut builder, test_context) = setup();
    let TestContext {
        csprusd_test_contract_package,
        ..
    } = test_context;

    test_approve_for(
        &mut builder,
        &test_context,
        Key::Hash(csprusd_test_contract_package.value()),
        Key::Hash(csprusd_test_contract_package.value()),
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
}

#[test]
fn should_approve_funds_contract_to_contract() {
    let (mut builder, test_context) = setup();
    let TestContext {
        csprusd_test_contract_package,
        ..
    } = test_context;

    test_approve_for(
        &mut builder,
        &test_context,
        Key::Hash(csprusd_test_contract_package.value()),
        Key::Hash(csprusd_test_contract_package.value()),
        Key::Hash([42; 32]),
    );
}

#[test]
fn should_approve_funds_account_to_account() {
    let (mut builder, test_context) = setup();

    test_approve_for(
        &mut builder,
        &test_context,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        Key::Account(*ACCOUNT_1_ADDR),
    );
}

#[test]
fn should_approve_funds_account_to_contract() {
    let (mut builder, test_context) = setup();
    test_approve_for(
        &mut builder,
        &test_context,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
        Key::Hash([42; 32]),
    );
}

#[test]
fn should_not_transfer_from_without_enough_allowance() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
    let transfer_from_amount_1 = allowance_amount_1 + U256::one();

    let sender = *DEFAULT_ACCOUNT_ADDR;
    let owner = sender;
    let recipient = *ACCOUNT_1_ADDR;

    let minter_to_configure = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let configure_minter_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        CONFIGURE_MINTER_ENTRY_POINT_NAME,
        runtime_args! {MINTER => minter_to_configure, MINTER_ALLOWED => U256::MAX},
    )
    .build();
    builder
        .exec(configure_minter_request)
        .expect_success()
        .commit();

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => minter_to_configure, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();

    let csprusd_approve_args = runtime_args! {
        ARG_OWNER => Key::Account(owner),
        ARG_SPENDER => Key::Account(recipient),
        ARG_AMOUNT => allowance_amount_1,
    };
    let csprusd_transfer_from_args = runtime_args! {
        ARG_OWNER => Key::Account(owner),
        ARG_RECIPIENT => Key::Account(recipient),
        ARG_AMOUNT => transfer_from_amount_1,
    };

    let spender_allowance_before =
        csprusd_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(recipient));
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        sender,
        csprusd_token,
        METHOD_APPROVE,
        csprusd_approve_args,
    )
    .build();

    let transfer_from_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        sender,
        csprusd_token,
        METHOD_TRANSFER_FROM,
        csprusd_transfer_from_args,
    )
    .build();

    builder.exec(approve_request_1).expect_success().commit();

    let account_1_allowance_after =
        csprusd_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(recipient));
    assert_eq!(account_1_allowance_after, allowance_amount_1);

    builder.exec(transfer_from_request_1).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_INSUFFICIENT_ALLOWANCE),
        "{:?}",
        error
    );

    // assert that no state was updated by the failed transfer_from() above
    assert_eq!(
        csprusd_check_balance_of(&mut builder, &csprusd_token, minter_to_configure),
        U256::from(TOKEN_OWNER_AMOUNT_1)
    );
}

#[test]
fn test_decrease_allowance() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();
    let sender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let owner = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let spender = Key::Hash([42; 32]);
    let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
    let allowance_amount_2 = U256::from(ALLOWANCE_AMOUNT_2);

    let spender_allowance_before = csprusd_check_allowance_of(&mut builder, owner, spender);
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request =
        make_csprusd_approve_request(sender, &csprusd_token, spender, allowance_amount_1);
    let decrease_allowance_request = ExecuteRequestBuilder::contract_call_by_hash(
        sender.into_account().unwrap(),
        csprusd_token,
        DECREASE_ALLOWANCE,
        runtime_args! {
            ARG_SPENDER => spender,
            ARG_AMOUNT => allowance_amount_2,
        },
    )
    .build();
    let increase_allowance_request = ExecuteRequestBuilder::contract_call_by_hash(
        sender.into_account().unwrap(),
        csprusd_token,
        INCREASE_ALLOWANCE,
        runtime_args! {
            ARG_SPENDER => spender,
            ARG_AMOUNT => allowance_amount_1,
        },
    )
    .build();

    builder.exec(approve_request).expect_success().commit();

    let account_1_allowance_after = csprusd_check_allowance_of(&mut builder, owner, spender);

    assert_eq!(account_1_allowance_after, allowance_amount_1);

    builder
        .exec(decrease_allowance_request)
        .expect_success()
        .commit();

    let account_1_allowance_after_decrease =
        csprusd_check_allowance_of(&mut builder, owner, spender);

    assert_eq!(
        account_1_allowance_after_decrease,
        allowance_amount_1 - allowance_amount_2
    );

    builder
        .exec(increase_allowance_request)
        .expect_success()
        .commit();

    let account_1_allowance_after_increase =
        csprusd_check_allowance_of(&mut builder, owner, spender);

    assert_eq!(
        account_1_allowance_after_increase,
        (allowance_amount_1 * 2) - allowance_amount_2
    );
}
