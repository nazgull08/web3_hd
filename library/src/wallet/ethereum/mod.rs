pub mod address;

use async_trait::async_trait;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Transaction, U256},
};

use crate::{
    error::Error,
    types::{crypto::Crypto, hdseed::{FromSeed, HDSeed}, token_data::TokenData},
    utils::{address::address_str_to_h160, key::keypair_by_index},
};

use self::address::extended_pubk_to_addr;

use super::Wallet;

pub struct EthereumWallet {
    pub seed: HDSeed,
}

impl FromSeed for EthereumWallet {
    fn from_seed(seed: HDSeed) -> Self {
        EthereumWallet { seed }
    }
}

impl EthereumWallet {
    fn eth_address_by_index(&self, index: i32) -> Result<String, Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (_, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;
        let eth_addr = extended_pubk_to_addr(&pubk)?;

        Ok(eth_addr.get().to_owned())
    }

    fn eth_pubkey_by_index(&self, index: i32) -> Result<String, Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (_, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok(pubk.to_string())
    }

    fn eth_privkey_by_index(&self, index: i32) -> Result<String, Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (privk, _) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok(privk.private_key.display_secret().to_string())
    }

    fn eth_keypair_by_index(&self, index: i32) -> Result<(String, String), Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (privk, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok((
            privk.private_key.display_secret().to_string(),
            pubk.to_string(),
        ))
    }

    async fn eth_balance_by_index(&self, index: i32, provider_url: &str) -> Result<U256, Error> {
        let addr = self.eth_address_by_index(index)?;
        let addr_h160 = address_str_to_h160(&addr)?;
        let provider = Provider::<Http>::try_from(provider_url)?;
        println!("provider {:?}",provider);
        let balance = provider.get_balance(addr_h160, None).await?;
        println!("balance {:?}",balance );
        Ok(balance)
    }
}

#[async_trait]
impl Wallet for EthereumWallet {
    fn address(&self, index: i32) -> Result<String, Error> {
        self.eth_address_by_index(index)
    }
    fn public(&self, index: i32) -> Result<String, Error> {
        self.eth_pubkey_by_index(index)
    }
    fn private(&self, index: i32) -> Result<String, Error> {
        self.eth_privkey_by_index(index)
    }
    fn keypair(&self, index: i32) -> Result<(String, String), Error> {
        self.eth_keypair_by_index(index)
    }
    async fn balance(&self, index: i32, provider: &str) -> Result<U256, Error> {
        self.eth_balance_by_index(index, provider).await
    }
    fn balance_token(
        &self,
        _index: i32,
        _token_address: &str,
        _provider: &str,
    ) -> Result<TokenData, Error> {
        unimplemented!()
    }
    fn sweep(&self, _index: i32, _to: &str, _provider: &str) -> Result<(Transaction, U256), Error> {
        unimplemented!()
    }
    fn sweep_token(
        &self,
        _index: i32,
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
    fn test_eth_address_by_index() {
        let mnemonic = Mnemonic::from_phrase(PHRASE, Language::English).unwrap();
        let seed = HDSeed { mnemonic };

        let wallet = EthereumWallet { seed };

        let expected_address_0 = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";
        assert_eq!(wallet.address(0).unwrap(), expected_address_0);
    }

    #[test]
    fn test_eth_pubkey_by_index() {
        let mnemonic = Mnemonic::from_phrase(PHRASE, Language::English).unwrap();
        let seed = HDSeed { mnemonic };

        let wallet = EthereumWallet { seed };

        let expected_pubkey = "xpub6H6LG2We64bdwqNF7gNkUJ5EvDibiT2gbs77oonbawV86XE3eMxZf9czGQ9CPdSzsdsHLnLEjiJJEDnFMAyLrWATesaVbTYeggBXMHaFKLg";
        assert_eq!(wallet.public(0).unwrap(), expected_pubkey);
    }

    #[test]
    fn test_eth_privkey_by_index() {
        let mnemonic = Mnemonic::from_phrase(PHRASE, Language::English).unwrap();
        let seed = HDSeed { mnemonic };

        let wallet = EthereumWallet { seed };

        let expected_privkey = "1ab42cc412b618bdea3a599e3c9bae199ebf030895b039e9db1e30dafb12b727";
        assert_eq!(wallet.private(0).unwrap(), expected_privkey);
    }
}
