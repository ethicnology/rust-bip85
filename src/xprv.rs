use bitcoin::bip32::{self, ChildNumber, DerivationPath};
use bitcoin::secp256k1::SecretKey;
use bitcoin::{bip32::Xpriv, key::Secp256k1, secp256k1};

use super::Error;

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
    const BIP32_APPLICATION_NUMBER: ChildNumber = ChildNumber::Hardened { index: 32 };
    if index >= 0x80000000 {
        return Err(Error::InvalidIndex(index));
    }
    let path = DerivationPath::from(vec![
        BIP32_APPLICATION_NUMBER,
        ChildNumber::from_hardened_idx(index).unwrap(),
    ]);
    let data = crate::derive(secp, root, &path)?;
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
