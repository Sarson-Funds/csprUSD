extern crate alloc;

use crate::{
    constants::{BLACKLISTER, IS_PAUSED, MASTER_MINTER, OWNER, PAUSER},
    error, minters, utils,
};

use casper_contract::{contract_api::runtime::revert, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{Key, PublicKey};

pub use error::CsprUSDError;
use utils::read_from;

use minters::is_minter_util;

pub(crate) fn only_pauser() {
    let caller: Key = utils::get_immediate_caller_address().unwrap_or_revert();
    let current_pauser = read_from::<Key>(PAUSER);

    if caller != current_pauser {
        revert(CsprUSDError::NotPauser);
    }
}

pub(crate) fn when_not_paused() {
    if read_from::<bool>(IS_PAUSED) {
        revert(CsprUSDError::ContractPaused);
    }
}

pub(crate) fn only_owner() {
    let caller: Key = utils::get_immediate_caller_address().unwrap_or_revert();
    let current_owner: Key = read_from::<Key>(OWNER);

    if caller != current_owner {
        revert(CsprUSDError::NotOwner);
    }
}

pub(crate) fn only_minters(account: Key) {
    if !is_minter_util(account) {
        revert(CsprUSDError::NotMinter);
    }
}

pub(crate) fn only_master_minter() {
    let caller: Key = utils::get_immediate_caller_address().unwrap_or_revert();
    let current_master_minter: Key = read_from::<Key>(MASTER_MINTER);

    if caller != current_master_minter {
        revert(CsprUSDError::NotMasterMinter);
    }
}

pub(crate) fn only_blacklister() {
    let caller: Key = utils::get_immediate_caller_address().unwrap_or_revert();

    let current_blacklister_pub_key: PublicKey = read_from::<PublicKey>(BLACKLISTER);
    let current_blacklister_acc_hash = Key::Account(current_blacklister_pub_key.to_account_hash());

    if caller != current_blacklister_acc_hash {
        revert(CsprUSDError::NotBlacklister);
    }
}
