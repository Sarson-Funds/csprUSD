use casper_engine_test_support::{ExecuteRequestBuilder, WasmTestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ACCOUNT_2_ADDR, ALLOWANCE_AMOUNT_1, AMOUNT, ARG_AMOUNT, ARG_OWNER,
        ARG_RECIPIENT, ARG_SPENDER, ARG_TOKEN_CONTRACT, CANNOT_TRANSFER_ZERO_AMOUNT,
        CONFIGURE_MINTER_ENTRY_POINT_NAME, ERROR_INSUFFICIENT_BALANCE, METHOD_APPROVE,
        METHOD_FROM_AS_STORED_CONTRACT, METHOD_MINT, METHOD_TRANSFER, METHOD_TRANSFER_FROM, MINTER,
        MINTER_ALLOWED, RECIPIENT, TOKEN_TOTAL_SUPPLY, TOTAL_SUPPLY_KEY, TRANSFER_AMOUNT_1,
    },
    installer_request_builders::{
        csprusd_check_allowance_of, csprusd_check_balance_of, make_csprusd_approve_request,
        make_csprusd_transfer_request, setup, test_csprusd_transfer, TestContext,
    },
};

use casper_execution_engine::{
    core::{engine_state::Error as CoreError, execution::Error as ExecError},
    storage::global_state::in_memory::InMemoryGlobalState,
};

#[test]
fn should_transfer_full_owned_amount() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
    let transfer_amount_1 = initial_supply;

    let transfer_1_sender = *DEFAULT_ACCOUNT_ADDR;
    let csprusd_transfer_1_args = runtime_args! {
        ARG_RECIPIENT => Key::Account(*ACCOUNT_1_ADDR),
        ARG_AMOUNT => transfer_amount_1,
    };

    mint_to_default_address_initial_amount(&mut builder, csprusd_token, initial_supply);

    let owner_balance_before = csprusd_check_balance_of(
        &mut builder,
        &csprusd_token,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(owner_balance_before, initial_supply);

    let account_1_balance_before =
        csprusd_check_balance_of(&mut builder, &csprusd_token, Key::Account(*ACCOUNT_1_ADDR));
    assert_eq!(account_1_balance_before, U256::zero());

    let token_transfer_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        transfer_1_sender,
        csprusd_token,
        METHOD_TRANSFER,
        csprusd_transfer_1_args,
    )
    .build();

    builder
        .exec(token_transfer_request_1)
        .expect_success()
        .commit();

    let account_1_balance_after =
        csprusd_check_balance_of(&mut builder, &csprusd_token, Key::Account(*ACCOUNT_1_ADDR));
    assert_eq!(account_1_balance_after, transfer_amount_1);

    let owner_balance_after = csprusd_check_balance_of(
        &mut builder,
        &csprusd_token,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(owner_balance_after, U256::zero());

    let total_supply: U256 = builder.get_value(csprusd_token, TOTAL_SUPPLY_KEY);
    assert_eq!(total_supply, initial_supply);
}

fn mint_to_default_address_initial_amount(
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    csprusd_token: ContractHash,
    initial_supply: U256,
) {
    let minter_to_configure = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let configure_minter_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        CONFIGURE_MINTER_ENTRY_POINT_NAME,
        runtime_args! {MINTER => minter_to_configure, MINTER_ALLOWED => initial_supply},
    )
    .build();
    builder
        .exec(configure_minter_request)
        .expect_success()
        .commit();

    let recipient = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => recipient, AMOUNT => initial_supply},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();
}

#[test]
fn should_not_transfer_more_than_owned_balance() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
    let transfer_amount = initial_supply + U256::one();

    mint_to_default_address_initial_amount(&mut builder, csprusd_token, initial_supply);

    let transfer_1_sender = *DEFAULT_ACCOUNT_ADDR;
    let transfer_1_recipient = *ACCOUNT_1_ADDR;

    let csprusd_transfer_1_args = runtime_args! {
        ARG_RECIPIENT => Key::Account(transfer_1_recipient),
        ARG_AMOUNT => transfer_amount,
    };

    let owner_balance_before = csprusd_check_balance_of(
        &mut builder,
        &csprusd_token,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );
    assert_eq!(owner_balance_before, initial_supply);
    assert!(transfer_amount > owner_balance_before);

    let account_1_balance_before =
        csprusd_check_balance_of(&mut builder, &csprusd_token, Key::Account(*ACCOUNT_1_ADDR));
    assert_eq!(account_1_balance_before, U256::zero());

    let token_transfer_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        transfer_1_sender,
        csprusd_token,
        METHOD_TRANSFER,
        csprusd_transfer_1_args,
    )
    .build();

    builder.exec(token_transfer_request_1).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_INSUFFICIENT_BALANCE),
        "{:?}",
        error
    );

    let account_1_balance_after = csprusd_check_balance_of(
        &mut builder,
        &csprusd_token,
        Key::Account(transfer_1_recipient),
    );
    assert_eq!(account_1_balance_after, account_1_balance_before);

    let owner_balance_after = csprusd_check_balance_of(
        &mut builder,
        &csprusd_token,
        Key::Account(transfer_1_sender),
    );
    assert_eq!(owner_balance_after, initial_supply);

    let total_supply: U256 = builder.get_value(csprusd_token, TOTAL_SUPPLY_KEY);
    assert_eq!(total_supply, initial_supply);
}

#[test]
fn should_transfer_from_from_account_to_account() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
    let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
    let transfer_from_amount_1 = allowance_amount_1;

    mint_to_default_address_initial_amount(&mut builder, csprusd_token, initial_supply);

    let owner = *DEFAULT_ACCOUNT_ADDR;
    let spender = *ACCOUNT_1_ADDR;

    let csprusd_approve_args = runtime_args! {
        ARG_OWNER => Key::Account(owner),
        ARG_SPENDER => Key::Account(spender),
        ARG_AMOUNT => allowance_amount_1,
    };
    let csprusd_transfer_from_args = runtime_args! {
        ARG_OWNER => Key::Account(owner),
        ARG_RECIPIENT => Key::Account(spender),
        ARG_AMOUNT => transfer_from_amount_1,
    };

    let spender_allowance_before =
        csprusd_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(spender));
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        owner,
        csprusd_token,
        METHOD_APPROVE,
        csprusd_approve_args,
    )
    .build();

    let transfer_from_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        spender,
        csprusd_token,
        METHOD_TRANSFER_FROM,
        csprusd_transfer_from_args,
    )
    .build();

    builder.exec(approve_request_1).expect_success().commit();

    let account_1_balance_before =
        csprusd_check_balance_of(&mut builder, &csprusd_token, Key::Account(owner));
    assert_eq!(account_1_balance_before, initial_supply);

    let account_1_allowance_before =
        csprusd_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(spender));
    assert_eq!(account_1_allowance_before, allowance_amount_1);

    builder
        .exec(transfer_from_request_1)
        .expect_success()
        .commit();

    let account_1_allowance_after =
        csprusd_check_allowance_of(&mut builder, Key::Account(owner), Key::Account(spender));
    assert_eq!(
        account_1_allowance_after,
        account_1_allowance_before - transfer_from_amount_1
    );

    let account_1_balance_after =
        csprusd_check_balance_of(&mut builder, &csprusd_token, Key::Account(owner));
    assert_eq!(
        account_1_balance_after,
        account_1_balance_before - transfer_from_amount_1
    );
}

#[test]
fn should_transfer_from_account_by_contract() {
    let (
        mut builder,
        TestContext {
            csprusd_token,
            csprusd_test_contract_package,
            ..
        },
    ) = setup();

    let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
    let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
    let transfer_from_amount_1 = allowance_amount_1;

    mint_to_default_address_initial_amount(&mut builder, csprusd_token, initial_supply);

    let owner = *DEFAULT_ACCOUNT_ADDR;

    let spender = Key::Hash(csprusd_test_contract_package.value());
    let recipient = Key::Account(*ACCOUNT_1_ADDR);

    let csprusd_approve_args = runtime_args! {
        ARG_OWNER => Key::Account(owner),
        ARG_SPENDER => spender,
        ARG_AMOUNT => allowance_amount_1,
    };
    let csprusd_transfer_from_args = runtime_args! {
        ARG_TOKEN_CONTRACT => Key::from(csprusd_token),
        ARG_OWNER => Key::Account(owner),
        ARG_RECIPIENT => recipient,
        ARG_AMOUNT => transfer_from_amount_1,
    };

    let spender_allowance_before =
        csprusd_check_allowance_of(&mut builder, Key::Account(owner), spender);
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request_1 = ExecuteRequestBuilder::contract_call_by_hash(
        owner,
        csprusd_token,
        METHOD_APPROVE,
        csprusd_approve_args,
    )
    .build();

    let transfer_from_request_1 = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        csprusd_test_contract_package,
        None,
        METHOD_FROM_AS_STORED_CONTRACT,
        csprusd_transfer_from_args,
    )
    .build();

    builder.exec(approve_request_1).expect_success().commit();

    let owner_balance_before =
        csprusd_check_balance_of(&mut builder, &csprusd_token, Key::Account(owner));
    assert_eq!(owner_balance_before, initial_supply);

    let spender_allowance_before =
        csprusd_check_allowance_of(&mut builder, Key::Account(owner), spender);
    assert_eq!(spender_allowance_before, allowance_amount_1);

    builder
        .exec(transfer_from_request_1)
        .expect_success()
        .commit();

    let spender_allowance_after =
        csprusd_check_allowance_of(&mut builder, Key::Account(owner), spender);
    assert_eq!(
        spender_allowance_after,
        spender_allowance_before - transfer_from_amount_1
    );

    let owner_balance_after =
        csprusd_check_balance_of(&mut builder, &csprusd_token, Key::Account(owner));
    assert_eq!(
        owner_balance_after,
        owner_balance_before - transfer_from_amount_1
    );
}

#[test]
fn should_not_be_able_to_own_transfer() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    let sender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let recipient = Key::Account(*DEFAULT_ACCOUNT_ADDR);

    let transfer_amount = U256::from(TRANSFER_AMOUNT_1);

    let sender_balance_before = csprusd_check_balance_of(&mut builder, &csprusd_token, sender);
    let recipient_balance_before =
        csprusd_check_balance_of(&mut builder, &csprusd_token, recipient);

    assert_eq!(sender_balance_before, recipient_balance_before);

    let token_transfer_request_1 =
        make_csprusd_transfer_request(sender, &csprusd_token, recipient, transfer_amount);

    builder.exec(token_transfer_request_1).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60017),
        "{:?}",
        error
    );
}

#[test]
fn should_not_be_able_to_own_transfer_from() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    let owner = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let spender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let sender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let recipient = Key::Account(*DEFAULT_ACCOUNT_ADDR);

    let allowance_amount = U256::from(ALLOWANCE_AMOUNT_1);
    let transfer_amount = U256::from(TRANSFER_AMOUNT_1);

    let approve_request =
        make_csprusd_approve_request(sender, &csprusd_token, spender, allowance_amount);

    builder.exec(approve_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60017),
        "{:?}",
        error
    );

    let sender_balance_before = csprusd_check_balance_of(&mut builder, &csprusd_token, sender);
    let recipient_balance_before =
        csprusd_check_balance_of(&mut builder, &csprusd_token, recipient);

    assert_eq!(sender_balance_before, recipient_balance_before);

    let transfer_from_request = {
        let csprusd_transfer_from_args = runtime_args! {
            ARG_OWNER => owner,
            ARG_RECIPIENT => recipient,
            ARG_AMOUNT => transfer_amount,
        };
        ExecuteRequestBuilder::contract_call_by_hash(
            sender.into_account().unwrap(),
            csprusd_token,
            METHOD_TRANSFER_FROM,
            csprusd_transfer_from_args,
        )
        .build()
    };

    builder.exec(transfer_from_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60017),
        "{:?}",
        error
    );
}

#[test]
fn should_verify_zero_amount_transfer_is_rejected() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    let sender = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let recipient = Key::Account(*ACCOUNT_1_ADDR);

    let transfer_amount = U256::zero();

    let token_transfer_request_1 =
        make_csprusd_transfer_request(sender, &csprusd_token, recipient, transfer_amount);

    builder.exec(token_transfer_request_1).commit();

    let error = builder
        .get_error()
        .expect("Transfer should be rejected because transferring 0 amount");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == CANNOT_TRANSFER_ZERO_AMOUNT),
        "{:?}",
        error
    );
}

#[test]
fn should_verify_zero_amount_transfer_from_is_rejected() {
    let (mut builder, TestContext { csprusd_token, .. }) = setup();

    let owner = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let spender = Key::Account(*ACCOUNT_1_ADDR);
    let recipient = Key::Account(*ACCOUNT_2_ADDR);

    let allowance_amount = U256::from(1);
    let transfer_amount = U256::zero();

    let approve_request =
        make_csprusd_approve_request(owner, &csprusd_token, spender, allowance_amount);

    builder.exec(approve_request).expect_success().commit();

    let transfer_from_request = {
        let csprusd_transfer_from_args = runtime_args! {
            ARG_OWNER => owner,
            ARG_RECIPIENT => recipient,
            ARG_AMOUNT => transfer_amount,
        };
        ExecuteRequestBuilder::contract_call_by_hash(
            owner.into_account().unwrap(),
            csprusd_token,
            METHOD_TRANSFER_FROM,
            csprusd_transfer_from_args,
        )
        .build()
    };

    builder.exec(transfer_from_request).commit();

    let error = builder
        .get_error()
        .expect("Transfer should be rejected because transferring 0 amount");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == CANNOT_TRANSFER_ZERO_AMOUNT),
        "{:?}",
        error
    );
}

#[test]
fn should_transfer_contract_to_contract() {
    let (mut builder, test_context) = setup();
    let TestContext {
        csprusd_test_contract_package,
        csprusd_token,
    } = test_context;

    let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
    mint_to_default_address_initial_amount(&mut builder, csprusd_token, initial_supply);

    let sender1 = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let recipient1 = Key::Hash(csprusd_test_contract_package.value());
    let sender2 = Key::Hash(csprusd_test_contract_package.value());
    let recipient2 = Key::Hash([42; 32]);

    test_csprusd_transfer(
        &mut builder,
        &test_context,
        sender1,
        recipient1,
        sender2,
        recipient2,
    );
}

#[test]
fn should_transfer_contract_to_account() {
    let (mut builder, test_context) = setup();
    let TestContext {
        csprusd_test_contract_package,
        csprusd_token,
    } = test_context;

    let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
    mint_to_default_address_initial_amount(&mut builder, csprusd_token, initial_supply);

    let sender1 = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let recipient1 = Key::Hash(csprusd_test_contract_package.value());

    let sender2 = Key::Hash(csprusd_test_contract_package.value());
    let recipient2 = Key::Account(*ACCOUNT_1_ADDR);

    test_csprusd_transfer(
        &mut builder,
        &test_context,
        sender1,
        recipient1,
        sender2,
        recipient2,
    );
}

#[test]
fn should_transfer_account_to_contract() {
    let (mut builder, test_context) = setup();
    let TestContext {
        csprusd_test_contract_package: _,
        csprusd_token,
    } = test_context;

    let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
    mint_to_default_address_initial_amount(&mut builder, csprusd_token, initial_supply);

    let sender1 = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let recipient1 = Key::Account(*ACCOUNT_1_ADDR);
    let sender2 = Key::Account(*ACCOUNT_1_ADDR);
    let recipient2 = Key::Hash(test_context.csprusd_test_contract_package.value());

    test_csprusd_transfer(
        &mut builder,
        &test_context,
        sender1,
        recipient1,
        sender2,
        recipient2,
    );
}

#[test]
fn should_transfer_account_to_account() {
    let (mut builder, test_context) = setup();
    let TestContext {
        csprusd_test_contract_package: _,
        csprusd_token,
    } = test_context;

    let initial_supply = U256::from(TOKEN_TOTAL_SUPPLY);
    mint_to_default_address_initial_amount(&mut builder, csprusd_token, initial_supply);

    let sender1 = Key::Account(*DEFAULT_ACCOUNT_ADDR);
    let recipient1 = Key::Account(*ACCOUNT_1_ADDR);
    let sender2 = Key::Account(*ACCOUNT_1_ADDR);
    let recipient2 = Key::Account(*ACCOUNT_2_ADDR);

    test_csprusd_transfer(
        &mut builder,
        &test_context,
        sender1,
        recipient1,
        sender2,
        recipient2,
    );
}
