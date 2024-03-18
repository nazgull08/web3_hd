pub mod address;

use async_trait::async_trait;
use bitcoin::base58;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Transaction, TransactionReceipt, U256},
};
use web3::contract::{Contract, Options};

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
        let addr_h160 = address_str_to_h160(&addr)?;
        let provider = Provider::<Http>::try_from(provider_url)?;
        let balance = provider.get_balance(addr_h160, None).await?;
        Ok(balance)
    }

    async fn tron_balance_token_by_index(
        &self,
        index: u32,
        provider_url: &str,
        token_addr: &str,
    ) -> Result<U256, Error> {
        let addr = self.tron_hex_address_by_index(index)?;
        let addr_h160 = address_str_to_h160(&addr)?;

        let transport = web3::transports::Http::new(provider_url).unwrap();
        let web3 = web3::Web3::new(transport);

        let token_addr_v = base58::decode(token_addr)?;
        let token_addr_hex = hex::encode(&token_addr_v);
        let token_addr_hex_p = "0x".to_owned() + &token_addr_hex[2..token_addr_hex.len() - 8];
        let token_addr_h160 = address_str_to_h160(&token_addr_hex_p)?;

        let contract = Contract::from_json(
            web3.eth(),
            token_addr_h160,
            include_bytes!("../../../res/erc20.abi.json"),
        )?;

        let result = contract.query("balanceOf", (addr_h160,), None, Options::default(), None);
        let balance: U256 = result.await?;

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
    async fn balance_token(
        &self,
        index: u32,
        token_address: &str,
        provider: &str,
    ) -> Result<U256, Error> {
        self.tron_balance_token_by_index(index, provider, token_address)
            .await
    }

    async fn transfer(
        &self,
        index: u32,
        to: &str,
        amount: U256,
        provider: &str,
    ) -> Result<TransactionReceipt, Error> {
        Err(Error::TronAddrLengthError)
    }

    async fn transfer_token(
        &self,
        index: u32,
        token_address: &str,
        to: &str,
        amount: U256,
        provider: &str,
    ) -> Result<TransactionReceipt, Error> {
        Err(Error::TronAddrLengthError)
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
