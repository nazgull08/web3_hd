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

    pub fn get_wallet_provider(
        &self,
        crypto: Crypto,
        seed: HDSeed,
    ) -> (Box<dyn Wallet>, &String) {
        match crypto {
            Crypto::Tron => (
                Box::new(TronWallet::from_seed(seed)) as Box<dyn Wallet>,
                &self.config.tron_provider,
            ),
            Crypto::Eth => (
                Box::new(EthereumWallet::from_seed(seed)) as Box<dyn Wallet>,
                &self.config.eth_provider,
            ),
            Crypto::BSC => (
                Box::new(EthereumWallet::from_seed(seed)) as Box<dyn Wallet>,
                &self.config.bsc_provider,
            ),
            Crypto::Polygon => (
                Box::new(EthereumWallet::from_seed(seed)) as Box<dyn Wallet>,
                &self.config.plg_provider,
            ),
        }
    }

    pub async fn handle_balance(&self, ocrypto: Option<Crypto>, c: u32) -> Result<(), Error> {
        let seed = HDSeed::new(&self.config.hd_phrase)?;
        if let Some(crypto) = ocrypto{
            let (wallet, provider_url) = self.get_wallet_provider(crypto, seed);
            let address = wallet.address(c)?;
            let balance = wallet.balance(c, provider_url).await?;
            println!("Address: {}, Balance: {}", address, balance);
            Ok(())
        } else{
            Err(Error::ArgsError)
        }
    }

    pub async fn handle_balances(&self, ocrypto: Option<Crypto>, c_from: Option<u32>, c_to: Option<u32>) -> Result<(), Error> {
        if let Some(crypto) = ocrypto{
            let seed = HDSeed::new(&self.config.hd_phrase)?;
            let c_from = c_from.unwrap_or(0);
            let c_to = c_to.unwrap_or(10);
            let (wallet, provider_url) = self.get_wallet_provider(crypto, seed);
            for index in c_from..=c_to {
                let address = wallet.address(index)?;
                let balance = wallet.balance(index, provider_url).await?;
                println!("Address: {}, Balance: {}", address, balance);
            }
            Ok(())
        } else{
            Err(Error::ArgsError)
        }
    }
}
