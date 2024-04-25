use crate::{
    constants::{
        BLACKLISTED_ADDRESSES_COUNT, DICT_BLACKLISTED_ADDR_TO_INDEX, DICT_INDEX_TO_BLACKLISTED_ADDR,
    },
    utils::{get_uref, read_from},
    CsprUSDError,
};
use alloc::string::{String, ToString};
use casper_types::{bytesrepr::ToBytes, Key, URef};

use casper_contract::{
    contract_api::{
        runtime::{self, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};

/// BLACKLISTED_ADDR_TO_INDEX is used to look up in O(1) whether an address/key is blacklisted or
/// INDEX_TO_BLACKLISTED_ADDR is a way to get the list of blacklisted accounts using
/// String -> u32
/// String -> String

pub(crate) fn is_blacklisted_util(key: Key) -> bool {
    let dict_atoi: URef = get_uref(DICT_BLACKLISTED_ADDR_TO_INDEX);
    let key_blake: String = hex::encode(runtime::blake2b(key.to_bytes().unwrap_or_revert()));

    let id: Option<u32> = storage::dictionary_get(dict_atoi, &key_blake).unwrap_or_revert();
    if let Some(id) = id {
        return id != 0;
    }
    false
}

pub(crate) fn blacklist_key(key: Key) {
    let key_blake: String = hex::encode(runtime::blake2b(key.to_bytes().unwrap_or_revert()));

    // revert if already blacklisted
    let dict_atoi: URef = get_uref(DICT_BLACKLISTED_ADDR_TO_INDEX);
    let id: Option<u32> = storage::dictionary_get(dict_atoi, &key_blake).unwrap_or_revert();
    if let Some(id) = id {
        if id != 0 {
            revert(CsprUSDError::AlreadyBlacklisted);
        }
    }

    let blacklist_count: u32 = read_from(BLACKLISTED_ADDRESSES_COUNT);
    let new_index: u32 = blacklist_count + 1;

    // insert into BLACKLISTED_ADDR_TO_INDEX
    storage::dictionary_put(dict_atoi, &key_blake, new_index);

    // insert into INDEX_TO_BLACKLISTED_ADDR
    let dict_itoa = get_uref(DICT_INDEX_TO_BLACKLISTED_ADDR);
    let new_index_str = new_index.to_string();
    storage::dictionary_put(dict_itoa, &new_index_str, key);

    // increment BLACKLISTED_ADDRESSES_COUNT
    let uref = get_uref(BLACKLISTED_ADDRESSES_COUNT);
    storage::write(uref, new_index);
}

pub(crate) fn un_blacklist_address(key: Key) {
    // get BLACKLISTED_ADDRESSES_COUNT
    let mut blacklist_count: u32 = read_from(BLACKLISTED_ADDRESSES_COUNT);

    let key_blake: String = hex::encode(runtime::blake2b(key.to_bytes().unwrap_or_revert()));
    let dict_atoi: URef = get_uref(DICT_BLACKLISTED_ADDR_TO_INDEX);

    // continue only if currently blacklisted
    let id: Option<u32> = storage::dictionary_get(dict_atoi, &key_blake).unwrap_or_revert();
    if let Some(id) = id {
        if id == 0 {
            revert(CsprUSDError::NotBlacklisted);
        }
        storage::dictionary_put(dict_atoi, &key_blake, 0);
        if id < blacklist_count {
            // unblacklisted address is not the address with last index
            let dict_itoa: URef = get_uref(DICT_INDEX_TO_BLACKLISTED_ADDR);
            let last_addr: Key = storage::dictionary_get(dict_itoa, &blacklist_count.to_string())
                .unwrap_or_revert()
                .unwrap_or_revert();
            storage::dictionary_put(dict_itoa, &id.to_string(), last_addr);
            let last_addr_blake =
                hex::encode(runtime::blake2b(last_addr.to_bytes().unwrap_or_revert()));
            storage::dictionary_put(dict_atoi, &last_addr_blake, id);
        }
    } else {
        revert(CsprUSDError::NotBlacklisted);
    }

    // decrement BLACKLISTED_ADDRESSES_COUNT
    blacklist_count -= 1;
    let uref = get_uref(BLACKLISTED_ADDRESSES_COUNT);
    storage::write(uref, blacklist_count);
}
