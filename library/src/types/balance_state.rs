use ethers::types::U256;

/// Enum of possible balance states
#[derive(Debug, Clone)]
pub enum BalanceState {
    Empty,
    ///< No money on wallet
    Tokens {
        ///< Only tokens on wallet
        tokens_balance: Vec<(String, U256)>,
    },
    ///< Tokens and main currency on wallet
    TokensMain {
        tokens_balance: Vec<(String, U256)>,
        balance: U256,
    },
    ///< Only main currency on wallet
    Main {
        balance: U256,
    },
}
