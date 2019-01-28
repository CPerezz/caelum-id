use web3::contract::{Contract, Options};
use web3::types::{Address, U256};
use web3::transports::Http;
use rustc_hex::FromHex;
use web3::futures::Future;
use web3;

pub fn deploy_identity(from: &Address, _gas_price: &U256, http: Http ) -> Option<web3::contract::Contract<web3::transports::Http>> {
    //Charge paths to ABI and Bytecode from .env
    
    //Generating the web3 object to be able to use the contract.
    let web3 = web3::Web3::new(http);

    //Importing bytecode
     let bytecode: Vec<u8> = include_str!("../compiled_sc/Identitat.bin").from_hex().unwrap();
    //Deploying a contract
    let contract = Contract::deploy(web3.eth(), include_bytes!("../compiled_sc/Identitat.abi"))
        .unwrap()
        .confirmations(0)
        .options(Options::with(|opt| {
            opt.gas = Some(7_000_000.into());
            opt.gas_price = Some(*_gas_price);
        }))
        .execute(
            bytecode,
            (),
            *from,
        )
        .expect("Correct parameters aren't passed to the constructor.")
        .wait()
        .unwrap();
        Some(contract)
}

pub fn gen_identity_at_address(_address: &Address, http: Http) -> web3::contract::Contract<web3::transports::Http> {
    let web3 = web3::Web3::new(http.clone());
    let contract = Contract::from_json(
        web3.eth(),
        _address.clone(),
        include_bytes!("../compiled_sc/Identitat.json"),
    ).expect("Error on the contract Generation");
    contract
}