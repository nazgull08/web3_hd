pub mod address;

use async_trait::async_trait;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Transaction, U256},
};

use crate::{
    error::Error,
    types::{
        crypto::Crypto,
        hdseed::{FromSeed, HDSeed},
        token_data::TokenData,
    },
    utils::{address::address_str_to_h160, key::keypair_by_index},
    wallet::ethereum::address::extended_pubk_to_addr,
};

use self::address::extended_pubk_to_addr_tron;
use super::Wallet;

pub struct TronWallet {
    pub seed: HDSeed,
}

impl FromSeed for TronWallet {
    fn from_seed(seed: HDSeed) -> Self {
        TronWallet { seed }
    }
}

impl TronWallet {
    pub fn tron_address_by_index(&self, index: u32) -> Result<String, Error> {
        let derivation_path = Crypto::Tron.get_hd_path(index)?;
        let (_, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;
        let tron_addr = extended_pubk_to_addr_tron(&pubk)?;

        Ok(tron_addr.get().to_owned())
    }

    pub fn tron_hex_address_by_index(&self, index: u32) -> Result<String, Error> {
        let derivation_path = Crypto::Tron.get_hd_path(index)?;
        let (_, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;
        let tron_hex_addr = extended_pubk_to_addr(&pubk)?;
        println!("tron_hex_addr {:?}", tron_hex_addr);

        Ok(tron_hex_addr.get().to_owned())
    }

    fn tron_pubkey_by_index(&self, index: u32) -> Result<String, Error> {
        let derivation_path = Crypto::Tron.get_hd_path(index)?;
        let (_, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok(pubk.to_string())
    }

    fn tron_privkey_by_index(&self, index: u32) -> Result<String, Error> {
        let derivation_path = Crypto::Tron.get_hd_path(index)?;
        let (privk, _) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok(privk.private_key.display_secret().to_string())
    }

    fn tron_keypair_by_index(&self, index: u32) -> Result<(String, String), Error> {
        let derivation_path = Crypto::Tron.get_hd_path(index)?;
        let (privk, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok((
            privk.private_key.display_secret().to_string(),
            pubk.to_string(),
        ))
    }

    async fn tron_balance_by_index(&self, index: u32, provider_url: &str) -> Result<U256, Error> {
        let addr = self.tron_hex_address_by_index(index)?;
        println!("addr {:?}", addr);
        let addr_h160 = address_str_to_h160(&addr)?;
        let provider = Provider::<Http>::try_from(provider_url)?;
        println!("addrs {:?}", addr);
        println!("addrs hex {:?}", addr_h160);
        let balance = provider.get_balance(addr_h160, None).await?;
        println!("balance {:?}", balance);
        Ok(balance)
    }
}

#[async_trait]
impl Wallet for TronWallet {
    fn address(&self, index: u32) -> Result<String, Error> {
        self.tron_address_by_index(index)
    }
    fn public(&self, index: u32) -> Result<String, Error> {
        self.tron_pubkey_by_index(index)
    }
    fn private(&self, index: u32) -> Result<String, Error> {
        self.tron_privkey_by_index(index)
    }
    fn keypair(&self, index: u32) -> Result<(String, String), Error> {
        self.tron_keypair_by_index(index)
    }
    async fn balance(&self, index: u32, provider: &str) -> Result<U256, Error> {
        self.tron_balance_by_index(index, provider).await
    }
    fn balance_token(
        &self,
        _index: u32,
        _token_address: &str,
        _provider: &str,
    ) -> Result<TokenData, Error> {
        unimplemented!()
    }
    fn sweep(&self, _index: u32, _to: &str, _provider: &str) -> Result<(Transaction, U256), Error> {
        unimplemented!()
    }
    fn sweep_token(
        &self,
        _index: u32,
        _token_address: &str,
        _to: &str,
        _provider: &str,
    ) -> Result<(Transaction, TokenData), Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip39::{Language, Mnemonic};
    const PHRASE : &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn test_tron_address_by_index() {
        let mnemonic = Mnemonic::from_phrase(PHRASE, Language::English).unwrap();
        let seed = HDSeed { mnemonic };

        let wallet = TronWallet { seed };

        let expected_address_0 = "TUEZSdKsoDHQMeZwihtdoBiN46zxhGWYdH";
        assert_eq!(wallet.address(0).unwrap(), expected_address_0);
    }

    #[test]
    fn test_tron_pubkey_by_index() {
        let mnemonic = Mnemonic::from_phrase(PHRASE, Language::English).unwrap();
        let seed = HDSeed { mnemonic };

        let wallet = TronWallet { seed };

        let expected_pubkey = "xpub6GH5FbhZEomKSf2YeFsq92oVisrWG9b1H6sHW2RYmGJtasVd7LckJXiovzCLL52Dz7GsrQJWoXTshExmhqxNtsnu8GoD1S3kHzLfg1Apo8d";
        assert_eq!(wallet.public(0).unwrap(), expected_pubkey);
    }

    #[test]
    fn test_tron_privkey_by_index() {
        let mnemonic = Mnemonic::from_phrase(PHRASE, Language::English).unwrap();
        let seed = HDSeed { mnemonic };

        let wallet = TronWallet { seed };

        let expected_privkey = "b5a4cea271ff424d7c31dc12a3e43e401df7a40d7412a15750f3f0b6b5449a28";
        assert_eq!(wallet.private(0).unwrap(), expected_privkey);
    }
}
