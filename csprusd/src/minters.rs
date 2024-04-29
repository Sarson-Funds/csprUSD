use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, Key, U256};

use crate::{
    constants::{MINTERS, MINTER_ALLOWED},
    utils::get_uref,
};

pub(crate) fn is_minter_util(account: Key) -> bool {
    let minters_dict_seed = get_uref(MINTERS);

    let minters_dict_key = hex::encode(runtime::blake2b(account.to_bytes().unwrap_or_revert()));
    let is_minter: Result<Option<bool>, _> =
        storage::dictionary_get(minters_dict_seed, &minters_dict_key);
    if let Ok(Some(is_minter)) = is_minter {
        return is_minter;
    }
    false
}

pub(crate) fn read_minter_allowed(minter: Key) -> U256 {
    let dict_seed = get_uref(MINTER_ALLOWED);
    let dict_key = hex::encode(runtime::blake2b(minter.to_bytes().unwrap_or_revert()));

    storage::dictionary_get(dict_seed, &dict_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}

pub(crate) fn set_minter_allowed(minter: Key, minter_allowed_amount: U256) {
    let dict_seed = get_uref(MINTER_ALLOWED);
    let dict_key = hex::encode(runtime::blake2b(minter.to_bytes().unwrap_or_revert()));

    storage::dictionary_put(dict_seed, &dict_key, minter_allowed_amount);
}

pub(crate) fn add_minter(minter: Key) {
    let dict_seed = get_uref(MINTERS);
    let dict_key = hex::encode(runtime::blake2b(minter.to_bytes().unwrap_or_revert()));

    storage::dictionary_put(dict_seed, &dict_key, true);
}

pub(crate) fn remove_minter_util(minter: Key) {
    let dict_seed = get_uref(MINTERS);
    let dict_key = hex::encode(runtime::blake2b(minter.to_bytes().unwrap_or_revert()));

    storage::dictionary_put(dict_seed, &dict_key, false);
}
