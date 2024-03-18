use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config loading error")]
    ConfigError(#[from] config::ConfigError),
    #[error("Web3HD error")]
    Web3HDError(#[from] web3_hd::error::Error),
    #[error("The 'crypto' argument is required for this command.")]
    ArgsError,
    #[error("A 'provider_url' must be specified in the configuration.")]
    ProviderUrlError,
    #[error("Amount parse error")]
    EtherParseAmountError(#[from] ethers::utils::ConversionError),
}
