use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{
        ACCOUNT_1_ADDR, ACCOUNT_1_PUBLIC_KEY, AMOUNT, ARG_CURRENCY, ARG_DECIMALS,
        ARG_MASTER_MINTER, ARG_NAME, ARG_SYMBOL, BLACKLISTER, CONFIGURE_MINTER_ENTRY_POINT_NAME,
        ERROR_EXCEEDS_MINT_ALLOWANCE, ERROR_OVERFLOW, METHOD_BURN, METHOD_MINT, MINTER,
        MINTER_ALLOWED, OWNER, PAUSER, RECIPIENT, TOKEN_CURRENCY, TOKEN_DECIMALS, TOKEN_NAME,
        TOKEN_OWNER_ADDRESS_1, TOKEN_OWNER_ADDRESS_2, TOKEN_OWNER_AMOUNT_1, TOKEN_OWNER_AMOUNT_2,
        TOKEN_SYMBOL,
    },
    installer_request_builders::{
        csprusd_check_balance_of, csprusd_check_total_supply, setup_with_args, TestContext,
    },
};

use casper_execution_engine::core::{
    engine_state::Error as CoreError, execution::Error as ExecError,
};

#[test]
fn test_minting_and_burning() {
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

    // configure minter
    let configure_minter_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        CONFIGURE_MINTER_ENTRY_POINT_NAME,
        runtime_args! {MINTER => account_1_key, MINTER_ALLOWED => U256::MAX},
    )
    .build();
    builder
        .exec(configure_minter_request)
        .expect_success()
        .commit();

    // mint
    let mint1_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => account_1_key, AMOUNT => U256::one()},
    )
    .build();
    builder.exec(mint1_request).expect_success().commit();

    // check balance
    assert_eq!(
        csprusd_check_balance_of(&mut builder, &csprusd_token, account_1_key),
        U256::from(1)
    );

    // check total_supply
    let total_supply = csprusd_check_total_supply(&mut builder, &csprusd_token);
    assert_eq!(total_supply, U256::one());

    // mint
    let mint1_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => account_1_key, AMOUNT => U256::from(443)},
    )
    .build();
    builder.exec(mint1_request).expect_success().commit();

    // check balance
    assert_eq!(
        csprusd_check_balance_of(&mut builder, &csprusd_token, account_1_key),
        U256::from(444)
    );

    // check total_supply
    let total_supply = csprusd_check_total_supply(&mut builder, &csprusd_token);
    assert_eq!(total_supply, U256::from(444));

    // burn
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_BURN,
        runtime_args! { AMOUNT => U256::one()},
    )
    .build();
    builder.exec(burn_request).expect_success().commit();

    // check balance
    assert_eq!(
        csprusd_check_balance_of(&mut builder, &csprusd_token, account_1_key),
        U256::from(443)
    );

    // check total_supply
    let total_supply = csprusd_check_total_supply(&mut builder, &csprusd_token);
    assert_eq!(total_supply, U256::from(443));

    // burn
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        csprusd_token,
        METHOD_BURN,
        runtime_args! { AMOUNT => U256::from(443)},
    )
    .build();
    builder.exec(burn_request).expect_success().commit();

    // check balance
    assert_eq!(
        csprusd_check_balance_of(&mut builder, &csprusd_token, account_1_key),
        U256::zero()
    );

    // check total_supply
    let total_supply = csprusd_check_total_supply(&mut builder, &csprusd_token);
    assert_eq!(total_supply, U256::zero());
}

#[test]
fn test_should_not_mint_or_burn_above_limits() {
    let mint_amount = U256::MAX;
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
        runtime_args! {RECIPIENT => TOKEN_OWNER_ADDRESS_1, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();

    let mint_request_2 = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => TOKEN_OWNER_ADDRESS_2, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_2)},
    )
    .build();
    builder.exec(mint_request_2).expect_success().commit();
    assert_eq!(
        csprusd_check_balance_of(&mut builder, &csprusd_token, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1)
    );

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        csprusd_token,
        METHOD_MINT,
        runtime_args! {
            RECIPIENT => TOKEN_OWNER_ADDRESS_1,
            AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_EXCEEDS_MINT_ALLOWANCE),
        "{:?}",
        error
    );

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
        runtime_args! {
            RECIPIENT => TOKEN_OWNER_ADDRESS_1,
            AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_OVERFLOW),
        "{:?}",
        error
    );
}
