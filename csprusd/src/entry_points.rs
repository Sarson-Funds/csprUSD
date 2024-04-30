//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};

use casper_types::{
    account::AccountHash, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType,
    EntryPoints, Key, Parameter, PublicKey, U256,
};

use crate::constants::{
    ADDRESS, ALLOWANCE_ENTRY_POINT_NAME, AMOUNT, APPROVE_ENTRY_POINT_NAME,
    BALANCE_OF_ENTRY_POINT_NAME, BLACKLISTER_ENTRY_POINT_NAME, BLACKLIST_ENTRY_POINT_NAME,
    BURN_ENTRY_POINT_NAME, CONFIGURE_MINTER_ENTRY_POINT_NAME, DECIMALS_ENTRY_POINT_NAME,
    DECREASE_ALLOWANCE_ENTRY_POINT_NAME, INCREASE_ALLOWANCE_ENTRY_POINT_NAME,
    INIT_ENTRY_POINT_NAME, IS_BLACKLISTED_ENTRY_POINT_NAME, IS_MINTER_ENTRY_POINT_NAME,
    IS_PAUSED_ENTRY_POINT_NAME, KEY, MASTER_MINTER, MASTER_MINTER_ENTRY_POINT_NAME, MINTER,
    MINTER_ALLOWANCE_ENTRY_POINT_NAME, MINTER_ALLOWED, MINT_ENTRY_POINT_NAME,
    NAME_ENTRY_POINT_NAME, NEW, OWNER, OWNER_ENTRY_POINT_NAME, PACKAGE_HASH,
    PAUSER_ENTRY_POINT_NAME, PAUSE_ENTRY_POINT_NAME, RECIPIENT, REMOVE_MINTER_ENTRY_POINT_NAME,
    SPENDER, SYMBOL_ENTRY_POINT_NAME, TOTAL_SUPPLY_ENTRY_POINT_NAME, TRANSFER_ENTRY_POINT_NAME,
    TRANSFER_FROM_ENTRY_POINT_NAME, TRANSFER_OWNERSHIP_ENTRY_POINT_NAME, UNPAUSE_ENTRY_POINT_NAME,
    UN_BLACKLIST_ENTRY_POINT_NAME, UPDATE_BLACKLISTER_ENTRY_POINT_NAME,
    UPDATE_MASTER_MINTER_ENTRY_POINT_NAME, UPDATE_PAUSER_ENTRY_POINT_NAME,
};

/// Returns the `name` entry point.
pub fn name() -> EntryPoint {
    EntryPoint::new(
        String::from(NAME_ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `symbol` entry point.
pub fn symbol() -> EntryPoint {
    EntryPoint::new(
        String::from(SYMBOL_ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn pauser() -> EntryPoint {
    EntryPoint::new(
        String::from(PAUSER_ENTRY_POINT_NAME),
        Vec::new(),
        PublicKey::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn is_paused() -> EntryPoint {
    EntryPoint::new(
        String::from(IS_PAUSED_ENTRY_POINT_NAME),
        Vec::new(),
        bool::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn owner() -> EntryPoint {
    EntryPoint::new(
        String::from(OWNER_ENTRY_POINT_NAME),
        Vec::new(),
        AccountHash::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn master_minter() -> EntryPoint {
    EntryPoint::new(
        String::from(MASTER_MINTER_ENTRY_POINT_NAME),
        Vec::new(),
        AccountHash::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn blacklister() -> EntryPoint {
    EntryPoint::new(
        String::from(BLACKLISTER_ENTRY_POINT_NAME),
        Vec::new(),
        PublicKey::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `transfer_from` entry point.
pub fn transfer_from() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_FROM_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER, Key::cl_type()),
            Parameter::new(RECIPIENT, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `allowance` entry point.
pub fn allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(ALLOWANCE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER, Key::cl_type()),
            Parameter::new(SPENDER, Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `approve` entry point.
pub fn approve() -> EntryPoint {
    EntryPoint::new(
        String::from(APPROVE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SPENDER, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `increase_allowance` entry point.
pub fn increase_allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(INCREASE_ALLOWANCE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SPENDER, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `decrease_allowance` entry point.
pub fn decrease_allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(DECREASE_ALLOWANCE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SPENDER, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `transfer` entry point.
pub fn transfer() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_ENTRY_POINT_NAME),
        vec![
            Parameter::new(RECIPIENT, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `balance_of` entry point.
pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        String::from(BALANCE_OF_ENTRY_POINT_NAME),
        vec![Parameter::new(ADDRESS, Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `total_supply` entry point.
pub fn total_supply() -> EntryPoint {
    EntryPoint::new(
        String::from(TOTAL_SUPPLY_ENTRY_POINT_NAME),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `decimals` entry point.
pub fn decimals() -> EntryPoint {
    EntryPoint::new(
        String::from(DECIMALS_ENTRY_POINT_NAME),
        Vec::new(),
        u8::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `burn` entry point.
pub fn burn() -> EntryPoint {
    EntryPoint::new(
        String::from(BURN_ENTRY_POINT_NAME),
        vec![Parameter::new(AMOUNT, U256::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `mint` entry point.
pub fn mint() -> EntryPoint {
    EntryPoint::new(
        String::from(MINT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(RECIPIENT, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `init` entry point.
pub fn init() -> EntryPoint {
    EntryPoint::new(
        String::from(INIT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(PACKAGE_HASH, Key::cl_type()),
            Parameter::new(MASTER_MINTER, Key::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn pause_contract() -> EntryPoint {
    EntryPoint::new(
        String::from(PAUSE_ENTRY_POINT_NAME),
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn unpause_contract() -> EntryPoint {
    EntryPoint::new(
        String::from(UNPAUSE_ENTRY_POINT_NAME),
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn update_pauser() -> EntryPoint {
    EntryPoint::new(
        String::from(UPDATE_PAUSER_ENTRY_POINT_NAME),
        vec![Parameter::new(NEW, PublicKey::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn transfer_ownership() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_OWNERSHIP_ENTRY_POINT_NAME),
        vec![Parameter::new(NEW, Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn configure_minter() -> EntryPoint {
    EntryPoint::new(
        String::from(CONFIGURE_MINTER_ENTRY_POINT_NAME),
        vec![
            Parameter::new(MINTER, Key::cl_type()),
            Parameter::new(MINTER_ALLOWED, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn remove_minter() -> EntryPoint {
    EntryPoint::new(
        String::from(REMOVE_MINTER_ENTRY_POINT_NAME),
        vec![Parameter::new(MINTER, Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn minter_allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(MINTER_ALLOWANCE_ENTRY_POINT_NAME),
        vec![Parameter::new(MINTER, Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn is_minter() -> EntryPoint {
    EntryPoint::new(
        String::from(IS_MINTER_ENTRY_POINT_NAME),
        vec![Parameter::new(KEY, Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn is_blacklisted() -> EntryPoint {
    EntryPoint::new(
        String::from(IS_BLACKLISTED_ENTRY_POINT_NAME),
        vec![Parameter::new(KEY, Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn blacklist() -> EntryPoint {
    EntryPoint::new(
        String::from(BLACKLIST_ENTRY_POINT_NAME),
        vec![Parameter::new(KEY, Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn un_blacklist() -> EntryPoint {
    EntryPoint::new(
        String::from(UN_BLACKLIST_ENTRY_POINT_NAME),
        vec![Parameter::new(KEY, Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn update_blacklister() -> EntryPoint {
    EntryPoint::new(
        String::from(UPDATE_BLACKLISTER_ENTRY_POINT_NAME),
        vec![Parameter::new(NEW, PublicKey::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn update_master_minter() -> EntryPoint {
    EntryPoint::new(
        String::from(UPDATE_MASTER_MINTER_ENTRY_POINT_NAME),
        vec![Parameter::new(NEW, Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of cspr USD token entry points.
pub fn generate_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init());
    entry_points.add_entry_point(name());
    entry_points.add_entry_point(symbol());
    entry_points.add_entry_point(decimals());
    entry_points.add_entry_point(pauser());
    entry_points.add_entry_point(is_paused());
    entry_points.add_entry_point(owner());
    entry_points.add_entry_point(master_minter());
    entry_points.add_entry_point(blacklister());
    entry_points.add_entry_point(total_supply());
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(transfer());
    entry_points.add_entry_point(approve());
    entry_points.add_entry_point(allowance());
    entry_points.add_entry_point(decrease_allowance());
    entry_points.add_entry_point(increase_allowance());
    entry_points.add_entry_point(transfer_from());
    entry_points.add_entry_point(burn());
    entry_points.add_entry_point(mint());
    entry_points.add_entry_point(pause_contract());
    entry_points.add_entry_point(unpause_contract());
    entry_points.add_entry_point(update_pauser());
    entry_points.add_entry_point(transfer_ownership());
    entry_points.add_entry_point(configure_minter());
    entry_points.add_entry_point(remove_minter());
    entry_points.add_entry_point(minter_allowance());
    entry_points.add_entry_point(is_minter());
    entry_points.add_entry_point(is_blacklisted());
    entry_points.add_entry_point(blacklist());
    entry_points.add_entry_point(un_blacklist());
    entry_points.add_entry_point(update_blacklister());
    entry_points.add_entry_point(update_master_minter());

    entry_points
}
