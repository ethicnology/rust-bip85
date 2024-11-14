// Rust implementation of bip-85
// Written in 2020 by Rita Kitic <rikitau@protonmail.com>
// Maintained in 2024 by Azad EMERY <ethicnology@pm.me>

//! # BIP-85 deterministic entropy generation
//!
//! Derives entropy from the extended private key according to
//! [BIP-85](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki).
//!
//! # Examples
//!
//! There are a few [examples](https://github.com/ethicnology/rust-bip85/tree/master/examples)
//! in the repository.
//!
//! # Optional features
//!
//! By default the library can derive entropy in any format specified by the standard except
//! mnemonics. To use mnemonics enable feature "mnemonic".
//!
//! All bip-39 languages except english are also optional, so if you plan generating mnemonics in
//! japanese enable feature "japanese", and so on.

#[cfg(feature = "mnemonic")]
pub extern crate bip39;
pub extern crate bitcoin;

pub mod drng;
pub mod error;
pub mod hex;
pub mod mnemonic;
pub mod pwd_base64;
pub mod pwd_base85;
pub mod wif;
pub mod xprv;

pub use drng::*;
pub use error::Error;
pub use hex::*;
pub use mnemonic::*;
pub use pwd_base64::*;
pub use pwd_base85::*;
pub use wif::*;
pub use xprv::*;

use bitcoin::bip32::ChildNumber;
use bitcoin::bip32::Xpriv;
use bitcoin::hashes::{hmac, sha512, Hash, HashEngine};
use bitcoin::secp256k1::{self, Secp256k1};

/// Derive raw bytes from the root inner using provided derivation path.
///
/// Use this function only for custom applications,
/// for standardized applications use application-specific functions - `to_wif`,
/// `to_hex`, `to_xprv` and `to_mnemonic[_in]`.
///
/// Derivation path should start *after* initial bip85 index (`83696968'`)
/// For example, to get entropy for WIF private inner (app_no `2`) with index `1`
/// use `DerivationPath::from_str("m/2'/0'")`.
pub fn derive<C: secp256k1::Signing, P: AsRef<[ChildNumber]>>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    path: &P,
) -> Result<Vec<u8>, Error> {
    const BIP85_CHILD_NUMBER: ChildNumber = ChildNumber::Hardened { index: 83696968 };
    let bip85_root = root.derive_priv(secp, &vec![BIP85_CHILD_NUMBER]).unwrap();
    let derived = bip85_root.derive_priv(secp, &path).unwrap();
    let mut h = hmac::HmacEngine::<sha512::Hash>::new("bip-entropy-from-k".as_bytes());
    h.input(&derived.private_key.secret_bytes());
    let data = hmac::Hmac::from_engine(h);
    Ok(data.to_byte_array().to_vec())
}
