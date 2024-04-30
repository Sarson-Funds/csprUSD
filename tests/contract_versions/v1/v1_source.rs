#![no_std]
#![no_main]

extern crate alloc;

mod allowances;
mod assertion_utils;
mod balances;
mod blacklisting;
pub mod constants;
pub mod entry_points;
mod error;
mod events;
mod minters;
mod utils;

use alloc::string::{String, ToString};

use allowances::{get_allowances_uref, read_allowance_from, write_allowance_to};
use balances::{get_balances_uref, read_balance_from, transfer_balance, write_balance_to};
use entry_points::generate_entry_points;

use casper_contract::{
    contract_api::{
        runtime::{self, print, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, AsymmetricType, CLValue, ContractHash, Key, PublicKey,
    RuntimeArgs, U256,
};

use constants::{
    ADDRESS, ALLOWANCES, AMOUNT, BALANCES, BLACKLISTED_ADDRESSES_COUNT, BLACKLISTER,
    CONTRACT_ACCESS, CONTRACT_HASH, CONTRACT_PACKAGE_HASH, CONTRACT_VERSION, CURRENCY, DECIMALS,
    DICT_BLACKLISTED_ADDR_TO_INDEX, DICT_INDEX_TO_BLACKLISTED_ADDR, INIT_ENTRY_POINT_NAME,
    IS_PAUSED, KEY, MASTER_MINTER, MINTER, MINTERS, MINTER_ALLOWED, NAME, NEW, OWNER, PACKAGE_HASH,
    PAUSER, RECIPIENT, SPENDER, SYMBOL, TOTAL_SUPPLY,
};
pub use error::CsprUSDError;
use events::{
    init_events, Blacklisted, BlacklisterChanged, Burn, DecreaseAllowance, Event,
    IncreaseAllowance, MasterMinterChanged, Mint, MinterConfigured, MinterRemoved, NewPauser,
    OwnershipTransferred, Pause, SetAllowance, Transfer, TransferFrom, UnBlacklisted, Unpause,
};
use utils::{
    get_immediate_caller_address, get_total_supply_uref, get_uref, read_total_supply_from,
    write_total_supply_to,
};

use assertion_utils::{
    only_blacklister, only_master_minter, only_minters, only_owner, only_pauser, when_not_paused,
};
use blacklisting::{blacklist_key, is_blacklisted_util, un_blacklist_address};
use minters::{
    add_minter, is_minter_util, read_minter_allowed, remove_minter_util, set_minter_allowed,
};

#[no_mangle]
pub extern "C" fn name() {
    runtime::ret(CLValue::from_t(utils::read_from::<String>(NAME)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    runtime::ret(CLValue::from_t(utils::read_from::<String>(SYMBOL)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn pauser() {
    runtime::ret(CLValue::from_t(utils::read_from::<PublicKey>(PAUSER)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn is_paused() {
    runtime::ret(CLValue::from_t(utils::read_from::<bool>(IS_PAUSED)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn owner() {
    runtime::ret(CLValue::from_t(utils::read_from::<Key>(OWNER)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn master_minter() {
    runtime::ret(CLValue::from_t(utils::read_from::<Key>(MASTER_MINTER)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn blacklister() {
    runtime::ret(CLValue::from_t(utils::read_from::<PublicKey>(BLACKLISTER)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    runtime::ret(CLValue::from_t(utils::read_from::<u8>(DECIMALS)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    runtime::ret(CLValue::from_t(utils::read_from::<U256>(TOTAL_SUPPLY)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn pause_contract() {
    only_pauser();

    storage::write(get_uref(IS_PAUSED), true);
    events::emit_event(Event::Pause(Pause {}));
}

#[no_mangle]
pub extern "C" fn unpause_contract() {
    only_pauser();

    storage::write(get_uref(IS_PAUSED), false);
    events::emit_event(Event::Unpause(Unpause {}));
}

#[no_mangle]
pub extern "C" fn update_pauser() {
    only_owner();

    let new_pauser: PublicKey = runtime::get_named_arg(NEW);
    storage::write(get_uref(PAUSER), new_pauser.clone());
    events::emit_event(Event::PauserChanged(NewPauser { new_pauser }));
}

#[no_mangle]
pub extern "C" fn update_master_minter() {
    only_owner();

    let new_master_minter: Key = runtime::get_named_arg(NEW);
    storage::write(get_uref(MASTER_MINTER), new_master_minter);
    events::emit_event(Event::MasterMinterChanged(MasterMinterChanged {
        new_master_minter,
    }));
}

#[no_mangle]
pub extern "C" fn is_blacklisted() {
    let address = runtime::get_named_arg(KEY);
    let is_blacklisted: bool = is_blacklisted_util(address);
    runtime::ret(CLValue::from_t(is_blacklisted).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn blacklist() {
    only_blacklister();

    let key: Key = runtime::get_named_arg(KEY);
    blacklist_key(key);

    events::emit_event(Event::Blacklisted(Blacklisted { key }));
}

#[no_mangle]
pub extern "C" fn un_blacklist() {
    only_blacklister();

    let key: Key = runtime::get_named_arg(KEY);
    un_blacklist_address(key);

    events::emit_event(Event::UnBlacklisted(UnBlacklisted { key }));
}

#[no_mangle]
pub extern "C" fn update_blacklister() {
    only_owner();

    let new_blacklister: PublicKey = runtime::get_named_arg(NEW);
    storage::write(get_uref(BLACKLISTER), new_blacklister.clone());

    events::emit_event(Event::BlacklisterChanged(BlacklisterChanged {
        new_blacklister,
    }));
}

#[no_mangle]
pub extern "C" fn transfer_ownership() {
    only_owner();

    let new_owner: Key = runtime::get_named_arg(NEW);
    storage::write(get_uref(OWNER), new_owner);
    events::emit_event(Event::OwnershipTransferred(OwnershipTransferred {
        new_owner,
    }));
}

#[no_mangle]
pub extern "C" fn configure_minter() {
    when_not_paused();

    only_master_minter();

    let minter: Key = runtime::get_named_arg(MINTER);
    add_minter(minter);

    let minter_allowance = runtime::get_named_arg(MINTER_ALLOWED);
    set_minter_allowed(minter, minter_allowance);

    events::emit_event(Event::MinterConfigured(MinterConfigured {
        minter,
        minter_allowance,
    }));
}

#[no_mangle]
pub extern "C" fn remove_minter() {
    only_master_minter();

    let minter: Key = runtime::get_named_arg(MINTER);
    remove_minter_util(minter);
    set_minter_allowed(minter, U256::zero());
    events::emit_event(Event::MinterRemoved(MinterRemoved { minter }));
}

#[no_mangle]
pub extern "C" fn minter_allowance() {
    let minter: Key = runtime::get_named_arg(MINTER);
    let minter_allowance: U256 = read_minter_allowed(minter);

    runtime::ret(CLValue::from_t(minter_allowance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn is_minter() {
    let key: Key = runtime::get_named_arg(KEY);
    let is_minter: bool = is_minter_util(key);

    runtime::ret(CLValue::from_t(is_minter).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Key = runtime::get_named_arg(ADDRESS);
    let balance = balances::read_balance_from(get_balances_uref(), address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn allowance() {
    let spender: Key = runtime::get_named_arg(SPENDER);
    let owner: Key = runtime::get_named_arg(OWNER);
    let allowances_uref = get_allowances_uref();
    let val: U256 = read_allowance_from(allowances_uref, owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn approve() {
    when_not_paused();

    let owner: Key = utils::get_immediate_caller_address().unwrap_or_revert();
    let spender: Key = runtime::get_named_arg(SPENDER);
    if spender == owner {
        revert(CsprUSDError::CannotTargetSelfUser);
    }

    if is_blacklisted_util(owner) || is_blacklisted_util(spender) {
        revert(CsprUSDError::BlackListedAccount);
    }

    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let allowances_uref = get_allowances_uref();
    write_allowance_to(allowances_uref, owner, spender, amount);
    events::emit_event(Event::SetAllowance(SetAllowance {
        owner,
        spender,
        allowance: amount,
    }));
}

#[no_mangle]
pub extern "C" fn decrease_allowance() {
    when_not_paused();

    let owner: Key = utils::get_immediate_caller_address().unwrap_or_revert();
    let spender: Key = runtime::get_named_arg(SPENDER);
    if spender == owner {
        revert(CsprUSDError::CannotTargetSelfUser);
    }

    if is_blacklisted_util(owner) || is_blacklisted_util(spender) {
        revert(CsprUSDError::BlackListedAccount);
    }

    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let allowances_uref = get_allowances_uref();
    let current_allowance = read_allowance_from(allowances_uref, owner, spender);
    let new_allowance = current_allowance.saturating_sub(amount);
    write_allowance_to(allowances_uref, owner, spender, new_allowance);
    events::emit_event(Event::DecreaseAllowance(DecreaseAllowance {
        owner,
        spender,
        decr_by: amount,
        allowance: new_allowance,
    }));
}

#[no_mangle]
pub extern "C" fn increase_allowance() {
    when_not_paused();

    let owner: Key = utils::get_immediate_caller_address().unwrap_or_revert();
    let spender: Key = runtime::get_named_arg(SPENDER);
    if spender == owner {
        revert(CsprUSDError::CannotTargetSelfUser);
    }

    if is_blacklisted_util(owner) || is_blacklisted_util(spender) {
        revert(CsprUSDError::BlackListedAccount);
    }

    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let allowances_uref = get_allowances_uref();
    let current_allowance = read_allowance_from(allowances_uref, owner, spender);
    let new_allowance = current_allowance.saturating_add(amount);
    write_allowance_to(allowances_uref, owner, spender, new_allowance);
    events::emit_event(Event::IncreaseAllowance(IncreaseAllowance {
        owner,
        spender,
        allowance: new_allowance,
        inc_by: amount,
    }));
}

#[no_mangle]
pub extern "C" fn transfer() {
    when_not_paused();

    let sender: Key = utils::get_immediate_caller_address().unwrap_or_revert();
    let recipient: Key = runtime::get_named_arg(RECIPIENT);

    if is_blacklisted_util(sender) || is_blacklisted_util(recipient) {
        revert(CsprUSDError::BlackListedAccount);
    }

    if sender == recipient {
        revert(CsprUSDError::CannotTargetSelfUser);
    }

    let amount: U256 = runtime::get_named_arg(AMOUNT);
    if amount.is_zero() {
        revert(CsprUSDError::CannotTransferZeroAmount);
    }

    transfer_balance(sender, recipient, amount).unwrap_or_revert();
    events::emit_event(Event::Transfer(Transfer {
        sender,
        recipient,
        amount,
    }));
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    when_not_paused();

    let spender: Key = utils::get_immediate_caller_address().unwrap_or_revert();
    let recipient: Key = runtime::get_named_arg(RECIPIENT);
    let owner: Key = runtime::get_named_arg(OWNER);

    if is_blacklisted_util(spender) || is_blacklisted_util(recipient) || is_blacklisted_util(owner)
    {
        revert(CsprUSDError::BlackListedAccount);
    }

    if owner == recipient {
        revert(CsprUSDError::CannotTargetSelfUser);
    }

    let amount: U256 = runtime::get_named_arg(AMOUNT);
    if amount.is_zero() {
        revert(CsprUSDError::CannotTransferZeroAmount);
    }

    let allowances_uref = get_allowances_uref();
    let spender_allowance: U256 = read_allowance_from(allowances_uref, owner, spender);
    let new_spender_allowance = spender_allowance
        .checked_sub(amount)
        .ok_or(CsprUSDError::InsufficientAllowance)
        .unwrap_or_revert();

    transfer_balance(owner, recipient, amount).unwrap_or_revert();
    write_allowance_to(allowances_uref, owner, spender, new_spender_allowance);

    events::emit_event(Event::TransferFrom(TransferFrom {
        spender,
        owner,
        recipient,
        amount,
    }));
}

#[no_mangle]
pub extern "C" fn mint() {
    print("starting mint");
    when_not_paused();

    let minter: Key = get_immediate_caller_address().unwrap_or_revert();
    only_minters(minter);

    print("yay, I'm minter");

    if is_blacklisted_util(minter) {
        revert(CsprUSDError::BlackListedAccount);
    }

    print("yay, I'm not blacklsted");

    let recipient = runtime::get_named_arg(RECIPIENT);

    print("yay, got recipient key");

    if is_blacklisted_util(recipient) {
        revert(CsprUSDError::BlackListedAccount);
    }
    print("yay noone blacklisted");

    let amount: U256 = runtime::get_named_arg(AMOUNT);
    if amount == U256::zero() {
        revert(CsprUSDError::CannotMintZeroAmount);
    }

    let minter_allowance = read_minter_allowed(minter);
    if minter_allowance < amount {
        revert(CsprUSDError::ExceedsMintAllowance);
    }

    print("somewhere down");
    let balances_uref = get_balances_uref();
    let new_balance_recipient_account = {
        let balance = read_balance_from(balances_uref, recipient);
        balance
            .checked_add(amount)
            .ok_or(CsprUSDError::Overflow)
            .unwrap_or_revert()
    };
    write_balance_to(balances_uref, recipient, new_balance_recipient_account);

    // update minter allowance
    let updated_allowance = minter_allowance.checked_sub(amount).unwrap_or_revert();
    set_minter_allowed(minter, updated_allowance);

    let total_supply_uref = get_total_supply_uref();
    let new_total_supply = {
        let total_supply = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_add(amount)
            .ok_or(CsprUSDError::Overflow)
            .unwrap_or_revert()
    };
    write_total_supply_to(total_supply_uref, new_total_supply);

    events::emit_event(Event::Mint(Mint {
        minter,
        recipient,
        amount,
    }));
}

#[no_mangle]
pub extern "C" fn burn() {
    when_not_paused();

    let minter: Key = get_immediate_caller_address().unwrap_or_revert();
    only_minters(minter);

    if is_blacklisted_util(minter) {
        revert(CsprUSDError::BlackListedAccount);
    }

    let amount_to_burn: U256 = runtime::get_named_arg(AMOUNT);
    if amount_to_burn == U256::zero() {
        revert(CsprUSDError::CannotBurnZeroAmount);
    }

    let balances_uref = get_balances_uref();
    let minter_current_balance = read_balance_from(balances_uref, minter);

    if minter_current_balance < amount_to_burn {
        revert(CsprUSDError::BurnExceedsBalance);
    }

    let total_supply_uref = get_total_supply_uref();
    let new_total_supply = {
        let total_supply = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_sub(amount_to_burn)
            .ok_or(CsprUSDError::Overflow)
            .unwrap_or_revert()
    };
    write_total_supply_to(total_supply_uref, new_total_supply);

    let new_minter_balance = {
        let balance = read_balance_from(balances_uref, minter);
        balance
            .checked_sub(amount_to_burn)
            .ok_or(CsprUSDError::InsufficientBalance)
            .unwrap_or_revert()
    };

    write_balance_to(balances_uref, minter, new_minter_balance);
    events::emit_event(Event::Burn(Burn {
        minter,
        amount: amount_to_burn,
    }));
}

#[no_mangle]
pub extern "C" fn init() {
    let package_hash = runtime::get_named_arg::<Key>(PACKAGE_HASH);
    runtime::put_key(PACKAGE_HASH, package_hash);

    storage::new_dictionary(ALLOWANCES)
        .unwrap_or_revert_with(CsprUSDError::FailedToCreateDictionary);
    storage::new_dictionary(BALANCES).unwrap_or_revert_with(CsprUSDError::FailedToCreateDictionary);
    storage::new_dictionary(MINTERS).unwrap_or_revert_with(CsprUSDError::FailedToCreateDictionary);
    storage::new_dictionary(MINTER_ALLOWED)
        .unwrap_or_revert_with(CsprUSDError::FailedToCreateDictionary);

    storage::new_dictionary(DICT_INDEX_TO_BLACKLISTED_ADDR)
        .unwrap_or_revert_with(CsprUSDError::FailedToCreateDictionary);
    storage::new_dictionary(DICT_BLACKLISTED_ADDR_TO_INDEX)
        .unwrap_or_revert_with(CsprUSDError::FailedToCreateDictionary);

    let master_minter: Key = runtime::get_named_arg(MASTER_MINTER);
    add_minter(master_minter);

    init_events();
}

pub fn install_contract() {
    let name: String = runtime::get_named_arg(NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL);
    let currency: String = runtime::get_named_arg(CURRENCY);
    let decimals: u8 = runtime::get_named_arg(DECIMALS);
    let master_minter: Key = runtime::get_named_arg(MASTER_MINTER);
    let pauser: PublicKey = runtime::get_named_arg(PAUSER);
    let blacklister: PublicKey = runtime::get_named_arg(BLACKLISTER);
    let owner: Key = runtime::get_named_arg(OWNER);

    let mut named_keys = NamedKeys::new();
    named_keys.insert(NAME.to_string(), storage::new_uref(name).into());
    named_keys.insert(SYMBOL.to_string(), storage::new_uref(symbol).into());
    named_keys.insert(CURRENCY.to_string(), storage::new_uref(currency).into());
    named_keys.insert(DECIMALS.to_string(), storage::new_uref(decimals).into());
    named_keys.insert(
        MASTER_MINTER.to_string(),
        storage::new_uref(master_minter).into(),
    );
    named_keys.insert(IS_PAUSED.to_string(), storage::new_uref(false).into());
    named_keys.insert(PAUSER.to_string(), storage::new_uref(pauser).into());
    named_keys.insert(
        BLACKLISTER.to_string(),
        storage::new_uref(blacklister).into(),
    );
    named_keys.insert(OWNER.to_string(), storage::new_uref(owner).into());
    named_keys.insert(
        TOTAL_SUPPLY.to_string(),
        storage::new_uref(U256::zero()).into(),
    );

    named_keys.insert(
        BLACKLISTED_ADDRESSES_COUNT.to_string(),
        storage::new_uref(0u32).into(),
    );

    let entry_points = generate_entry_points();

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(CONTRACT_PACKAGE_HASH.to_string()),
        Some(CONTRACT_ACCESS.to_string()),
    );

    // Store contract_hash and contract_version under the keys CONTRACT_NAME and CONTRACT_VERSION
    runtime::put_key(CONTRACT_HASH, contract_hash.into());
    runtime::put_key(CONTRACT_VERSION, storage::new_uref(contract_version).into());

    // Call contract to initialize it
    let package_hash = runtime::get_key(CONTRACT_PACKAGE_HASH).unwrap_or_revert();
    let init_args = runtime_args! { MASTER_MINTER => master_minter, PACKAGE_HASH => package_hash};
    runtime::call_contract::<()>(contract_hash, INIT_ENTRY_POINT_NAME, init_args);
}

#[no_mangle]
pub extern "C" fn init_upgrade() {
    storage::new_dictionary(DICT_INDEX_TO_BLACKLISTED_ADDR)
        .unwrap_or_revert_with(CsprUSDError::FailedToCreateDictionary);
    storage::new_dictionary(DICT_BLACKLISTED_ADDR_TO_INDEX)
        .unwrap_or_revert_with(CsprUSDError::FailedToCreateDictionary);
}

pub fn upgrade_contract_blacklister_key_to_publickey() {
    print("Upgrading contract");
    // update blacklister
    let blacklister: PublicKey = runtime::get_named_arg(BLACKLISTER);
    let s = blacklister.to_hex();
    print(&s);

    let mut named_keys = NamedKeys::new();
    named_keys.insert(
        BLACKLISTER.to_string(),
        Key::URef(storage::new_uref(blacklister.clone())),
    );
    named_keys.insert(
        "random_key".to_string(),
        Key::URef(storage::new_uref(blacklister)),
    );
    print("added bll to named keys");

    let entry_points = generate_entry_points();

    // Get the counter package hash so we can upgrade the package.
    let counter_package_hash = runtime::get_key(CONTRACT_PACKAGE_HASH)
        .unwrap_or_revert()
        .into_hash()
        .unwrap()
        .into();

    let previous_contract_hash = runtime::get_key(CONTRACT_HASH)
        .unwrap_or_revert()
        .into_hash()
        .map(ContractHash::new)
        .unwrap_or_revert_with(CsprUSDError::AlreadyBlacklisted);

    let (contract_hash, contract_version) =
        storage::add_contract_version(counter_package_hash, entry_points, named_keys);
    print("upgraded contract");
    let package_hash = runtime::get_key(&CONTRACT_PACKAGE_HASH).unwrap_or_revert();

    storage::disable_contract_version(counter_package_hash, previous_contract_hash)
        .unwrap_or_revert();
    let disabled_hash = previous_contract_hash.to_formatted_string();
    print("Disabled contract with hasah:");
    print(&disabled_hash);

    // Store contract_hash and contract_version under the keys CONTRACT_NAME and CONTRACT_VERSION
    runtime::put_key(CONTRACT_HASH, contract_hash.into());
    runtime::put_key(CONTRACT_PACKAGE_HASH, package_hash.into());
    runtime::put_key(CONTRACT_VERSION, storage::new_uref(contract_version).into());

    runtime::call_contract::<()>(contract_hash, "init_upgrade", runtime_args! {});
}

#[no_mangle]
pub extern "C" fn call() {
    match runtime::get_key(CONTRACT_VERSION) {
        None => {
            // The given key doesn't exist, so install the contract.
            install_contract();
            // Next, upgrade the contract.
            upgrade_contract_blacklister_key_to_publickey();
        }
        Some(_contract_key) => {
            // The stored contract and key exist, so upgrade the contract.
            upgrade_contract_blacklister_key_to_publickey();
        }
    }
}
