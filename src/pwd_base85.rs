use super::Error;
use bitcoin::bip32::{ChildNumber, DerivationPath};
use bitcoin::{bip32::Xpriv, key::Secp256k1, secp256k1};

/// The `length` can be from 10 to 80 and defines number of bytes derived.
///
/// See [specs](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#pwd-base85) for more info.
pub fn to_pwd_base85<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    length: u32,
    index: u32,
) -> Result<String, Error> {
    const BIP85_PWD_B64_APPLICATION_NUMBER: ChildNumber = ChildNumber::Hardened { index: 707785 };
    if length < 10 || length > 80 {
        return Err(Error::InvalidLength(length));
    }
    if index >= 0x80000000 {
        return Err(Error::InvalidIndex(index));
    }
    let path = DerivationPath::from(vec![
        BIP85_PWD_B64_APPLICATION_NUMBER,
        ChildNumber::from_hardened_idx(length).unwrap(),
        ChildNumber::from_hardened_idx(index).unwrap(),
    ]);
    let data = crate::derive(secp, root, &path)?;
    let mut pwd = base85::encode(&data);
    pwd.truncate(length.try_into().unwrap());
    Ok(pwd)
}