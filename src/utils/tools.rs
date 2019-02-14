/*use tiny_keccak::Keccak;
use ethereum_types::H256;
/*
*As of Rust version 1.26, it is possible to convert a String
*to &'static str without using unsafe code.
*This converts the String instance into a boxed str and immediately leaks it.
*This frees all excess capacity the string may currently occupy.
*/
pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

/// Currently being implemented
pub fn keccak256(_val: &[u8]) -> H256 {
    let mut sha3 = Keccak::new_sha3_256();
    let data2: Vec<u8> = _val.to_vec();
    sha3.update(&data2);
    let mut res: [u8; 32] = [0; 32];
    sha3.finalize(&mut res);
    H256::from(res)
}
*/
