use std::str::FromStr;

use bitcoin::{bip32::Xpub, PublicKey};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{error::Error, utils::key::keccak_hash};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EthAddr(String);

impl EthAddr {
    pub fn new(addr: &str) -> Result<Self, Error> {
        let mut proper_addr = addr.to_owned();
        //check for 0x prefix
        if !addr.starts_with("0x") {
            proper_addr = format!("0x{}", addr);
        }
        //check that passed str is a hex string
        hex::decode(&proper_addr[2..]).map_err(|e| {
            info!("String passed into EthAddr is not hex.");
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


pub fn extended_pubk_to_addr(pubk: &Xpub) -> Result<EthAddr, Error> {
    //massage into the right format
    let pubk_str = pubk.public_key.to_string();
    let pubk_secp = secp256k1::PublicKey::from_str(&pubk_str)?;
    //format as uncompressed key, remove "04" in the beginning
    let pubk_uncomp = &PublicKey::new_uncompressed(pubk_secp).to_string()[2..];
    //decode from hex and pass to keccak for hashing
    let pubk_bytes = hex::decode(pubk_uncomp)?;
    let addr = &keccak_hash(&pubk_bytes);
    //keep last 20 bytes of the result
    let addr = &addr[(addr.len() - 40)..];
    //massage into domain unit
    EthAddr::new(addr)
}
