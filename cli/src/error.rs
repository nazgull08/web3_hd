use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Config loading error")]
    ConfigError(#[from] config::ConfigError),
}
