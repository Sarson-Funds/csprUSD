use crate::{
    constants::BLACKLISTED,
    utils::{get_uref, read_from},
};
use alloc::vec::Vec;
use casper_contract::contract_api::storage;
use casper_types::Key;

pub(crate) fn is_blacklisted_util(account: Key) -> bool {
    let currently_blacklisted: Vec<Key> = read_from(BLACKLISTED);

    if currently_blacklisted.iter().any(|value| *value == account) {
        return true;
    }
    false
}

pub(crate) fn blacklist_address(address: Key) {
    let mut currently_blacklisted: Vec<Key> = read_from(BLACKLISTED);

    currently_blacklisted.push(address);

    let uref = get_uref(BLACKLISTED);
    storage::write(uref, currently_blacklisted);
}

pub(crate) fn un_blacklist_address(address: Key) {
    let mut currently_blacklisted: Vec<Key> = read_from(BLACKLISTED);

    if let Some(index) = currently_blacklisted
        .iter()
        .position(|value| *value == address)
    {
        currently_blacklisted.swap_remove(index);
    }

    let uref = get_uref(BLACKLISTED);
    storage::write(uref, currently_blacklisted);
}
