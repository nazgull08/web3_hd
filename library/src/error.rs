use ethers::{
    middleware::signer,
    providers::{Http, Provider},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Bitcoin bip32 error")]
    BitcoinBip32Error(#[from] bitcoin::bip32::Error),
    #[error("Bitcoin Base58 error")]
    BitcoinBase58Error(#[from] bitcoin::base58::Error),
    #[error("EthAddr is {0} instead of 42 chars long")]
    EthAddrLengthError(usize),
    #[error("secp256k1 error")]
    Secp256Error(#[from] secp256k1::Error),
    #[error("Hex decoding error")]
    HexError(#[from] hex::FromHexError),
    #[error("Mnemonic error")]
    MnemonicError(String),
    #[error("String to H160 error {0}")]
    AddrToH160Error(#[from] rustc_hex::FromHexError),
    #[error("Url parse error")]
    UrlParseError(#[from] url::ParseError),
    #[error("Ether providers error")]
    EtherProvidersError(#[from] ethers::providers::ProviderError),
    #[error("TronAddr decoding error")]
    TronAddrDecodingError,
    #[error("TronAddr is invalid length")]
    TronAddrLengthError,
    #[error("Serde parse error")]
    SerdeParseError(#[from] serde_json::Error),
    #[error("Ethers Contract ABI error")]
    EthersContractAbiError(#[from] ethers::contract::AbiError),
    #[error("Ethers ABI error")]
    EthersAbiError(#[from] ethers::abi::Error),
    #[error("Ethers wallet error")]
    EthersWalletError(#[from] ethers::signers::WalletError),
    #[error("Ethers contract error")]
    EthersContractError(#[from] ethers::contract::ContractError<Provider<Http>>),
    #[error("Ethers sekp error")]
    EthersSekpError(
        #[from]
        signer::SignerMiddlewareError<
            Provider<Http>,
            ethers::signers::Wallet<ecdsa::SigningKey<ethers::core::k256::Secp256k1>>,
        >,
    ),
    #[error("Web3 Contract error")]
    Web3ContractError(#[from] web3::contract::Error),
    #[error("Eth transfer tx receipt is none")]
    EthNoneTransferTransactionReceiptError,
}
