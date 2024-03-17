//! # Wallet Module
//!
//! This module provides the necessary structures and functionality to manage
//! different cryptocurrency wallets. It defines a common `Wallet` trait that
//! all specific wallet implementations must adhere to, ensuring a consistent
//! interface for interacting with various types of wallets.
//!
//! ## Modules
//!
//! - `ethereum`: Implementation of the Wallet trait for Ethereum.
//! - `tron`: Implementation of the Wallet trait for Tron.
//!
//! ## Usage
//!
//! The wallet module is designed to be flexible and extensible, allowing for
//! the addition of new wallet types as needed. Users can interact with different
//! blockchain wallets through a unified interface provided by the `Wallet` trait.

use async_trait::async_trait;
use ethers::types::{Transaction, U256};

use crate::{error::Error, types::token_data::TokenData};

pub mod ethereum;
pub mod tron;

/// A common trait defining the interface for interacting with cryptocurrency wallets.
/// This trait specifies the methods that all wallet implementations must provide,
/// ensuring a consistent behavior across different types of wallets.
#[async_trait]
pub trait Wallet {
    /// Retrieves the wallet's address at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - An index specifying which address to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the address string if successful, or an error if not.
    fn address(&self, index: u32) -> Result<String, Error>;

    /// Retrieves the wallet's private key at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - An index specifying which private key to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the private key string if successful, or an error if not.
    fn private(&self, index: u32) -> Result<String, Error>;

    /// Retrieves the wallet's public key at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - An index specifying which public key to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing the public key string if successful, or an error if not.
    fn public(&self, index: u32) -> Result<String, Error>;

    /// Generates and returns the keypair (private and public keys) for the wallet at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - An index at which the keypair should be generated.
    ///
    /// # Returns
    ///
    /// A `Result` containing a tuple of (private key, public key) if successful, or an error if not.
    fn keypair(&self, index: u32) -> Result<(String, String), Error>;

    /// Retrieves the balance of the wallet at the specified index.
    ///
    /// # Arguments
    ///
    /// * `index` - An index specifying which wallet's balance to retrieve.
    /// * `provider` - A string slice that holds the provider URL to fetch the balance from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the balance as `U256` if successful, or an error if not.
    async fn balance(&self, index: u32, provider: &str) -> Result<U256, Error>;

    /// Retrieves the token balance of the specified token in the wallet at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - An index specifying which wallet's token balance to retrieve.
    /// * `token_address` - The address of the token.
    /// * `provider` - A string slice that holds the provider URL to fetch the token balance from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the token balance as `TokenData` if successful, or an error if not.
    async fn balance_token(
        &self,
        index: u32,
        token_address: &str,
        provider: &str,
    ) -> Result<U256, Error>;
    /// Transfers all available native currency from the wallet at the specified index to another address.
    ///
    /// # Arguments
    ///
    /// * `index` - An index specifying which wallet to sweep.
    /// * `to` - The destination address to which the funds should be transferred.
    /// * `provider` - A string slice that holds the provider URL to execute the sweep operation.
    ///
    /// # Returns
    ///
    /// A `Result` containing a tuple (transaction details, transferred balance) if successful, or an error if not.
    fn sweep(&self, index: u32, to: &str, provider: &str) -> Result<(Transaction, U256), Error>;

    /// Transfers all available tokens of a specific type from the wallet at the specified index to another address.
    ///
    /// # Arguments
    ///
    /// * `index` - An index specifying which wallet's tokens to sweep.
    /// * `token_address` - The address of the token to be swept.
    /// * `to` - The destination address to which the tokens should be transferred.
    /// * `provider` - A string slice that holds the provider URL to execute the token sweep operation.
    ///
    /// # Returns
    ///
    /// A `Result` containing a tuple (transaction details, transferred token balance) if successful, or an error if not.
    fn sweep_token(
        &self,
        index: u32,
        token_address: &str,
        to: &str,
        provider: &str,
    ) -> Result<(Transaction, TokenData), Error>;
}
