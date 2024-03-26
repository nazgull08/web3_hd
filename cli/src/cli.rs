use clap::Parser;

use bip39::Mnemonic;
use web3_hd::{
    types::{
        crypto::Crypto,
        hdseed::{FromSeed, HDSeed},
    },
    wallet::{ethereum::EthereumWallet, tron::TronWallet, Wallet},
};

use crate::{commands::Commands, error::Error, settings::Settings, wallet_manager::WalletManager};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub crypto: Option<Crypto>,
    #[arg(default_value = "./config.toml")]
    pub path: String,
    #[command(subcommand)]
    pub command: Commands,
}

pub async fn handle_command(args: Cli, config: Settings) -> Result<(), Error> {
    let manager = WalletManager::new(config);
    match args.command {
        Commands::Balance { c } => {
            manager.handle_balance(args.crypto, c).await?;
        }
        Commands::BalancesRange { c_from, c_to } => {
            manager.handle_balances(args.crypto, c_from, c_to).await?;
        }
        Commands::TokenBalance { c } => {
            manager.handle_balance_tokens(args.crypto, c).await?;
        }
        Commands::TokenBalancesRange { c_from, c_to } => {
            manager
                .handle_balances_tokens(args.crypto, c_from, c_to)
                .await?;
        }
        Commands::TotalBalance { c } => {
            manager.handle_total_balance(args.crypto, c).await?;
        }
        Commands::TotalBalancesRange { c_from, c_to } => {
            manager
                .handle_total_balances_range(args.crypto, c_from, c_to)
                .await?;
        }
        Commands::Refill {c }=> {
            // Логика для пополнения кошелька
        }
        Commands::Sweep { c } => {
            //let address = wallet.address(c as i32)?;
            //let (tx, balance) = wallet.sweep(c as i32, &address, &config.eth_provider)?;
            //println!("Swept {} from address {}", balance, address);
        }
        Commands::GenPhrase => {
            let a = Mnemonic::new(bip39::MnemonicType::Words12, bip39::Language::English);
            let phrase = a.into_phrase();
            println!("-----------");
            println!("{:?}", phrase);
        }
        Commands::PrivKey { c } => {
            manager.handle_priv_key(args.crypto, c)?;
        }
        Commands::SendMain { c_from, c_to } => {
            manager.handle_transfer(args.crypto, c_from, c_to).await?;
        }
        Commands::SendToken {
            c_from,
            c_to,
            c_token,
        } => {
            // Логика отправки токенов
        }
    }

    Ok(())
}

fn create_wallet_with_provider(
    crypto: Crypto,
    config: &Settings,
    seed: HDSeed,
) -> (Box<dyn Wallet>, &str) {
    match crypto {
        Crypto::Tron => (Box::new(TronWallet::from_seed(seed)), &config.tron_provider),
        Crypto::Eth => (
            Box::new(EthereumWallet::from_seed(seed)),
            &config.eth_provider,
        ),
        Crypto::BSC => (
            Box::new(EthereumWallet::from_seed(seed)),
            &config.bsc_provider,
        ),
        Crypto::Polygon => (
            Box::new(EthereumWallet::from_seed(seed)),
            &config.plg_provider,
        ),
    }
}
