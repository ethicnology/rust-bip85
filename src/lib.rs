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

use std::convert::TryInto;
use std::default::Default;
use std::fmt;

use bitcoin::bip32;
use bitcoin::bip32::ChildNumber;
use bitcoin::bip32::DerivationPath;
use bitcoin::bip32::Xpriv;
use bitcoin::hashes::{hmac, sha512, Hash, HashEngine};
use bitcoin::secp256k1::{self, Secp256k1, SecretKey};
use bitcoin::PrivateKey;

#[cfg(feature = "mnemonic")]
use bip39::Language;
#[cfg(feature = "mnemonic")]
use bip39::Mnemonic;

/// A BIP85 error.
#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    /// Hardened index is provided, but only non-hardened indexes are allowed
    InvalidIndex(u32),
    /// Wrong number of bytes requested
    InvalidLength(u32),
    /// Wrong number of words for mnemonic
    InvalidWordCount(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidIndex(index) => write!(
                f,
                "invalid index for derivation, should be less than 0x80000000: {}",
                index,
            ),
            Error::InvalidLength(len) => write!(
                f,
                "invalid bytes length: {}. Should be between 16 and 64",
                len,
            ),
            Error::InvalidWordCount(word_count) => write!(
                f,
                "invalid number of words for mnemonic: {}. Should be 12, 18 or 24",
                word_count,
            ),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

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

/// Derive Bitcoin Private Key from the root inner
///
/// See [specs](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#hd-seed-wif)
/// for more info.
///
/// `index` can be any number lower than `0x80000000`
pub fn to_wif<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    index: u32,
) -> Result<PrivateKey, Error> {
    const BIP85_WIF_INDEX: ChildNumber = ChildNumber::Hardened { index: 2 };
    if index >= 0x80000000 {
        return Err(Error::InvalidIndex(index));
    }
    let path = DerivationPath::from(vec![
        BIP85_WIF_INDEX,
        ChildNumber::from_hardened_idx(index).unwrap(),
    ]);
    let data = derive(secp, root, &path)?;
    Ok(PrivateKey {
        compressed: true,
        network: root.network,
        inner: SecretKey::from_slice(&data[0..32]).unwrap(),
    })
}

/// Derive bip32 extended private inner from root xprv
///
/// See [specs](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#xprv) for more info.
///
/// `index` can be any number lower than `0x80000000`
pub fn to_xprv<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    index: u32,
) -> Result<Xpriv, Error> {
    const BIP85_BIP32_INDEX: ChildNumber = ChildNumber::Hardened { index: 32 };
    if index >= 0x80000000 {
        return Err(Error::InvalidIndex(index));
    }
    let path = DerivationPath::from(vec![
        BIP85_BIP32_INDEX,
        ChildNumber::from_hardened_idx(index).unwrap(),
    ]);
    let data = derive(secp, root, &path)?;
    let chain_code: [u8; 32] = data[..32].try_into().expect("");
    Ok(Xpriv {
        network: root.network,
        depth: 0,
        parent_fingerprint: Default::default(),
        child_number: ChildNumber::Normal { index: 0 },
        private_key: SecretKey::from_slice(&data[32..]).unwrap(),
        chain_code: bip32::ChainCode::from(chain_code),
    })
}

/// Derive binary entropy of certain length from the root inner
///
/// The `length` can be from 16 to 64 and defines number of bytes derived.
///
/// See [specs](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#hex) for more info.
pub fn to_hex<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    length: u32,
    index: u32,
) -> Result<Vec<u8>, Error> {
    const BIP85_HEX_INDEX: ChildNumber = ChildNumber::Hardened { index: 128169 };
    if length < 16 || length > 64 {
        return Err(Error::InvalidLength(length));
    }
    if index >= 0x80000000 {
        return Err(Error::InvalidIndex(index));
    }
    let path = DerivationPath::from(vec![
        BIP85_HEX_INDEX,
        ChildNumber::from_hardened_idx(length).unwrap(),
        ChildNumber::from_hardened_idx(index).unwrap(),
    ]);
    let data = derive(secp, root, &path)?;
    Ok(data[0..length as usize].to_vec())
}

#[cfg(feature = "mnemonic")]
/// Derive mnemonic in given language
///
/// See [specs](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip39)
/// for more info.
///
/// `word_count` can be 12, 18 or 24, `index` - anything lower than `0x80000000`
pub fn to_mnemonic_in<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    lang: Language,
    word_count: u32,
    index: u32,
) -> Result<Mnemonic, Error> {
    if word_count < 12 || word_count > 24 || word_count % 6 != 0 {
        return Err(Error::InvalidWordCount(word_count));
    }
    if index >= 0x80000000 {
        return Err(Error::InvalidIndex(index));
    }
    const BIP85_BIP39_INDEX: ChildNumber = ChildNumber::Hardened { index: 39 };
    let language_index = match lang {
        Language::English => 0,
        #[cfg(feature = "japanese")]
        Language::Japanese => 1,
        #[cfg(feature = "korean")]
        Language::Korean => 2,
        #[cfg(feature = "spanish")]
        Language::Spanish => 3,
        #[cfg(feature = "chinese-simplified")]
        Language::SimplifiedChinese => 4,
        #[cfg(feature = "chinese-traditional")]
        Language::TraditionalChinese => 5,
        #[cfg(feature = "french")]
        Language::French => 6,
        #[cfg(feature = "italian")]
        Language::Italian => 7,
        #[cfg(feature = "czech")]
        Language::Czech => 8,
    };
    let path = DerivationPath::from(vec![
        BIP85_BIP39_INDEX,
        ChildNumber::Hardened {
            index: language_index,
        },
        ChildNumber::from_hardened_idx(word_count).unwrap(),
        ChildNumber::from_hardened_idx(index).unwrap(),
    ]);
    let data = derive(secp, root, &path)?;
    let len = word_count * 4 / 3;
    let mnemonic = Mnemonic::from_entropy_in(lang, &data[0..len as usize]).unwrap();
    Ok(mnemonic)
}

/// Derive mnemonic from the xprv inner
///
/// Same as `to_mnemonic_in` using English language as default.
///
/// `word_count` can be 12, 18 or 24, `index` - anything lower than `0x80000000`
#[cfg(feature = "mnemonic")]
pub fn to_mnemonic<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    word_count: u32,
    index: u32,
) -> Result<Mnemonic, Error> {
    to_mnemonic_in(secp, root, Language::English, word_count, index)
}
