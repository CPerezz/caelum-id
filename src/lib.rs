extern crate web3;
extern crate rustc_hex;
extern crate tiny_keccak;
extern crate ethereum_types;


pub mod utils;
pub mod controllers;
pub mod models;

/*
#[cfg(test)]
mod deployment_tests {
    use crate::utils;
    #[test]
    fn it_deploys() {
        let _gas_price = web3::types::U256::from("1000000000");
        let _from = web3::types::Address::from("0x00a329c0648769a73afac7f9381e08fb43dbea72");
        let (loop_hand ,_http) = web3::transports::Http::new("http://localhost:8545").expect("Not connected to an RPC Client."); 
        utils::wallet::unlock_account(&_from, "", None, &_http);
        let contract = utils::deploy::deploy_identity(&_from, &_from, &_gas_price, _http)
        .expect("Failed to deploy indentity, Review the parameters.");
        assert!(contract.address().to_string() != web3::types::Address::from("0000000000000000000000000000000000000000").to_string());
    }
    #[test]
    fn it_generates_contract_at_addr() {
        let _gas_price = web3::types::U256::from("1000000000");
        let _from = web3::types::Address::from("0x00a329c0648769a73afac7f9381e08fb43dbea72");
        let (loop_hand ,_http) = web3::transports::Http::new("http://localhost:8545").expect("Not connected to an RPC Client."); 
        utils::wallet::unlock_account(&_from, "", None, &_http);
        let deployed_contract = utils::deploy::deploy_identity(&_from, &_from, &_gas_price, _http.clone())
        .expect("Failed to deploy indentity, Review the parameters.");
        let instanciated_contract = utils::deploy::gen_identity_at_address(&web3::types::Address::from(utils::tools::string_to_static_str(deployed_contract.address().hex())), _http.clone());
        assert_eq!(deployed_contract.address(), instanciated_contract.address());
    }
}

#[cfg(test)]
mod key_tests {
    use crate::utils;
    use crate::models;
    use crate::controllers::keys;
    use web3::types::{H256, Address, U256};
    use crate::utils::tools;

    #[test]
    #[ignore]
    fn it_gets_keys_bypurpose() {
        let _gas_price = web3::types::U256::from("1000000000");
        let _from = web3::types::Address::from("0x00a329c0648769a73afac7f9381e08fb43dbea72");
        let purpose = web3::types::U256::from("1");
        let (loop_hand ,_http) = web3::transports::Http::new("http://localhost:8545").expect("Not connected to an RPC Client."); 
        utils::wallet::unlock_account(&_from, "", None, &_http);
        let deployed_contract = utils::deploy::deploy_identity(&_from, &_from, &_gas_price, _http.clone())
        .expect("Failed to deploy indentity, Review the parameters.");
        let keys: Vec<H256> = keys::get_keys_by_purpose(&_from, &purpose, &deployed_contract).expect("Error on getting the keys");
        let keccaked_key = utils::tools::keccak256(&H256::from(Address::from(_from)));
        assert_eq!(keys[0], keccaked_key);
    }

    #[test]
    #[ignore]
    fn it_gets_keys() {
        let _gas_price = web3::types::U256::from("1000000000");
        let _from = web3::types::Address::from("0x00a329c0648769a73afac7f9381e08fb43dbea72");
        let (loop_hand ,_http) = web3::transports::Http::new("http://localhost:8545").expect("Not connected to an RPC Client."); 
        utils::wallet::unlock_account(&_from, "", None, &_http);
        let deployed_contract = utils::deploy::deploy_identity(&_from, &_from, &_gas_price, _http.clone())
        .expect("Failed to deploy indentity, Review the parameters.");
        let _key: models::data_structs::Key = keys::get_key(&_from, H256::from([102, 98, 193, 58, 124, 52, 75, 143, 140, 12, 61, 118, 106, 139, 108, 166, 10, 18, 162, 92, 150, 32, 132, 83, 185, 187, 22, 61, 108, 142, 203, 86]), &deployed_contract).expect("Error getting the key");
        assert_eq!(_key.key, H256::from([102, 98, 193, 58, 124, 52, 75, 143, 140, 12, 61, 118, 106, 139, 108, 166, 10, 18, 162, 92, 150, 32, 132, 83, 185, 187, 22, 61, 108, 142, 203, 86]));
    }

    #[test]
    fn it_adds_key() {
        let _gas_price = web3::types::U256::from("1000000000");
        let _from = web3::types::Address::from("0x00a329c0648769a73afac7f9381e08fb43dbea72");
        let (_ ,_http) = web3::transports::Http::new("http://localhost:8545").expect("Not connected to an RPC Client."); 
        utils::wallet::unlock_account(&_from, "", None, &_http);
        let deployed_contract = utils::deploy::deploy_identity(&_from, &_from, &_gas_price, _http.clone())
        .expect("Failed to deploy indentity, Review the parameters.");
        let _key = models::data_structs::Key {
            key: H256::from([101, 98, 193, 58, 124, 52, 75, 143, 140, 12, 61, 118, 106, 139, 108, 166, 10, 18, 162, 92, 150, 32, 132, 83, 185, 187, 22, 61, 108, 142, 203, 86]),
            purpose: U256::from("2"),
            key_type: U256::from("2")
        };
        let added = keys::add_key(_key.clone(), &_gas_price, &_from, &deployed_contract).unwrap();
        assert_ne!(added.0.key, H256::from(0));
    }
}

#[cfg(test)]
mod claim_tests {
    use crate::utils;
    use crate::models;
    use crate::controllers::{claims, keys};
    use ethereum_types::{H256, U256, Address};

    #[test]
    fn it_adds_claims() {
        let _gas_price = web3::types::U256::from("1000000000");
        let _from = web3::types::Address::from("0x00a329c0648769a73afac7f9381e08fb43dbea72");
        let (loop_hand ,_http) = web3::transports::Http::new("http://localhost:8545").expect("Not connected to an RPC Client."); 
        utils::wallet::unlock_account(&_from, "", None, &_http);
        let deployed_contract = utils::deploy::deploy_identity(&_from, &_from, &_gas_price, _http.clone())
        .expect("Failed to deploy indentity, Review the parameters.");

        let _key = models::data_structs::Key {
            key: H256::from(Address::from("0x00a329c0648769a73afac7f9381e08fb43dbea72")),//Always as bytes, &str not supported.
            purpose: U256::from("3"),
            key_type: U256::from("1")
        };

        keys::add_key(_key.clone(), &_gas_price, &_from, &deployed_contract);
        //assert_eq!(added.unwrap(), true);
        utils::wallet::unlock_account(&_from, "", None, &_http);
        let _claim = models::data_structs::Claim {
            id: H256::zero(),
            topic: U256::from("50"),
            scheme: U256::from("1"),
            issuer: Address::zero(),
            signature: H256::zero(),
            data: H256::zero(),
            uri: H256::zero()
        }; 
        match claims::add_claim(_claim, &_gas_price, &_from, &deployed_contract) {
            None => panic!("Claim couldn't be added."),
            Some(claim_id) =>  assert_ne!(claim_id, H256::zero())
        };
    }
}
*/