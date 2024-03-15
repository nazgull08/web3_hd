use bip39::{Mnemonic, Seed};
use bitcoin::{bip32::{DerivationPath, Xpriv, Xpub}, Network};
use secp256k1::Secp256k1;
use serde::Serialize;
use sha3::{Digest, Keccak256};

use crate::error::Error;


pub fn get_extended_keypair(
    seed: &[u8],
    hd_path: &DerivationPath,
) -> Result<(Xpriv, Xpub), Error> {
    let secp = Secp256k1::new();
    let pk = Xpriv::new_master(Network::Bitcoin, seed)
        // we convert HD Path to bitcoin lib format (DerivationPath)
        .and_then(|k| k.derive_priv(&secp, hd_path))?;
    let pubk = Xpub::from_priv(&secp, &pk);
    Ok((pk, pubk))
}

pub fn keccak_hash<T>(data: &T) -> String
where
    T: ?Sized + Serialize + AsRef<[u8]>,
{
    let mut hasher = Keccak256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}


pub fn keypair_by_index(mnemonic: &Mnemonic, derivation_path: &DerivationPath) -> Result<(Xpriv,Xpub), Error> {
    let seed_m = Seed::new(mnemonic, "");
    let (privk, pubk) = get_extended_keypair(seed_m.as_bytes(), derivation_path)?;

    Ok((privk,pubk))
}
