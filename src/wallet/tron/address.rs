use std::str::FromStr;

use bitcoin::{bip32::Xpub, PublicKey,base58};
use log::info;
use serde::{Deserialize, Serialize};
use sha256::digest;

use crate::{error::Error, utils::key::keccak_hash};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TronAddr(String);

impl TronAddr {
    pub fn new(addr: &str) -> Result<Self, Error> {
        let mut proper_addr = addr.to_owned();
        //check for 0x prefix
        if !addr.starts_with("0x") {
            proper_addr = format!("0x{}", addr);
        }
        //check that passed str is a hex string
        hex::decode(&proper_addr[2..]).map_err(|e| {
            info!("String passed into TronAddr is not base58.");
            e
        })?;
        //check length
        if proper_addr.len() != 42 {
            return Err(Error::EthAddrLengthError(proper_addr.len()));
        }
        //checksum and return
        let checksummed_addr = eth_checksum::checksum(&proper_addr);
        Ok(Self(checksummed_addr))
    }
    pub fn get(&self) -> &str {
        &self.0
    }
}


pub fn extended_pubk_to_addr_tron(pubk: &Xpub) -> Result<String, Error> {
    //massage into the right format
    let pubk_str = pubk.public_key.to_string();
    let pubk_secp = secp256k1::PublicKey::from_str(&pubk_str)?;
    //format as uncompressed key, remove "04" in the beginning
    let pubk_uncomp = &PublicKey::new_uncompressed(pubk_secp).to_string()[2..];
    //decode from hex and pass to keccak for hashing
    let pubk_bytes = hex::decode(pubk_uncomp)?;
    let k_addr = &keccak_hash(&pubk_bytes);
    //keep last 20 bytes of the result
    let experimental_addr = "41".to_owned() + &k_addr[24..];
    let hex_exp_addr = hex::decode(&experimental_addr)?;
    let s_hex_exp_addr = hex_exp_addr.as_slice();
    let val0 = digest(s_hex_exp_addr);
    let hex_val0 = hex::decode(val0)?;
    let s_hex_val0 = hex_val0.as_slice();
    let val1 = digest(s_hex_val0);
    let check_sum_val1 = &val1[0..8];
    let final_addr = experimental_addr + check_sum_val1;
    let final_addr_bytes = hex::decode(final_addr)?;

    Ok(base58::encode(&final_addr_bytes))
}
