use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    MINIMUM_ACCOUNT_CREATION_BALANCE, PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::core::engine_state::ExecuteRequest;
use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, system::mint, CLTyped, ContractHash,
    ContractPackageHash, Key, RuntimeArgs, U256,
};

use crate::utility::constants::{
    ALLOWANCE_AMOUNT_1, ALLOWANCE_AMOUNT_2, TRANSFER_AMOUNT_1, TRANSFER_AMOUNT_2,
};

use super::constants::{
    ACCOUNT_1_ADDR, ACCOUNT_1_PUBLIC_KEY, ACCOUNT_2_ADDR, ADDRESS, ARG_AMOUNT, ARG_CURRENCY,
    ARG_DECIMALS, ARG_MASTER_MINTER, ARG_NAME, ARG_OWNER, ARG_RECIPIENT, ARG_SPENDER, ARG_SYMBOL,
    ARG_TOKEN_CONTRACT, BLACKLISTER, CHECK_ALLOWANCE_OF_ENTRYPOINT, CHECK_BALANCE_OF_ENTRYPOINT,
    CHECK_TOTAL_SUPPLY_ENTRYPOINT, CONTRACT_HASH, CSPR_USD_CONTRACT_WASM,
    CSPR_USD_TEST_CONTRACT_WASM, METHOD_APPROVE, METHOD_APPROVE_AS_STORED_CONTRACT,
    METHOD_TRANSFER, METHOD_TRANSFER_AS_STORED_CONTRACT, OWNER, PAUSER, RESULT_KEY,
    TEST_CONTRACT_PACKAGE_HASH, TOKEN_CURRENCY, TOKEN_DECIMALS, TOKEN_NAME, TOKEN_SYMBOL,
};

/// Converts hash addr of Account into Hash, and Hash into Account
///
/// This is useful for making sure CSPR USD library respects different variants of Key when storing
/// balances.
pub(crate) fn invert_csprusd_address(address: Key) -> Key {
    match address {
        Key::Account(account_hash) => Key::Hash(account_hash.value()),
        Key::Hash(contract_hash) => Key::Account(AccountHash::new(contract_hash)),
        _ => panic!("Unsupported Key variant"),
    }
}

#[derive(Copy, Clone)]
pub(crate) struct TestContext {
    pub(crate) csprusd_token: ContractHash,
    pub(crate) csprusd_test_contract_package: ContractPackageHash,
}

pub(crate) fn setup() -> (InMemoryWasmTestBuilder, TestContext) {
    let account_1_key: Key = Key::Account(*ACCOUNT_1_ADDR);
    setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_CURRENCY => TOKEN_CURRENCY,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_MASTER_MINTER => account_1_key,
        PAUSER => account_1_key,
        BLACKLISTER => ACCOUNT_1_PUBLIC_KEY.clone(),
        OWNER => account_1_key,
    })
}

pub(crate) fn setup_with_args(install_args: RuntimeArgs) -> (InMemoryWasmTestBuilder, TestContext) {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);

    let id: Option<u64> = None;
    let transfer_1_args = runtime_args! {
        mint::ARG_TARGET => *ACCOUNT_1_ADDR,
        mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
        mint::ARG_ID => id,
    };
    let transfer_2_args = runtime_args! {
        mint::ARG_TARGET => *ACCOUNT_2_ADDR,
        mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
        mint::ARG_ID => id,
    };

    let transfer_request_1 =
        ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_1_args).build();
    let transfer_request_2 =
        ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_2_args).build();

    let install_request_1 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CSPR_USD_CONTRACT_WASM,
        install_args,
    )
    .build();

    let install_request_2 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CSPR_USD_TEST_CONTRACT_WASM,
        RuntimeArgs::default(),
    )
    .build();

    builder.exec(transfer_request_1).expect_success().commit();
    builder.exec(transfer_request_2).expect_success().commit();
    builder.exec(install_request_1).expect_success().commit();
    builder.exec(install_request_2).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let csprusd_token = account
        .named_keys()
        .get(CONTRACT_HASH)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let csprusd_test_contract_package = account
        .named_keys()
        .get(TEST_CONTRACT_PACKAGE_HASH)
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract package hash");

    let test_context = TestContext {
        csprusd_token,
        csprusd_test_contract_package,
    };

    (builder, test_context)
}

pub(crate) fn csprusd_check_total_supply(
    builder: &mut InMemoryWasmTestBuilder,
    csprusd_contract_hash: &ContractHash,
) -> U256 {
    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let csprusd_test_contract_package = account
        .named_keys()
        .get(TEST_CONTRACT_PACKAGE_HASH)
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have test contract hash");

    let check_total_supply_args = runtime_args! {
        ARG_TOKEN_CONTRACT => Key::from(*csprusd_contract_hash),
    };

    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        csprusd_test_contract_package,
        None,
        CHECK_TOTAL_SUPPLY_ENTRYPOINT,
        check_total_supply_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();

    get_test_result(builder, csprusd_test_contract_package)
}

pub(crate) fn get_test_result<T: FromBytes + CLTyped>(
    builder: &mut InMemoryWasmTestBuilder,
    csprusd_test_contract_package: ContractPackageHash,
) -> T {
    let contract_package = builder
        .get_contract_package(csprusd_test_contract_package)
        .expect("should have contract package");
    let enabled_versions = contract_package.enabled_versions();
    let (_version, contract_hash) = enabled_versions
        .iter()
        .rev()
        .next()
        .expect("should have latest version");

    builder.get_value(*contract_hash, RESULT_KEY)
}

pub(crate) fn csprusd_check_balance_of(
    builder: &mut InMemoryWasmTestBuilder,
    csprusd_contract_hash: &ContractHash,
    address: Key,
) -> U256 {
    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let csprusd_test_contract_package = account
        .named_keys()
        .get(TEST_CONTRACT_PACKAGE_HASH)
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have test contract hash");

    let check_balance_args = runtime_args! {
        ARG_TOKEN_CONTRACT => Key::from(*csprusd_contract_hash),
        ADDRESS => address,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        csprusd_test_contract_package,
        None,
        CHECK_BALANCE_OF_ENTRYPOINT,
        check_balance_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();

    get_test_result(builder, csprusd_test_contract_package)
}

pub(crate) fn csprusd_check_allowance_of(
    builder: &mut InMemoryWasmTestBuilder,
    owner: Key,
    spender: Key,
) -> U256 {
    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");
    let csprusd_contract_hash = account
        .named_keys()
        .get(CONTRACT_HASH)
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have test contract hash");
    let csprusd_test_contract_package = account
        .named_keys()
        .get(TEST_CONTRACT_PACKAGE_HASH)
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have test contract hash");

    let check_balance_args = runtime_args! {
        ARG_TOKEN_CONTRACT => Key::from(csprusd_contract_hash),
        ARG_OWNER => owner,
        ARG_SPENDER => spender,
    };
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        csprusd_test_contract_package,
        None,
        CHECK_ALLOWANCE_OF_ENTRYPOINT,
        check_balance_args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();

    get_test_result(builder, csprusd_test_contract_package)
}

pub(crate) fn test_csprusd_transfer(
    builder: &mut InMemoryWasmTestBuilder,
    test_context: &TestContext,
    sender1: Key,
    recipient1: Key,
    sender2: Key,
    recipient2: Key,
) {
    let TestContext { csprusd_token, .. } = test_context;

    let transfer_amount_1 = U256::from(TRANSFER_AMOUNT_1);
    let transfer_amount_2 = U256::from(TRANSFER_AMOUNT_2);

    let sender_balance_before = csprusd_check_balance_of(builder, csprusd_token, sender1);
    assert_ne!(sender_balance_before, U256::zero());

    let account_1_balance_before = csprusd_check_balance_of(builder, csprusd_token, recipient1);
    assert_eq!(account_1_balance_before, U256::zero());

    let account_2_balance_before = csprusd_check_balance_of(builder, csprusd_token, recipient1);
    assert_eq!(account_2_balance_before, U256::zero());

    let token_transfer_request_1 =
        make_csprusd_transfer_request(sender1, csprusd_token, recipient1, transfer_amount_1);

    builder
        .exec(token_transfer_request_1)
        .expect_success()
        .commit();

    let account_1_balance_after = csprusd_check_balance_of(builder, csprusd_token, recipient1);
    assert_eq!(account_1_balance_after, transfer_amount_1);
    let account_1_balance_before = account_1_balance_after;

    let sender_balance_after = csprusd_check_balance_of(builder, csprusd_token, sender1);
    assert_eq!(
        sender_balance_after,
        sender_balance_before - transfer_amount_1
    );
    let sender_balance_before = sender_balance_after;

    let token_transfer_request_2 =
        make_csprusd_transfer_request(sender2, csprusd_token, recipient2, transfer_amount_2);

    builder
        .exec(token_transfer_request_2)
        .expect_success()
        .commit();

    let sender_balance_after = csprusd_check_balance_of(builder, csprusd_token, sender1);
    assert_eq!(sender_balance_after, sender_balance_before);

    let account_1_balance_after = csprusd_check_balance_of(builder, csprusd_token, recipient1);
    assert!(account_1_balance_after < account_1_balance_before);
    assert_eq!(
        account_1_balance_after,
        transfer_amount_1 - transfer_amount_2
    );

    let account_2_balance_after = csprusd_check_balance_of(builder, csprusd_token, recipient2);
    assert_eq!(account_2_balance_after, transfer_amount_2);
}

pub(crate) fn make_csprusd_transfer_request(
    sender: Key,
    csprusd_token: &ContractHash,
    recipient: Key,
    amount: U256,
) -> ExecuteRequest {
    match sender {
        Key::Account(sender) => ExecuteRequestBuilder::contract_call_by_hash(
            sender,
            *csprusd_token,
            METHOD_TRANSFER,
            runtime_args! {
                ARG_AMOUNT => amount,
                ARG_RECIPIENT => recipient,
            },
        )
        .build(),
        Key::Hash(contract_package_hash) => ExecuteRequestBuilder::versioned_contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            ContractPackageHash::new(contract_package_hash),
            None,
            METHOD_TRANSFER_AS_STORED_CONTRACT,
            runtime_args! {
                ARG_TOKEN_CONTRACT => Key::from(*csprusd_token),
                ARG_AMOUNT => amount,
                ARG_RECIPIENT => recipient,
            },
        )
        .build(),
        _ => panic!("Unknown variant"),
    }
}

pub(crate) fn make_csprusd_approve_request(
    sender: Key,
    csprusd_token: &ContractHash,
    spender: Key,
    amount: U256,
) -> ExecuteRequest {
    match sender {
        Key::Account(sender) => ExecuteRequestBuilder::contract_call_by_hash(
            sender,
            *csprusd_token,
            METHOD_APPROVE,
            runtime_args! {
                ARG_SPENDER => spender,
                ARG_AMOUNT => amount,
            },
        )
        .build(),
        Key::Hash(contract_package_hash) => ExecuteRequestBuilder::versioned_contract_call_by_hash(
            *DEFAULT_ACCOUNT_ADDR,
            ContractPackageHash::new(contract_package_hash),
            None,
            METHOD_APPROVE_AS_STORED_CONTRACT,
            runtime_args! {
                ARG_TOKEN_CONTRACT => Key::from(*csprusd_token),
                ARG_SPENDER => spender,
                ARG_AMOUNT => amount,
            },
        )
        .build(),
        _ => panic!("Unknown variant"),
    }
}

pub(crate) fn test_approve_for(
    builder: &mut InMemoryWasmTestBuilder,
    test_context: &TestContext,
    sender: Key,
    owner: Key,
    spender: Key,
) {
    let TestContext { csprusd_token, .. } = test_context;
    let allowance_amount_1 = U256::from(ALLOWANCE_AMOUNT_1);
    let allowance_amount_2 = U256::from(ALLOWANCE_AMOUNT_2);

    let spender_allowance_before = csprusd_check_allowance_of(builder, owner, spender);
    assert_eq!(spender_allowance_before, U256::zero());

    let approve_request_1 =
        make_csprusd_approve_request(sender, csprusd_token, spender, allowance_amount_1);
    let approve_request_2 =
        make_csprusd_approve_request(sender, csprusd_token, spender, allowance_amount_2);

    builder.exec(approve_request_1).expect_success().commit();
    {
        let account_1_allowance_after = csprusd_check_allowance_of(builder, owner, spender);
        assert_eq!(account_1_allowance_after, allowance_amount_1);
    }

    // Approve overwrites existing amount rather than increase it

    builder.exec(approve_request_2).expect_success().commit();

    let account_1_allowance_after = csprusd_check_allowance_of(builder, owner, spender);
    assert_eq!(account_1_allowance_after, allowance_amount_2);

    // Swap Key::Account into Hash and other way
    let inverted_spender_key = invert_csprusd_address(spender);

    let inverted_spender_allowance =
        csprusd_check_allowance_of(builder, owner, inverted_spender_key);
    assert_eq!(inverted_spender_allowance, U256::zero());
}
