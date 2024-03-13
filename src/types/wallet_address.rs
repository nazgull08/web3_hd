use ethers::types::U256;


#[derive(Debug, Clone)]
pub struct WalletAddress {
    pub id: u32,
    pub address: String,
    pub balance: U256,
    pub balance_token: (String, U256),
}

