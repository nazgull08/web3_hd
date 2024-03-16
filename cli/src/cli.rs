use clap::Parser;
use web3_hd::{types::{crypto::Crypto, hdseed::{FromSeed, HDSeed}}, wallet::{ethereum::EthereumWallet, tron::TronWallet, Wallet}};

use crate::{commands::Commands, error::Error, settings::Settings};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub crypto: Crypto,
    #[arg(default_value = "./config.toml")]
    pub path: String,
    #[command(subcommand)]
    pub command: Commands
}


pub async fn handle_command(args: Cli, config: Settings) -> Result<(), Error> {
    let seed = HDSeed::new(&config.hd_phrase)?;
    let wallet: Box<dyn Wallet> = match args.crypto { // Проблема
        Crypto::Tron => Box::new(TronWallet::from_seed(seed)),
        _ => Box::new(EthereumWallet::from_seed(seed)),
        // Другие криптовалюты...
    };
    match args.command {
        Commands::Balance { c } => {
            let address = wallet.address(c as i32)?;
            let balance = wallet.balance(c as i32, &config.eth_provider).await?;
            println!("Address: {}, Balance: {}", address, balance);
        },
        Commands::Balances { c_from, c_to } => {
            let c_from = c_from.unwrap_or(0);
            let c_to = c_to.unwrap_or(10);
            for index in c_from..=c_to {
                let address = wallet.address(index as i32)?;
                let balance = wallet.balance(index as i32, &config.eth_provider).await?;
                println!("Address: {}, Balance: {}", address, balance);
            }
        },
        Commands::Refill => {
            // Логика для пополнения кошелька
        },
        Commands::Sweep { c } => {
            let address = wallet.address(c as i32)?;
            let (tx, balance) = wallet.sweep(c as i32, &address, &config.eth_provider)?;
            println!("Swept {} from address {}", balance, address);
        },
        Commands::GenPhrase => {
            // Генерация фразы
        },
        Commands::PrivKey { c } => {
            let private_key = wallet.private(c as i32)?;
            println!("Private Key: {}", private_key);
        },
        Commands::SendMain { c_from, c_to } => {
            // Логика отправки основной валюты
        },
        Commands::SendToken { c_from, c_to, c_token } => {
            // Логика отправки токенов
        },
    }   

    Ok(())
}
