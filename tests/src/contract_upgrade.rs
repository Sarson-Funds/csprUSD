use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs};

use crate::utility::constants::{
    ACCOUNT_1_ADDR, ARG_CURRENCY, ARG_DECIMALS, ARG_MASTER_MINTER, ARG_NAME, ARG_SYMBOL,
    BLACKLISTER, CONTRACT_HASH, OWNER, PAUSER, TOKEN_CURRENCY, TOKEN_DECIMALS, TOKEN_NAME,
    TOKEN_SYMBOL,
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
    // install v0 of contract
    let mut builder = InMemoryWasmTestBuilder::default();
    builder
        .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
        .commit();

    // Install the first version of the contract
    let contract_v0_file: &str = "./../contract_versions/v0/csprUSD_v0.wasm"; // searches given file in: tests/wasm/
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

    // Check the contract hash.
    let contract_v1_hash = builder
        .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
        .named_keys()
        .get("csprusd_contract_hash_CasperTest")
        .expect("must have contract hash key as part of contract creation")
        .into_hash()
        .map(ContractHash::new)
        .expect("must get contract hash");

    // Verify the first contract version is 1. We'll check this when we upgrade later.
    // TODO: !!! our contract doesn't have "version" stored in the account's or contract's named
    // keys. Do we need it?  maybe a timestamp as version?
    // let version_key = *account
    //     .named_keys()
    //     .get("version")
    //     .expect("version uref should exist");

    // let version = builder
    //     .query(None, version_key, &[])
    //     .expect("should be stored value.")
    //     .as_cl_value()
    //     .expect("should be cl value.")
    //     .clone()
    //     .into_t::<u32>()
    //     .expect("should be u32.");
    // assert_eq!(version, 1);
    // TODO: !!!

    // prove that blacklister is a Key::Account
    let blacklister: Key = builder.get_value(csprusd_token, BLACKLISTER);
    assert_eq!(blacklister, account_1_key);

    // do some side effects: name a minter, mint some money, blacklist someone, etc

    // upgrade contract to v1
    let contract_v1_file: &str = "./../contract_versions/v1/csprUSD_v0.wasm"; // searches given file in: tests/wasm/

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
            BLACKLISTER => account_1_key,
            OWNER => account_1_key,
        },
    )
    .build();

    builder
        .exec(contract_v2_installation_request)
        .expect_success()
        .commit();

    // prove that blacklister is now a PublicKey

    // prove that new version still has access to the same data: minting allowance changed, etc
    //  do this by methods, not only by just simply querying global state e.g.: who is owner, etc?
    //   e.g.: try minting some money which exceeds the minting allowance left from operations in v0
}

// /// Install version 1 of the counter contract and check its functionality.
//     /// Then, upgrade the contract by installing a second Wasm for version 2.
//     /// Check the functionality of the second version.
//     /// Test summary:
//     /// - Install the counter-v1.wasm contract.
//     /// - Check the contract hash.
//     /// - Check the contract version is 1.
//     /// - Verify the initial value of count is 0.
//     /// - Test the counter_inc entry point and increment the counter.
//     /// - Verify that the count value is now 1.
//     /// - Call the decrement entry point, which should fail.
//     /// - Ensure the count value was not decremented and is still 1.
//     /// - UPGRADE the contract by installing the counter-v2.wasm.
//     /// - Assert that we have a new contract hash for the upgraded version.
//     /// - Verify the new contract version is 2.
//     /// - Increment the counter to check that counter_inc is still working after the upgrade.
// Count is now 2.     /// - Call the decrement entry point and verify that the count is now 1.
//     #[test]
//     fn install_version1_and_upgrade_to_version2() {
//         let mut builder = InMemoryWasmTestBuilder::default();
//         builder
//             .run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST)
//             .commit();

//         // Install the first version of the contract.
//         let contract_v1_installation_request = ExecuteRequestBuilder::standard(
//             *DEFAULT_ACCOUNT_ADDR,
//             COUNTER_V1_WASM,
//             runtime_args! {},
//         )
//         .build();

//         builder
//             .exec(contract_v1_installation_request)
//             .expect_success()
//             .commit();

//         // Check the contract hash.
//         let contract_v1_hash = builder
//             .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
//             .named_keys()
//             .get(CONTRACT_KEY)
//             .expect("must have contract hash key as part of contract creation")
//             .into_hash()
//             .map(ContractHash::new)
//             .expect("must get contract hash");

//         // Verify the first contract version is 1. We'll check this when we upgrade later.
//         let account = builder
//             .get_account(*DEFAULT_ACCOUNT_ADDR)
//             .expect("should have account");

//         let version_key = *account
//             .named_keys()
//             .get(CONTRACT_VERSION_KEY)
//             .expect("version uref should exist");

//         let version = builder
//             .query(None, version_key, &[])
//             .expect("should be stored value.")
//             .as_cl_value()
//             .expect("should be cl value.")
//             .clone()
//             .into_t::<u32>()
//             .expect("should be u32.");

//         assert_eq!(version, 1);

//         // Verify the initial value of count is 0.
//         let contract = builder
//             .get_contract(contract_v1_hash)
//             .expect("this contract should exist");

//         let count_key = *contract
//             .named_keys()
//             .get(COUNT_KEY)
//             .expect("count uref should exist in the contract named keys");

//         let count = builder
//             .query(None, count_key, &[])
//             .expect("should be stored value.")
//             .as_cl_value()
//             .expect("should be cl value.")
//             .clone()
//             .into_t::<i32>()
//             .expect("should be i32.");

//         assert_eq!(count, 0);

//         // Use session code to increment the counter.
//         let session_code_request = ExecuteRequestBuilder::standard(
//             *DEFAULT_ACCOUNT_ADDR,
//             COUNTER_CALL_WASM,
//             runtime_args! {
//                 CONTRACT_KEY => contract_v1_hash
//             },
//         )
//         .build();

//         builder.exec(session_code_request).expect_success().commit();

//         // Verify the value of count is now 1.
//         let incremented_count = builder
//             .query(None, count_key, &[])
//             .expect("should be stored value.")
//             .as_cl_value()
//             .expect("should be cl value.")
//             .clone()
//             .into_t::<i32>()
//             .expect("should be i32.");

//         assert_eq!(incremented_count, 1);

//         // Call the decrement entry point, which should not be in version 1 before the upgrade.
//         let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
//             *DEFAULT_ACCOUNT_ADDR,
//             contract_v1_hash,
//             ENTRY_POINT_COUNTER_DECREMENT,
//             runtime_args! {},
//         )
//         .build();

//         // Try executing the decrement entry point and expect an error.
//         builder
//             .exec(contract_decrement_request)
//             .expect_failure()
//             .commit();

//         // Ensure the count value was not decremented.
//         let current_count = builder
//             .query(None, count_key, &[])
//             .expect("should be stored value.")
//             .as_cl_value()
//             .expect("should be cl value.")
//             .clone()
//             .into_t::<i32>()
//             .expect("should be i32.");

//         assert_eq!(current_count, 1);

//         ////////////////////////////////////////////////////////////////
//         // Upgrade the contract.
//         ////////////////////////////////////////////////////////////////
//         let contract_v2_installation_request = ExecuteRequestBuilder::standard(
//             *DEFAULT_ACCOUNT_ADDR,
//             COUNTER_V2_WASM,
//             runtime_args! {},
//         )
//         .build();

//         builder
//             .exec(contract_v2_installation_request)
//             .expect_success()
//             .commit();

//         let contract_v2_hash = builder
//             .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
//             .named_keys()
//             .get(CONTRACT_KEY)
//             .expect("must have contract hash key as part of contract creation")
//             .into_hash()
//             .map(ContractHash::new)
//             .expect("must get contract hash");

//         // Assert that we have a new contract hash for the upgraded version.
//         assert_ne!(contract_v1_hash, contract_v2_hash);

//         // Verify the contract version is now 2.
//         let account = builder
//             .get_account(*DEFAULT_ACCOUNT_ADDR)
//             .expect("should have account");

//         let version_key = *account
//             .named_keys()
//             .get(CONTRACT_VERSION_KEY)
//             .expect("version uref should exist");

//         let version = builder
//             .query(None, version_key, &[])
//             .expect("should be stored value.")
//             .as_cl_value()
//             .expect("should be cl value.")
//             .clone()
//             .into_t::<u32>()
//             .expect("should be u32.");

//         assert_eq!(version, 2);

//         // Call the increment entry point to increment the value stored under "count".
//         let contract_increment_request = ExecuteRequestBuilder::contract_call_by_hash(
//             *DEFAULT_ACCOUNT_ADDR,
//             contract_v2_hash,
//             ENTRY_POINT_COUNTER_INC,
//             runtime_args! {},
//         )
//         .build();

//         builder
//             .exec(contract_increment_request)
//             .expect_success()
//             .commit();

//         // Call the decrement entry point to decrement the value stored under "count".
//         let contract_call_request = ExecuteRequestBuilder::contract_call_by_hash(
//             *DEFAULT_ACCOUNT_ADDR,
//             contract_v2_hash,
//             ENTRY_POINT_COUNTER_DECREMENT,
//             runtime_args! {},
//         )
//         .build();

//         builder
//             .exec(contract_call_request)
//             .expect_success()
//             .commit();

//         // Expect the counter to be 1 now.
//         // This tells us the contract was successfully upgraded and the decrement entry point can
// be called.         let decremented_count = builder
//             .query(None, count_key, &[])
//             .expect("should be stored value.")
//             .as_cl_value()
//             .expect("should be cl value.")
//             .clone()
//             .into_t::<i32>()
//             .expect("should be i32.");

//         assert_eq!(decremented_count, 1);
//     }
