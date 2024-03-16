use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub sweeper: String,
    pub sweeper_tron_address: String,
    pub hd_phrase: String,
    pub eth_tokens: Vec<String>,
    pub eth_safe: String,
    pub eth_provider: String,
    pub tron_tokens: Vec<String>,
    pub tron_safe: String,
    pub tron_provider: String,
    pub plg_tokens: Vec<String>,
    pub plg_safe: String,
    pub plg_provider: String,
    pub bsc_tokens: Vec<String>,
    pub bsc_safe: String,
    pub bsc_provider: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config.toml").required(true))
            .build()?;
        s.try_deserialize()
    }
}
