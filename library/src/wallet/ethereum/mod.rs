pub mod address;
use async_trait::async_trait;
use ethers::{
    abi::Abi,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::Wallet,
    types::{Transaction, U256},
};
use secp256k1::SecretKey;
use std::{str::FromStr, sync::Arc};

use crate::{
    error::Error,
    types::{
        crypto::Crypto,
        hdseed::{FromSeed, HDSeed},
        token_data::TokenData,
    },
    utils::{address::address_str_to_h160, key::keypair_by_index},
};

use self::address::extended_pubk_to_addr;

use super::Wallet as HDWallet;

pub struct EthereumWallet {
    pub seed: HDSeed,
}

impl FromSeed for EthereumWallet {
    fn from_seed(seed: HDSeed) -> Self {
        EthereumWallet { seed }
    }
}

impl EthereumWallet {
    fn eth_address_by_index(&self, index: u32) -> Result<String, Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (_, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;
        let eth_addr = extended_pubk_to_addr(&pubk)?;

        Ok(eth_addr.get().to_owned())
    }

    fn eth_pubkey_by_index(&self, index: u32) -> Result<String, Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (_, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok(pubk.to_string())
    }

    fn eth_privkey_by_index(&self, index: u32) -> Result<String, Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (privk, _) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok(privk.private_key.display_secret().to_string())
    }

    fn eth_privkey_sekp_by_index(&self, index: u32) -> Result<SecretKey, Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (privk, _) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok(privk.private_key)
    }

    fn eth_keypair_by_index(&self, index: u32) -> Result<(String, String), Error> {
        let derivation_path = Crypto::Eth.get_hd_path(index)?;
        let (privk, pubk) = keypair_by_index(&self.seed.mnemonic, &derivation_path)?;

        Ok((
            privk.private_key.display_secret().to_string(),
            pubk.to_string(),
        ))
    }

    async fn eth_balance_by_index(&self, index: u32, provider_url: &str) -> Result<U256, Error> {
        let addr = self.eth_address_by_index(index)?;
        let addr_h160 = address_str_to_h160(&addr)?;
        let provider = Provider::<Http>::try_from(provider_url)?;
        let balance = provider.get_balance(addr_h160, None).await?;
        Ok(balance)
    }

    async fn eth_balance_token_by_index(
        &self,
        index: u32,
        provider_url: &str,
        token_addr: &str,
    ) -> Result<U256, Error> {
        // Получаем адрес по индексу, как и в предыдущем случае
        let addr = self.eth_address_by_index(index)?;
        let addr_h160 = address_str_to_h160(&addr)?;

        // Создаем провайдера
        let provider = Provider::<Http>::try_from(provider_url)?;

        // Адрес токена в формате H160
        let token_addr_h160 = address_str_to_h160(token_addr)?;

        // Загружаем ABI контракта ERC20
        let contract_abi = include_str!("../../../res/erc20.abi.json");
        let contract_abi = serde_json::from_str::<Abi>(contract_abi)?;

        let erc20_contract = Contract::new(token_addr_h160, contract_abi, Arc::new(provider));

        // Получаем баланс токенов на адресе
        let balance: U256 = erc20_contract
            .method::<_, U256>("balanceOf", addr_h160)?
            .call()
            .await?;

        Ok(balance)
    }

    async fn eth_transfer(
        &self,
        index: u32,
        to: &str,
        amount: U256,
        provider: &str,
    ) -> Result<Option<TransactionReceipt>, Error> {
        // Retrieve the private key for the specified wallet index.
        let priv_key_str = self.eth_privkey_by_index(index)?;

        // Parse the destination address.
        let to_address = to.parse::<Address>()?;

        // Create the provider.
        let provider =
            Provider::<Http>::try_from(provider)?.interval(std::time::Duration::from_millis(2000));

        // Create a wallet from the private key and attach it to the provider.
        let wallet = Wallet::from_str(&priv_key_str)?.with_chain_id(Chain::Sepolia);
        println!("wallet {:?}", wallet);
        let client = SignerMiddleware::new(provider.clone(), wallet.clone());

        let tx = TransactionRequest::new().to(to_address).value(amount);

        let tx = client.send_transaction(tx, None).await?.await?;
        // Return the details of the confirmed transaction.
        Ok(tx)
    }
}

#[async_trait]
impl HDWallet for EthereumWallet {
    fn address(&self, index: u32) -> Result<String, Error> {
        self.eth_address_by_index(index)
    }
    fn public(&self, index: u32) -> Result<String, Error> {
        self.eth_pubkey_by_index(index)
    }
    fn private(&self, index: u32) -> Result<String, Error> {
        self.eth_privkey_by_index(index)
    }
    fn keypair(&self, index: u32) -> Result<(String, String), Error> {
        self.eth_keypair_by_index(index)
    }
    async fn balance(&self, index: u32, provider: &str) -> Result<U256, Error> {
        self.eth_balance_by_index(index, provider).await
    }
    async fn balance_token(
        &self,
        index: u32,
        token_address: &str,
        provider: &str,
    ) -> Result<U256, Error> {
        self.eth_balance_token_by_index(index, provider, token_address)
            .await
    }
    async fn transfer(
        &self,
        index: u32,
        to: &str,
        amount: U256,
        provider: &str,
    ) -> Result<TransactionReceipt, Error> {
        match self.eth_transfer(index, to, amount, provider).await? {
            Some(receipt) => Ok(receipt),
            None => Err(Error::EthNoneTransferTransactionReceiptError),
        }
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
