use std::str::FromStr;

use ethers::types::H160;

use crate::error::Error;

pub fn address_str_to_h160(address_str: &str) -> Result<H160, Error> {
    // Удалить возможный префикс `0x`
    let trimmed_address = address_str.trim_start_matches("0x");

    // Преобразовать строку в H160
    H160::from_str(trimmed_address).map_err(|e| Error::AddrToH160Error(e))
}
