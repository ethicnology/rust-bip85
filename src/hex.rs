use super::Error;
use bitcoin::bip32::{ChildNumber, DerivationPath};
use bitcoin::hex::DisplayHex;
use bitcoin::{bip32::Xpriv, key::Secp256k1, secp256k1};

/// Derive entropy into hexadecimal
///
/// The `length` can be from 16 to 64 and defines number of bytes derived.
///
/// See [specs](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#hex) for more info.
///
/// ### Example
/// ```rust
/// use bip85_extended::*;
/// use bitcoin::{bip32::Xpriv, key::Secp256k1};
/// use std::str::FromStr;
///
/// let root = Xpriv::from_str("xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb").unwrap();
/// let secp = Secp256k1::new();
/// let derived = to_hex(&secp, &root, 64, 0).unwrap();
/// ```
pub fn to_hex<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    length: u32,
    index: u32,
) -> Result<String, Error> {
    const HEX_APPLICATION_NUMBER: ChildNumber = ChildNumber::Hardened { index: 128169 };
    if length < 16 || length > 64 {
        return Err(Error::InvalidLength(length));
    }
    if index >= 0x80000000 {
        return Err(Error::InvalidIndex(index));
    }
    let path = DerivationPath::from(vec![
        HEX_APPLICATION_NUMBER,
        ChildNumber::from_hardened_idx(length).unwrap(),
        ChildNumber::from_hardened_idx(index).unwrap(),
    ]);
    let data = crate::derive(secp, root, &path)?;
    Ok(data[0..length as usize].to_lower_hex_string())
}
