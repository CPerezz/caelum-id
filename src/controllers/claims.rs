use crate::models::data_structs::Claim;
use web3::types::{Address, U256, H256};
use web3::contract::Options;
use web3::futures::Future;


/** 
 * @dev Requests the ADDITION of a claim from an issuer.
 * @param Claim Struct to add
 * @param from Address from 
 * @gas_price U256 Ammount og ethers to pay gor each Gwei
 * @return claim_id Unique identifier for this transaction,
 */
pub fn add_claim(_claim: Claim, _gas_price: &U256, from: &Address, contract: &web3::contract::Contract<web3::transports::Http>) -> Option<H256> {
    let tx_hash: H256 = contract.call(&"addClaim", (_claim.topic, _claim.scheme, _claim.signature, _claim.data, _claim.uri ), *from, Options::with(|opt| {
        opt.gas_price= Some(*_gas_price);
    })).wait().expect("Failed to add claim");
    println!("Claim added. Tx_hash: {:?}", tx_hash);
    Some(tx_hash)
}

/**
 * @dev Removes a claim. Can only be removed by the claim issuer, or the claim holder itself.
 * @param Claim Struct to add
 * @param from Address from 
 * @gas_price U256 Ammount og ethers to pay gor each Gwei
 */
pub fn remove_claim(_claim: Claim, _gas_price: &U256, from: &Address, contract: &web3::contract::Contract<web3::transports::Http>) -> Option<H256> {
    let tx_receipt: H256 = contract.call(&"removeClaim", _claim.id, *from, Options::with(|opt| {
        opt.gas_price= Some(*_gas_price);
    })).wait().expect("Failed to add claim");
    
    Some(tx_receipt) 
}

/** 
 * @dev Return a list of all claims with same topic
 * @param topic topic we want to check
 * @return claimIds list of the claims with the same topic
 */
pub fn get_claim(from: &Address, _claim: Claim, contract: &web3::contract::Contract<web3::transports::Http>) -> Option<Claim> {
    let (_topic, _scheme, _issuer, _signature, _data): (U256, U256, Address, H256, H256) =
    contract.query(&"getClaim", _claim.id, *from, Options::default(), None).wait().expect("Failed to retrieve Claim");

    let claim = Claim {
        id: _claim.id,
        topic : _topic,
        scheme : _scheme,
        issuer: _issuer,
        signature : _signature,
        data : _data,
        uri : H256::zero()
    };
    Some(claim)
}