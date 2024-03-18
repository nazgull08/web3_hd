use ethers::types::U256;
use web3_hd::{
    types::{
        crypto::Crypto,
        hdseed::{FromSeed, HDSeed},
    },
    wallet::{ethereum::EthereumWallet, tron::TronWallet, Wallet},
};

use crate::{error::Error, settings::Settings};

pub struct WalletManager {
    pub config: Settings,
}

impl WalletManager {
    pub fn new(config: Settings) -> Self {
        WalletManager { config }
    }

    pub fn get_provider(&self, crypto: &Crypto) -> &String {
        match crypto {
            Crypto::Tron => &self.config.tron_provider,
            Crypto::Eth => &self.config.eth_provider,
            Crypto::BSC => &self.config.bsc_provider,
            Crypto::Polygon => &self.config.plg_provider,
        }
    }
    pub fn get_wallet(&self, crypto: &Crypto, seed: HDSeed) -> Box<dyn Wallet> {
        match crypto {
            Crypto::Tron => Box::new(TronWallet::from_seed(seed)) as Box<dyn Wallet>,
            Crypto::Eth => Box::new(EthereumWallet::from_seed(seed)) as Box<dyn Wallet>,
            Crypto::BSC => Box::new(EthereumWallet::from_seed(seed)) as Box<dyn Wallet>,
            Crypto::Polygon => Box::new(EthereumWallet::from_seed(seed)) as Box<dyn Wallet>,
        }
    }

    pub fn get_wallet_tokens(&self, crypto: &Crypto) -> &Vec<String> {
        match crypto {
            Crypto::Tron => &self.config.tron_tokens,
            Crypto::Eth => &self.config.eth_tokens,
            Crypto::BSC => &self.config.bsc_tokens,
            Crypto::Polygon => &self.config.plg_tokens,
        }
    }

    pub async fn handle_balance(&self, ocrypto: Option<Crypto>, c: u32) -> Result<(), Error> {
        if let Some(crypto) = ocrypto {
            let seed = HDSeed::new(&self.config.hd_phrase)?;
            let wallet = self.get_wallet(&crypto, seed);
            let provider_url = &self.get_provider(&crypto);
            let address = wallet.address(c)?;
            let balance = wallet.balance(c, provider_url).await?;
            println!("Address: {}, Balance: {}", address, balance);
            Ok(())
        } else {
            Err(Error::ArgsError)
        }
    }

    pub async fn handle_balances(
        &self,
        ocrypto: Option<Crypto>,
        c_from: Option<u32>,
        c_to: Option<u32>,
    ) -> Result<(), Error> {
        if let Some(crypto) = ocrypto {
            let seed = HDSeed::new(&self.config.hd_phrase)?;
            let c_from = c_from.unwrap_or(0);
            let c_to = c_to.unwrap_or(10);
            let wallet = self.get_wallet(&crypto, seed);
            let provider_url = &self.get_provider(&crypto);
            for index in c_from..=c_to {
                let address = wallet.address(index)?;
                let balance = wallet.balance(index, provider_url).await?;
                println!("Address: {}, Balance: {}", address, balance);
            }
            Ok(())
        } else {
            Err(Error::ArgsError)
        }
    }

    pub async fn handle_balance_tokens(
        &self,
        ocrypto: Option<Crypto>,
        c: u32,
    ) -> Result<(), Error> {
        if let Some(crypto) = ocrypto {
            let seed = HDSeed::new(&self.config.hd_phrase)?;
            let wallet = self.get_wallet(&crypto, seed);
            let provider_url = &self.get_provider(&crypto);
            let tokens = self.get_wallet_tokens(&crypto);
            let address = wallet.address(c)?;
            for token in tokens {
                let balance = wallet.balance_token(c, &token, &provider_url).await?;
                println!(
                    "Address: {},\n Token: {}, Balance: {}",
                    address, token, balance
                );
            }
            Ok(())
        } else {
            Err(Error::ArgsError)
        }
    }

    pub async fn handle_balances_tokens(
        &self,
        ocrypto: Option<Crypto>,
        c_from: Option<u32>,
        c_to: Option<u32>,
    ) -> Result<(), Error> {
        if let Some(crypto) = ocrypto {
            let seed = HDSeed::new(&self.config.hd_phrase)?;
            let c_from = c_from.unwrap_or(0);
            let c_to = c_to.unwrap_or(10);
            let wallet = self.get_wallet(&crypto, seed);
            let provider_url = &self.get_provider(&crypto);
            let tokens = self.get_wallet_tokens(&crypto);
            for token in tokens {
                for index in c_from..=c_to {
                    let address = wallet.address(index)?;
                    let balance = wallet.balance_token(index, &token, &provider_url).await?;
                    println!(
                        "Address: {},\n Token: {}, Balance: {}",
                        address, token, balance
                    );
                }
            }
            Ok(())
        } else {
            Err(Error::ArgsError)
        }
    }

    pub async fn handle_total_balance(&self, ocrypto: Option<Crypto>, c: u32) -> Result<(), Error> {
        if let Some(crypto) = ocrypto {
            let seed = HDSeed::new(&self.config.hd_phrase)?;
            let wallet = self.get_wallet(&crypto, seed);
            let provider_url = &self.get_provider(&crypto);
            let address = wallet.address(c)?;
            let balance = wallet.balance(c, provider_url).await?;
            let tokens = self.get_wallet_tokens(&crypto);
            println!("Total Balance for Address: {}", address);
            println!("Main Currency: {}", balance);
            for token in tokens {
                let token_balance = wallet.balance_token(c, &token, provider_url).await?;
                println!("Token: {}, Balance: {}", token, token_balance);
            }
            Ok(())
        } else {
            Err(Error::ArgsError)
        }
    }

    pub async fn handle_total_balances_range(
        &self,
        ocrypto: Option<Crypto>,
        c_from: Option<u32>,
        c_to: Option<u32>,
    ) -> Result<(), Error> {
        if let Some(crypto) = ocrypto {
            let seed = HDSeed::new(&self.config.hd_phrase)?;
            let c_from = c_from.unwrap_or(0);
            let c_to = c_to.unwrap_or(10);
            let wallet = self.get_wallet(&crypto, seed);
            let provider_url = &self.get_provider(&crypto);
            let tokens = self.get_wallet_tokens(&crypto);
            for index in c_from..=c_to {
                let address = wallet.address(index)?;
                let balance = wallet.balance(index, provider_url).await?;
                println!("Total Balance for Address: {}", address);
                println!("Main Currency: {}", balance);
                for token in tokens {
                    let token_balance = wallet.balance_token(index, &token, provider_url).await?;
                    println!("Token: {}, Balance: {}", token, token_balance);
                }
            }
            Ok(())
        } else {
            Err(Error::ArgsError)
        }
    }

    pub fn handle_priv_key(&self, ocrypto: Option<Crypto>, c: u32) -> Result<(), Error> {
        if let Some(crypto) = ocrypto {
            let seed = HDSeed::new(&self.config.hd_phrase)?;
            let wallet = self.get_wallet(&crypto, seed);
            println!(
                "Address: {}\n Private: {}",
                wallet.address(c)?,
                wallet.private(c)?
            );
            Ok(())
        } else {
            Err(Error::ArgsError)
        }
    }

    pub async fn handle_transfer(&self, ocrypto: Option<Crypto>, c_from: u32, c_to: String) -> Result<(), Error> {
        if let Some(crypto) = ocrypto {
            let seed = HDSeed::new(&self.config.hd_phrase)?;
            let wallet = self.get_wallet(&crypto, seed);
            let provider_url = &self.get_provider(&crypto);
            let amount = U256::from(ethers::utils::parse_ether(0.000000000000123)?);
            let balance = wallet.balance(c_from, provider_url).await?;
            println!("balance {:?}",balance);
            println!("amount {:?}",amount);
            let receipt = wallet.transfer(c_from, &c_to, amount,&provider_url).await?;
            println!("Transaction Receipt {:?}",receipt);
            Ok(())
        } else {
            Err(Error::ArgsError)
        }
    }
}
