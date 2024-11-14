use super::Error;
use bitcoin::bip32::{ChildNumber, DerivationPath};
use bitcoin::secp256k1::SecretKey;
use bitcoin::PrivateKey;
use bitcoin::{bip32::Xpriv, key::Secp256k1, secp256k1};

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
    let data = crate::derive(secp, root, &path)?;
    Ok(PrivateKey {
        compressed: true,
        network: root.network,
        inner: SecretKey::from_slice(&data[0..32]).unwrap(),
    })
}
