use alloy_core::primitives::U256;
use bip39::{Language, Mnemonic};

use crate::error::Error;



#[derive(Debug, Clone)]
pub struct HDSeed {
    pub mnemonic: Mnemonic,
}

impl HDSeed {
    pub fn new(phrase: &str) -> Result<Self, Error> {
        let mnemonic = Mnemonic::from_phrase(phrase, Language::English)
            .map_err(|_| Error::MnemonicError(phrase.to_owned()))?;
        Ok(HDSeed { mnemonic })
    }
}

#[derive(Debug, Clone)]
pub enum HDWallet {
    Ethereum(HDSeed),
    Tron(HDSeed),
}


//NTD add funcion for save key to file
impl HDWallet {
    pub fn address(&self, index: i32) -> Result<String, Error> {
        match self {
            HDWallet::Ethereum(seed) => eth_address_by_index(seed, index),
            HDWallet::Tron(seed) => tron_address_by_index(seed, index),
        }
    }

    pub fn address_hex(&self, index: i32) -> Result<String, Error> {
        match self {
            HDWallet::Ethereum(seed) => eth_address_by_index(seed, index),
            HDWallet::Tron(seed) => tron_address_by_index_hex(seed, index),
        }
    }

    pub fn private(&self, index: i32) -> Result<String, Error> {
        match self {
            HDWallet::Ethereum(seed) => eth_private_by_index(seed, index),
            HDWallet::Tron(seed) => tron_private_by_index(seed, index),
        }
    }

    pub fn keypair(&self, index: i32) -> Result<(ExtendedPrivKey, ExtendedPubKey), Error> {
        match self {
            HDWallet::Ethereum(seed) => eth_keypair_by_index(seed, index),
            HDWallet::Tron(seed) => tron_keypair_by_index(seed, index),
        }
    }

    pub fn public(&self, index: i32) -> Result<String, Error> {
        match self {
            HDWallet::Ethereum(seed) => eth_public_by_index(seed, index),
            HDWallet::Tron(seed) => tron_public_by_index(seed, index),
        }
    }

    pub async fn balance(&self, index: i32, provider: &str) -> Result<U256, Error> {
        match self {
            HDWallet::Ethereum(seed) => eth_balance(seed, index, provider).await,
            HDWallet::Tron(seed) => tron_balance(seed, index, provider).await,
        }
    }

    pub async fn balance_token(
        &self,
        index: i32,
        addr: &str,
        provider: &str,
    ) -> Result<TokenData, Error> {
        match self {
            HDWallet::Ethereum(seed) => eth_balance_token(seed, index, addr, provider).await,
            HDWallet::Tron(seed) => tron_balance_token(seed, index, addr, provider).await,
            HDWallet::Stellar(master_key) => {
                stellar_balance_token(master_key, index, addr, provider).await
            }
        }
    }

    pub async fn sweep(
        &self,
        index: i32,
        to: &str,
        provider: &str,
    ) -> Result<(Transaction, U256), Error> {
        match self {
            HDWallet::Ethereum(seed) => eth_sweep_main(seed, index, to, provider).await,
            HDWallet::Tron(seed) => tron_sweep_main(seed, index, to, provider).await,
        }
    }

    pub async fn sweep_token(
        &self,
        index: i32,
        addr: &str,
        to: &str,
        provider: &str,
    ) -> Result<(Transaction, U256), Error> {
        match self {
            HDWallet::Ethereum(seed) => {
                eth_sweep_token(seed, index, addr, to, provider, Crypto::Eth).await
            }
            HDWallet::Tron(seed) => {
                tron_sweep_token(seed, index, addr, to, provider, Crypto::Tron).await
            }
        }
    }
}

