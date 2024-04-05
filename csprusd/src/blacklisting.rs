use crate::{
    constants::BLACKLISTED,
    utils::{get_uref, read_from},
};
use alloc::vec::Vec;
use casper_contract::contract_api::storage;
use casper_types::{Key, PublicKey};

pub(crate) fn is_blacklisted_util(account: Key) -> bool {
    let currently_blacklisted: Vec<PublicKey> = read_from(BLACKLISTED);
    if currently_blacklisted
        .iter()
        .any(|value| Key::Account((*value).to_account_hash()) == account)
    {
        return true;
    }
    false
}

pub(crate) fn blacklist_pubkey(pubkey: PublicKey) {
    let mut currently_blacklisted: Vec<PublicKey> = read_from(BLACKLISTED);

    currently_blacklisted.push(pubkey);

    let uref = get_uref(BLACKLISTED);
    storage::write(uref, currently_blacklisted);
}

pub(crate) fn un_blacklist_address(pubkey: PublicKey) {
    let mut currently_blacklisted: Vec<PublicKey> = read_from(BLACKLISTED);

    if let Some(index) = currently_blacklisted
        .iter()
        .position(|value| *value == pubkey)
    {
        currently_blacklisted.swap_remove(index);
    }

    let uref = get_uref(BLACKLISTED);
    storage::write(uref, currently_blacklisted);
}
