use web3::types::{Address, U256, H256};
use web3::transports::http::Http;
use web3::futures::Future;
use web3;


/// Returns the node registered addresses
pub fn list_accounts(http: &Http) -> Vec<Address> {
    let web3 = web3::Web3::new(http);
    let accounts = web3.personal().list_accounts().wait().expect("Failed listing accounts");
    accounts
}

/// Creates an account given a password
pub fn create_account(password: &str, http: &Http) -> Address {
    let web3 = web3::Web3::new(http);
    let new_account = web3.personal().new_account(password).wait().expect("Failed on the account creation");
    new_account
}

/// Unlocks an account given an account and password. Time unlocking not supported, pass None as Option<u16>
/// this will unlock the account just for the next transaction.
pub fn unlock_account(address: &Address, password: &str, duration: Option<u16>, http: &Http) -> bool {
    let web3 = web3::Web3::new(http);
    let result = web3.personal().unlock_account(*address, password, duration).wait().expect("Failed to unlock the account");
    result
}

/// Send ether to another address
pub fn send_ehter(_from: &Address, _to: &Address, _value: &'static str, http: &Http, _pwd: &str) -> H256 {

    let web3 = web3::Web3::new(http); 
    let txreq = web3::types::TransactionRequest {
        from: *_from,
        to: Some(*_to),
        gas: None,
        gas_price: None,
        value: Some(U256::from(_value)),
        data: None,
        nonce: None,
        condition: None
    };
    let txid = web3.personal().send_transaction(txreq, _pwd).wait().expect("Failed on the tx sending.");
    txid
}