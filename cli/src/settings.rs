use config::{Config, ConfigError, Environment, File};
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
            .add_source(File::with_name("config").required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()?;
        let mut settings : Settings= s.clone().try_deserialize()?;
        if let Ok(tokens_str) = s.get::<String>("eth_tokens") {
            settings.eth_tokens = tokens_str.split(',').map(|s| s.trim().to_string()).collect();
        }
        if let Ok(tokens_str) = s.get::<String>("tron_tokens") {
            settings.tron_tokens = tokens_str.split(',').map(|s| s.trim().to_string()).collect();
        }
        if let Ok(tokens_str) = s.get::<String>("plg_tokens") {
            settings.plg_tokens = tokens_str.split(',').map(|s| s.trim().to_string()).collect();
        }
        if let Ok(tokens_str) = s.get::<String>("bsc_tokens") {
            settings.bsc_tokens = tokens_str.split(',').map(|s| s.trim().to_string()).collect();
        }
        println!("s {:?}",settings);
        Ok(settings)
    }
}
