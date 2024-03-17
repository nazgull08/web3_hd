use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get the balance for a single address
    Balance { c: u32 },
    /// Get the balances for a range of addresses
    Balances {
        c_from: Option<u32>,
        c_to: Option<u32>,
    },
    /// Refill the wallet
    Refill,
    /// Sweep the balance
    Sweep { c: u32 },
    /// Generate a new phrase
    GenPhrase,
    /// Get the private key
    PrivKey { c: u32 },
    /// Send the main currency
    SendMain { c_from: u32, c_to: String },
    /// Send a token
    SendToken {
        c_from: u32,
        c_to: String,
        c_token: String,
    },
}
