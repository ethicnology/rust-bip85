use super::Error;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bitcoin::bip32::{ChildNumber, DerivationPath};
use bitcoin::{bip32::Xpriv, key::Secp256k1, secp256k1};

/// Derive entropy into base64
///
/// The `length` can be from 20 to 86 and defines number of bytes derived.
///
/// See [specs](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#pwd-base64) for more info.
///
/// ### Example
/// ```rust
/// use bip85_fork::*;
/// use bitcoin::{bip32::Xpriv, key::Secp256k1};
/// use std::str::FromStr;
///
/// let root = Xpriv::from_str("xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb").unwrap();
/// let pwd_base64 = to_pwd_base64(&Secp256k1::new(), &root, 21, 0).unwrap();
/// ```
pub fn to_pwd_base64<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    length: u32,
    index: u32,
) -> Result<String, Error> {
    const PWD_BASE64_APPLICATION_NUMBER: ChildNumber = ChildNumber::Hardened { index: 707764 };
    if length < 20 || length > 86 {
        return Err(Error::InvalidLength(length));
    }
    if index >= 0x80000000 {
        return Err(Error::InvalidIndex(index));
    }
    let path = DerivationPath::from(vec![
        PWD_BASE64_APPLICATION_NUMBER,
        ChildNumber::from_hardened_idx(length).unwrap(),
        ChildNumber::from_hardened_idx(index).unwrap(),
    ]);
    let data = crate::derive(secp, root, &path)?;
    let mut pwd = BASE64_STANDARD.encode(data);
    pwd.truncate(length.try_into().unwrap());
    Ok(pwd)
}
