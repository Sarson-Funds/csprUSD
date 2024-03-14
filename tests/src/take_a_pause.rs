use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ACCOUNT_1_PUBLIC_KEY, ACCOUNT_2_ADDR, AMOUNT, APPROVE_ENTRY_POINT_NAME,
        ARG_CURRENCY, ARG_DECIMALS, ARG_MASTER_MINTER, ARG_NAME, ARG_SYMBOL, BLACKLISTER,
        CONFIGURE_MINTER_ENTRY_POINT_NAME, CONTRACT_PAUSED_ERROR_CODE, IS_PAUSED, METHOD_PAUSE,
        METHOD_UNPAUSE, METHOD_UPDATE_PAUSER, MINTER, MINTER_ALLOWED, NEW, NON_PAUSER_ERROR_CODE,
        OWNER, PAUSER, RECIPIENT, SPENDER, TOKEN_CURRENCY, TOKEN_DECIMALS, TOKEN_NAME,
        TOKEN_SYMBOL,
    },
    installer_request_builders::{setup_with_args, TestContext},
};
use casper_engine_test_support::{ExecuteRequestBuilder, WasmTestBuilder};
use casper_execution_engine::{
    core::{
        engine_state::{Error as CoreError, ExecuteRequest},
        execution::Error as ExecError,
    },
    storage::global_state::in_memory::InMemoryGlobalState,
};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, U256};

#[test]
fn only_pauser_can_pause_and_can_update_pauser() {
    let account_1_key: Key = Key::Account(*ACCOUNT_1_ADDR);
    let account_2_key = Key::Account(*ACCOUNT_2_ADDR);

    let (mut builder, TestContext { csprusd_token, .. }) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_CURRENCY => TOKEN_CURRENCY,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_MASTER_MINTER => account_1_key,
        PAUSER => account_1_key,
        BLACKLISTER => ACCOUNT_1_PUBLIC_KEY.clone(),
        OWNER => account_1_key,
    });

    // assure who's the pauser
    let pauser: Key = builder.get_value(csprusd_token, PAUSER);
    assert_eq!(pauser, account_1_key);

    // non-pauser account tries to pause
    let non_pauser_pause_contract_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        METHOD_PAUSE,
        runtime_args! {},
    )
    .build();
    builder.exec(non_pauser_pause_contract_request).commit();
    let error = builder
        .get_error()
        .expect("Request should be rejected because non-paused account tried to pause contract!!!");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == NON_PAUSER_ERROR_CODE),
        "{:?}",
        error
    );

    // pauser account pauses contract
    let pause_contract_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_PAUSE,
        runtime_args! {},
    )
    .build();
    builder
        .exec(pause_contract_request)
        .expect_success()
        .commit();

    // assure it's paused
    let paused: bool = builder.get_value(csprusd_token, IS_PAUSED);
    assert!(paused);

    // non-pauser account tries to unpause
    let non_pauser_unpause_contract_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        METHOD_UNPAUSE,
        runtime_args! {},
    )
    .build();
    builder.exec(non_pauser_unpause_contract_request).commit();
    let error = builder
        .get_error()
        .expect("Request should be rejected because non-paused account tried to pause contract!!!");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == NON_PAUSER_ERROR_CODE),
        "{:?}",
        error
    );

    // update pauser account
    let update_pauser_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_UPDATE_PAUSER,
        runtime_args! {NEW => account_2_key},
    )
    .build();
    builder
        .exec(update_pauser_request)
        .expect_success()
        .commit();

    // assure who's the pauser
    let pauser: Key = builder.get_value(csprusd_token, PAUSER);
    assert_eq!(pauser, account_2_key);

    // assure old pauser can't unpause contract
    let non_pauser_unpause_contract_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_UNPAUSE,
        runtime_args! {},
    )
    .build();
    builder.exec(non_pauser_unpause_contract_request).commit();
    let error = builder
        .get_error()
        .expect("Request should be rejected because non-paused account tried to pause contract!!!");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == NON_PAUSER_ERROR_CODE),
        "{:?}",
        error
    );

    // assure new pauser can unpause contract
    // pauser account pauses contract
    let unpause_contract_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_2_ADDR,
        csprusd_token,
        METHOD_PAUSE,
        runtime_args! {},
    )
    .build();
    builder
        .exec(unpause_contract_request)
        .expect_success()
        .commit();
}

#[test]
fn wont_execute_entrypoints_guarded_by_when_not_paused() {
    // install contract
    let account_1_key: Key = Key::Account(*ACCOUNT_1_ADDR);

    let (mut builder, TestContext { csprusd_token, .. }) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_CURRENCY => TOKEN_CURRENCY,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_MASTER_MINTER => account_1_key,
        PAUSER => account_1_key,
        BLACKLISTER => ACCOUNT_1_PUBLIC_KEY.clone(),
        OWNER => account_1_key,
    });

    // assure it's unpaused
    let paused: bool = builder.get_value(csprusd_token, IS_PAUSED);
    assert!(!paused);

    // assure who's the pauser
    let pauser: Key = builder.get_value(csprusd_token, PAUSER);
    assert_eq!(pauser, account_1_key);

    // pause contract
    let pause_contract_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_PAUSE,
        runtime_args! {},
    )
    .build();
    builder
        .exec(pause_contract_request)
        .expect_success()
        .commit();

    // assure it's paused
    let paused: bool = builder.get_value(csprusd_token, IS_PAUSED);
    assert!(paused);

    // try calling different entrypoints which are guarded by "when_not_paused()":
    // configure minter
    let configure_minter_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        CONFIGURE_MINTER_ENTRY_POINT_NAME,
        runtime_args! {MINTER => account_1_key, MINTER_ALLOWED => U256::MAX},
    )
    .build();
    assert_fails_because_contract_paused(&mut builder, configure_minter_request);

    // approve
    let approve_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        APPROVE_ENTRY_POINT_NAME,
        runtime_args! {SPENDER => account_1_key, AMOUNT => U256::one()},
    )
    .build();
    assert_fails_because_contract_paused(&mut builder, approve_request);

    // decrease_allowance
    let decrease_allowance_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        APPROVE_ENTRY_POINT_NAME,
        runtime_args! {SPENDER => account_1_key, AMOUNT => U256::one()},
    )
    .build();
    assert_fails_because_contract_paused(&mut builder, decrease_allowance_request);

    // increase_allowance
    let increase_allowance_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        APPROVE_ENTRY_POINT_NAME,
        runtime_args! {SPENDER => account_1_key, AMOUNT => U256::one()},
    )
    .build();
    assert_fails_because_contract_paused(&mut builder, increase_allowance_request);

    // transfer
    let transfer_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        APPROVE_ENTRY_POINT_NAME,
        runtime_args! {RECIPIENT => account_1_key, AMOUNT => U256::one()},
    )
    .build();
    assert_fails_because_contract_paused(&mut builder, transfer_request);

    // transfer_from
    let transfer_from_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        APPROVE_ENTRY_POINT_NAME,
        runtime_args! {RECIPIENT => account_1_key,OWNER => account_1_key, AMOUNT => U256::one()},
    )
    .build();
    assert_fails_because_contract_paused(&mut builder, transfer_from_request);

    // mint
    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        APPROVE_ENTRY_POINT_NAME,
        runtime_args! {RECIPIENT => account_1_key, AMOUNT => U256::one()},
    )
    .build();
    assert_fails_because_contract_paused(&mut builder, mint_request);

    // burn
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        APPROVE_ENTRY_POINT_NAME,
        runtime_args! {AMOUNT => U256::one()},
    )
    .build();
    assert_fails_because_contract_paused(&mut builder, burn_request);
}

fn assert_fails_because_contract_paused(
    builder: &mut WasmTestBuilder<InMemoryGlobalState>,
    request: ExecuteRequest,
) {
    builder.exec(request).commit();
    let error = builder
        .get_error()
        .expect("Should have error because contract is paused!!!");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == CONTRACT_PAUSED_ERROR_CODE),
        "{:?}",
        error
    );
}
