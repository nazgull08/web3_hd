pub mod address;

use std::str::FromStr;

use bip39::Seed;
use bitcoin::bip32::DerivationPath;
use ethers::types::{Transaction, U256};

use crate::{error::Error, types::{hdseed::HDSeed, token_data::TokenData}, utils::key::get_extended_keypair};

use super::Wallet;

pub struct EthereumWallet {
    pub seed: HDSeed,
}

impl EthereumWallet {
    fn eth_address_by_index(&self, index: i32) -> Result<String, Error> {
        let hd_path_str = format!("m/44'/60'/0'/0/{}", index);
        let seed_m = Seed::new(&self.seed.mnemonic, "");
        let derivation_path =  DerivationPath::from_str(&hd_path_str)?;
        let (_, pubk) = get_extended_keypair(seed_m.as_bytes(), &derivation_path)?;
        let eth_addr = extended_pubk_to_addr(&pubk)?;

        Ok(eth_addr.get().to_owned())
    }
}

impl Wallet for EthereumWallet {
    fn address(&self, index: i32) -> Result<String, Error> {
        unimplemented!() 
    }
    fn public(&self, index: i32) -> Result<String, Error> {
        unimplemented!()
    }
    fn private(&self, index: i32) -> Result<String, Error> {
        unimplemented!()
    }
    fn keypair(&self, index: i32) -> Result<(String, String), Error> {
        unimplemented!()
    }
    fn balance(&self, index: i32, provider: &str) -> Result<ethers::types::U256, Error> {
        unimplemented!()
    }
    fn balance_token(&self, index: i32, token_address: &str, provider: &str) -> Result<TokenData, Error> {
        unimplemented!()
    }
    fn sweep(&self, index: i32, to: &str, provider: &str) -> Result<(Transaction,U256), Error> {
        unimplemented!()
    }
    fn sweep_token(&self, index: i32, token_address: &str, to: &str, provider: &str) -> Result<(Transaction, TokenData), Error> {
        unimplemented!()
    }
}
