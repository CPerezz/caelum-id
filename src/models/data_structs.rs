use web3::types::{Address, U256, H256};

#[derive(Debug, Clone)]
pub struct Key {
    pub purpose: U256,
    pub key_type: U256,
    pub key: H256
}

#[derive(Debug, Clone)]
pub struct Claim {
    pub id: H256,
    pub topic: U256,
    pub scheme: U256,
    pub issuer: Address,
    pub signature: H256,
    pub data: H256,
    pub uri: H256
}