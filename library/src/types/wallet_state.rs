use super::balance_state::BalanceState;

#[derive(Debug, Clone)]
pub struct WalletState {
    pub id: u32,
    pub address: String,
    pub state: BalanceState,
}
