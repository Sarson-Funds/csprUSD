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
    let atoi: URef = get_uref(DICT_BLACKLISTED_ADDR_TO_INDEX);
    let key_blake: String = hex::encode(runtime::blake2b(key.to_bytes().unwrap_or_revert()));

    let id: Option<u32> = storage::dictionary_get(atoi, &key_blake).unwrap_or_revert();
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

    let mut blacklist_count: u32 = read_from(BLACKLISTED_ADDRESSES_COUNT);

    // insert into BLACKLISTED_ADDR_TO_INDEX
    storage::dictionary_put(dict_atoi, &key_blake, blacklist_count);

    // insert into INDEX_TO_BLACKLISTED_ADDR
    let dict_itoa = get_uref(DICT_INDEX_TO_BLACKLISTED_ADDR);
    let key_str: String = hex::encode(key.to_bytes().unwrap_or_revert());
    let blacklist_count_str = blacklist_count.to_string();
    storage::dictionary_put(dict_itoa, &blacklist_count_str, key_str);

    // increment BLACKLISTED_ADDRESSES_COUNT
    blacklist_count += 1;
    let uref = get_uref(BLACKLISTED_ADDRESSES_COUNT);
    storage::write(uref, blacklist_count);
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
        if id < blacklist_count - 1 {
            // not address with last index
            let last_index: u32 = blacklist_count - 1;

            let dict_itoa: URef = get_uref(DICT_INDEX_TO_BLACKLISTED_ADDR);
            let last_addr: String = storage::dictionary_get(dict_itoa, &last_index.to_string())
                .unwrap_or_revert()
                .unwrap_or_revert();
            storage::dictionary_put(dict_itoa, &id.to_string(), last_addr.clone());
            let last_addr_blake = hex::encode(runtime::blake2b(hex::decode(last_addr).unwrap()));
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
