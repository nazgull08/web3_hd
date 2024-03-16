use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config loading error")]
    ConfigError(#[from] config::ConfigError),
    #[error("Web3HD error")]
    Web3HDError(#[from] web3_hd::error::Error),
}
