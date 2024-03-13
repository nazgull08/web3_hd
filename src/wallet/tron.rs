use ethers::types::{Transaction, U256};

use crate::{error::Error, types::{hdseed::HDSeed, token_data::TokenData}};

use super::Wallet;

pub struct TronWallet {
    pub seed: HDSeed,
}

impl Wallet for TronWallet {
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
    fn balance(&self, index: i32, provider: &str) -> Result<U256, Error> {
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
