use super::Error;
use bitcoin::bech32::{self, Bech32, Hrp};
use bitcoin::bip32::{ChildNumber, DerivationPath};
use bitcoin::{bip32::Xpriv, key::Secp256k1, secp256k1};

const NOSTR_APPLICATION_NUMBER: ChildNumber = ChildNumber::Hardened { index: 9000 };

/// Derive a Nostr private key as an `nsec` Bech32 string (NIP19) using BIP85.
///
/// See [specs](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#nostr)
/// for more info.
///
/// Path format: `m/83696968'/9000'/{identity}'/{account}'`.
///
/// Per the spec, `identity = 0'` is reserved for future protocol use and
/// `account = 0'` is reserved across all identities for key management
/// operations; usable keys start at `identity >= 1'` and `account >= 1'`.
/// This function will still derive zero-indexed children — callers that want
/// to enforce the "usable" convention must check the arguments themselves.
///
/// Both `identity` and `account` must be lower than `0x80000000`
/// (hardened-index range).
///
/// ### Example
/// ```rust
/// use bip85_extended::*;
/// use bitcoin::{bip32::Xpriv, key::Secp256k1};
/// use std::str::FromStr;
///
/// let root = Xpriv::from_str("xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb").unwrap();
/// let secp = Secp256k1::new();
/// let nsec = to_nostr(&secp, &root, 1, 1).unwrap();
/// assert_eq!(nsec, "nsec1254dr4tclcdujf7we9sjv5t99vru2tw7gqtezx7z80y4x45qwhlsmxapst");
/// ```
pub fn to_nostr<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    identity: u32,
    account: u32,
) -> Result<String, Error> {
    let key = to_nostr_bytes(secp, root, identity, account)?;
    let hrp = Hrp::parse("nsec").expect("nsec is a valid HRP");
    Ok(bech32::encode::<Bech32>(hrp, &key).expect("32-byte payload fits in Bech32 limits"))
}

/// Derive the 32-byte Nostr private key bytes using BIP85, without Bech32 encoding.
///
/// This is the most significant 32 bytes of the BIP85 HMAC output for path
/// `m/83696968'/9000'/{identity}'/{account}'`.
///
/// ### Example
/// ```rust
/// use bip85_extended::*;
/// use bitcoin::{bip32::Xpriv, key::Secp256k1, hex::DisplayHex};
/// use std::str::FromStr;
///
/// let root = Xpriv::from_str("xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb").unwrap();
/// let secp = Secp256k1::new();
/// let key = to_nostr_bytes(&secp, &root, 1, 1).unwrap();
/// assert_eq!(
///     key.to_lower_hex_string(),
///     "552ad1d578fe1bc927cec9612651652b07c52dde4017911bc23bc953568075ff",
/// );
/// ```
pub fn to_nostr_bytes<C: secp256k1::Signing>(
    secp: &Secp256k1<C>,
    root: &Xpriv,
    identity: u32,
    account: u32,
) -> Result<[u8; 32], Error> {
    if identity >= 0x80000000 {
        return Err(Error::InvalidIndex(identity));
    }
    if account >= 0x80000000 {
        return Err(Error::InvalidIndex(account));
    }
    let path = DerivationPath::from(vec![
        NOSTR_APPLICATION_NUMBER,
        ChildNumber::from_hardened_idx(identity).unwrap(),
        ChildNumber::from_hardened_idx(account).unwrap(),
    ]);
    let data = crate::derive(secp, root, &path)?;
    let mut out = [0u8; 32];
    out.copy_from_slice(&data[..32]);
    Ok(out)
}
