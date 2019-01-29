use crate::models::data_structs::Key;
use web3::types::{Address, U256};
use ethereum_types::H256;
use web3::contract::Options;
use web3::futures::Future;

/** 
 * @dev Adds a _key to the identity. The _purpose specifies the purpose of key. Initially we propose four purposes
 * @param _key Struct Key containing the key to add info. 
 * @param from Address from 
 * @gas_price U256 Ammount og ethers to pay gor each Gwei
 * @return Key data structure that has been added and the tx_hash.
 */
pub fn add_key(_key: Key, _gas_price: &U256, from: &Address, contract: &web3::contract::Contract<web3::transports::http::Http>) -> Option<(Key,H256)> {
    let tx_hash: H256 = contract.call(&"addKey", (_key.key, _key.purpose, _key.key_type), *from, Options::with(|opt| {
        opt.gas_price= Some(*_gas_price);
    })).wait().expect("Failed to add key");
    let key_added = Key {
        key: _key.key,
        purpose:  _key.purpose,
        key_type:  _key.key_type
    };
    Some((key_added, tx_hash))
}

/** 
 * @dev Removes key from the identity.
 * @param from Address from 
 * @gas_price U256 Ammount og ethers to pay gor each Gwei
 * @param key key that want to delete.
 */
pub fn remove_key(_key: Key, _gas_price: &U256, from: &Address, contract: &web3::contract::Contract<web3::transports::http::Http>) -> Option<H256> {
    let txhash = contract.query(&"removeKey", _key.key, *from, Options::with(|opt| {
        opt.gas_price= Some(*_gas_price);
    }), None).wait().expect("Failed to remove key");
    Some(txhash)        
}

/**
 * @dev Returns the Keys with the passed purpose.
 * @param purpose purpose to check.
 * @return keys list of keys from one purpose.
 */
pub fn get_keys_by_purpose(from: &Address, purpose: &U256, contract: &web3::contract::Contract<web3::transports::http::Http>) -> Option<Vec<H256>> {
    let keys: Vec<H256> = contract.query(&"getKeysByPurpose", *purpose, *from, Options::default(), None).wait().unwrap();

    match keys.len() {
        0 => None,
        _ => Some(keys)      
    }
}

/**
 * @dev Returns the key data, if hold by the identity.
 * @param key keccack256 of original key.
 * @return Purpose e.g., MANAGEMENT_KEY = 1, ACTION_KEY = 2, CLAIM_SIGNER_KEY = 3, ENCRYPTION_KEY = 4;
 * @return keyType e.g. 1 = ECDSA, 2 = RSA, etc.
 * @return The key.
 */
pub fn get_key(from: &Address, decoded_key: H256, contract: &web3::contract::Contract<web3::transports::http::Http>) -> Option<Key> {
    let (a, b, c): (U256, U256, H256) = contract.query(&"getKey", decoded_key, *from, Options::default(), None).wait()
    .expect("Failed to retrieve the key");
    let response = Key {
        key: c,
        purpose: a,
        key_type: b 
    };
    Some(response)
}

/**
 * @dev Returns the TRUE if a key has is present and has the given purpose. If key is not present it returns FALSE.
 * @param key keccack256 of original key.
 * @param purpose purpose to check.
 * @return exist True if exist, False if not.
 */
pub fn key_has_purpose(from: &Address, decoded_key: H256, contract: &web3::contract::Contract<web3::transports::http::Http>) -> bool {
    let res: bool = contract.query(&"keyHasPurpose", decoded_key.clone(), *from, Options::default(), None).wait()
    .expect("Failed to retrieve the key");
    res
}