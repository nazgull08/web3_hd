use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bitcoin bip32 error")]
    BitcoinBip32Error(#[from] bitcoin::bip32::Error),
    #[error("EthAddr is {0} instead of 42 chars long")]
    EthAddrLengthError(usize),
    #[error("secp256k1 error")]
    Secp256Error(#[from] secp256k1::Error),
    #[error("Hex decoding error")]
    HexError(#[from] hex::FromHexError),
    #[error("Mnemonic error")]
    MnemonicError(String),
}