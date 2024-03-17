use std::str::FromStr;

use bitcoin::bip32::{DerivationPath, Error};

#[derive(Debug, Clone)]
pub enum Crypto {
    Eth,
    Tron,
    Polygon,
    BSC,
}

impl Crypto {
    pub fn get_hd_path(&self, index: u32) -> Result<DerivationPath, Error> {
        let str_path = match self {
            Crypto::Eth | Crypto::Polygon | Crypto::BSC => format!("m/44'/60'/0'/0/{}", index),
            Crypto::Tron => format!("m/44'/195'/0'/0/{}", index),
        };
        DerivationPath::from_str(&str_path)
    }
}

impl FromStr for Crypto {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "eth" => Ok(Crypto::Eth),
            "tron" => Ok(Crypto::Tron),
            "polygon" => Ok(Crypto::Polygon),
            "bsc" => Ok(Crypto::BSC),
            _ => Err("Unknown crypto"),
        }
    }
}

impl std::fmt::Display for Crypto {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Crypto::Eth => "Ethereum",
            Crypto::Tron => "Tron",
            Crypto::Polygon => "Polygon",
            Crypto::BSC => "Binance Smart Chain",
        };
        write!(f, "{}", s)
    }
}
