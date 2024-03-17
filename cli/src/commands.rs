use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Retrieves the balance of the main currency for a specified address.
    Balance { 
        /// The address index for which to retrieve the balance.
        c: u32 
    },
    /// Retrieves the balances of the main currency for a range of addresses.
    BalancesRange {
        /// The starting address index for the balance retrieval range.
        c_from: Option<u32>,
        /// The ending address index for the balance retrieval range.
        c_to: Option<u32>,
    },
    /// Retrieves the balance of specified tokens for a single address.
    TokenBalance {
        /// The address index for which to retrieve token balances.
        c: u32,
    },
    /// Retrieves the balances of specified tokens for a range of addresses.
    TokenBalancesRange {
        /// The starting address index for the token balance retrieval range.
        c_from: Option<u32>,
        /// The ending address index for the token balance retrieval range.
        c_to: Option<u32>,
    },
    /// Retrieves the total balance (main currency and tokens) for a single address.
    TotalBalance {
        /// The address index for which to retrieve the total balance.
        c: u32,
    },
    /// Retrieves the total balances (main currency and tokens) for a range of addresses.
    TotalBalancesRange {
        /// The starting address index for the total balance retrieval range.
        c_from: Option<u32>,
        /// The ending address index for the total balance retrieval range.
        c_to: Option<u32>,
    },
    /// Refills the wallet balance. Specific implementation details can vary.
    Refill,
    /// Sweeps the balance of the main currency to another address.
    Sweep { 
        /// The address index from which to sweep the balance.
        c: u32 
    },
    /// Generates a new mnemonic phrase for wallet creation.
    GenPhrase,
    /// Retrieves the private key for a specified address.
    PrivKey { 
        /// The address index for which to retrieve the private key.
        c: u32 
    },
    /// Sends the main currency to another address.
    SendMain { 
        /// The sending address index.
        c_from: u32, 
        /// The recipient's address.
        c_to: String 
    },
    /// Sends a specified token to another address.
    SendToken {
        /// The sending address index.
        c_from: u32,
        /// The recipient's address.
        c_to: String,
        /// The token address to send.
        c_token: String,
    },
}
