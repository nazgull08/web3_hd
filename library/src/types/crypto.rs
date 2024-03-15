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
    pub fn get_hd_path(&self, index: i32) -> Result<DerivationPath, Error> {
        let str_path = match self {
            Crypto::Eth | Crypto::Polygon | Crypto::BSC => format!("m/44'/60'/0'/0/{}", index),
            Crypto::Tron => format!("m/44'/195'/0'/0/{}", index),
        };
        DerivationPath::from_str(&str_path)
    }
}
