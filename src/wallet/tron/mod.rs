pub mod address;

use std::str::FromStr;

use bip39::Seed;
use bitcoin::bip32::DerivationPath;
use ethers::types::{Transaction, U256};

use crate::{error::Error, types::{crypto::Crypto, hdseed::HDSeed, token_data::TokenData}, utils::key::{get_extended_keypair, keypair_by_index}};

use self::address::extended_pubk_to_addr_tron;
use super::Wallet;

pub struct TronWallet{
    pub seed: HDSeed,
}

impl TronWallet {
    pub fn tron_address_by_index(&self, index: i32) -> Result<String, Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (privk,pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path, index)?;
        let tron_addr = extended_pubk_to_addr_tron(&pubk)?;

        Ok(tron_addr)
    }
    
}

impl Wallet for TronWallet {
    fn address(&self, index: i32) -> Result<String, Error> {
        self.tron_address_by_index(index)
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



#[cfg(test)]
mod tests {
    use super::*;
    use bip39::{Mnemonic, Language};
    const PHRASE : &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn test_tron_address_by_index() {
        let mnemonic = Mnemonic::from_phrase(PHRASE, Language::English).unwrap();
        let seed = HDSeed { mnemonic };

        let wallet = TronWallet { seed };
        
        let expected_address_0 = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";
        assert_eq!(wallet.address(0).unwrap(), expected_address_0);
    }
/*
    #[test]
    fn test_eth_pubkey_by_index() {
        let mnemonic = Mnemonic::from_phrase(PHRASE, Language::English).unwrap();
        let seed = HDSeed { mnemonic };

        let wallet = TronWallet { seed };
        
        let expected_pubkey = "xpub6H6LG2We64bdwqNF7gNkUJ5EvDibiT2gbs77oonbawV86XE3eMxZf9czGQ9CPdSzsdsHLnLEjiJJEDnFMAyLrWATesaVbTYeggBXMHaFKLg";
        assert_eq!(wallet.public(0).unwrap(), expected_pubkey);
    }

    #[test]
    fn test_eth_privkey_by_index() {
        let mnemonic = Mnemonic::from_phrase(PHRASE, Language::English).unwrap();
        let seed = HDSeed { mnemonic };

        let wallet = TronWallet { seed };
        
        let expected_privkey = "1ab42cc412b618bdea3a599e3c9bae199ebf030895b039e9db1e30dafb12b727";
        assert_eq!(wallet.private(0).unwrap(), expected_privkey);
    }
*/
}
