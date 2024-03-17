use bip39::{Language, Mnemonic};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct HDSeed {
    pub mnemonic: Mnemonic,
}

impl HDSeed {
    pub fn new(phrase: &str) -> Result<Self, Error> {
        let mnemonic = Mnemonic::from_phrase(phrase, Language::English)
            .map_err(|_| Error::MnemonicError(phrase.to_owned()))?;
        Ok(HDSeed { mnemonic })
    }
}

pub trait FromSeed {
    fn from_seed(seed: HDSeed) -> Self;
}
