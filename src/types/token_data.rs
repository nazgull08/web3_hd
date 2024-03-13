use ethers::types::U256;

#[derive(Debug, Clone)]
pub struct TokenData {
    pub balance: U256,
    pub balance_f: f64,
    pub decimals: u8,
    pub symbol: String,
    pub address: String,
}

