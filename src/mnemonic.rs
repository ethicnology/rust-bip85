#[cfg(feature = "mnemonic")]
use bip39::Language;
#[cfg(feature = "mnemonic")]
use bip39::Mnemonic;

use super::Error;
use bitcoin::bip32::{ChildNumber, DerivationPath};
use bitcoin::{bip32::Xpriv, key::Secp256k1, secp256k1};

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
    const BIP39_APPLICATION_NUMBER: ChildNumber = ChildNumber::Hardened { index: 39 };
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
        BIP39_APPLICATION_NUMBER,
        ChildNumber::Hardened {
            index: language_index,
        },
        ChildNumber::from_hardened_idx(word_count).unwrap(),
        ChildNumber::from_hardened_idx(index).unwrap(),
    ]);
    let data = crate::derive(secp, root, &path)?;
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
