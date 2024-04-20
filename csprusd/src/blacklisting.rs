use crate::{
    constants::{BLACKLISTED_DICT, BLACKLISTED_LIST},
    utils::{get_uref, read_from},
};
use alloc::vec::Vec;
use casper_types::{bytesrepr::ToBytes, Key, PublicKey};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

pub(crate) fn is_blacklisted_util(account: Key) -> bool {
    let dict_seed = get_uref(BLACKLISTED_DICT);
    let dict_key = hex::encode(runtime::blake2b(account.to_bytes().unwrap_or_revert()));

    let is_blacklisted: Result<Option<bool>, _> = storage::dictionary_get(dict_seed, &dict_key);
    if let Ok(Some(is_blacklisted)) = is_blacklisted {
        return is_blacklisted;
    }
    false
}

pub(crate) fn blacklist_pubkey(pubkey: PublicKey) {
    // insert into BLACKLISTED_LIST
    let mut currently_blacklisted: Vec<PublicKey> = read_from(BLACKLISTED_LIST);
    currently_blacklisted.push(pubkey.clone());
    let uref = get_uref(BLACKLISTED_LIST);
    storage::write(uref, currently_blacklisted);

    // insert into BLACKLISTED_DICT
    let dict_seed = get_uref(BLACKLISTED_DICT);
    let key = Key::Account(pubkey.to_account_hash());
    let dict_key = hex::encode(runtime::blake2b(key.to_bytes().unwrap_or_revert()));
    storage::dictionary_put(dict_seed, &dict_key, true);
}

pub(crate) fn un_blacklist_address(pubkey: PublicKey) {
    // remove from BLACKLISTED_LIST
    let mut currently_blacklisted: Vec<PublicKey> = read_from(BLACKLISTED_LIST);
    if let Some(index) = currently_blacklisted
        .iter()
        .position(|value| *value == pubkey)
    {
        currently_blacklisted.swap_remove(index);
    }
    let uref = get_uref(BLACKLISTED_LIST);
    storage::write(uref, currently_blacklisted);

    // remove from BLACKLISTED_DICT
    let dict_seed = get_uref(BLACKLISTED_DICT);
    let key = Key::Account(pubkey.to_account_hash());
    let dict_key = hex::encode(runtime::blake2b(key.to_bytes().unwrap_or_revert()));
    storage::dictionary_put(dict_seed, &dict_key, false);
}
