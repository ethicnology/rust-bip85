use sha3::{
    digest::{core_api::XofReaderCoreWrapper, ExtendableOutput, Update},
    Shake256, Shake256ReaderCore,
};

/// BIP85-DRNG-SHAKE256 is a deterministic random number generator for cryptographic functions that require deterministic outputs, but where the input to that function requires more than the 64 bytes provided by BIP85's HMAC output.
/// BIP85-DRNG-SHAKE256 uses BIP85 to seed a SHAKE256 stream (from the SHA-3 standard).
///
/// The input must be exactly 64 bytes long.
///
/// See [specs](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip85-drng) for more info.
///
/// ### Example
/// ```rust
/// use bip85_fork::*;
/// use std::str::FromStr;
/// use bitcoin::{
///     bip32::{DerivationPath, Xpriv},
///     hex::DisplayHex,
///     key::Secp256k1,
/// };
///
/// let xprv = Xpriv::from_str("xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb").unwrap();
/// let path = DerivationPath::from_str("m/0'/0'").unwrap();
/// let derived = derive(&Secp256k1::new(), &xprv, &path).unwrap();
/// let entropy: [u8; 64] = derived.try_into().unwrap();
/// println!("Entropy: {}", entropy.clone().to_lower_hex_string());
///
/// let mut drng_reader = DRNG::new(entropy);
/// let mut drng_80_bytes = [0u8; 80];
/// drng_reader.read(&mut drng_80_bytes);
/// println!("DRNG: {}", drng_80_bytes.to_lower_hex_string());
/// ```
pub struct DRNG {
    reader: XofReaderCoreWrapper<Shake256ReaderCore>,
}

impl DRNG {
    pub fn new(entropy: [u8; 64]) -> Self {
        let mut hasher = Shake256::default();
        hasher.update(&entropy);
        DRNG {
            reader: hasher.finalize_xof(),
        }
    }

    pub fn read(&mut self, buffer: &mut [u8]) {
        sha3::digest::XofReader::read(&mut self.reader, buffer);
    }
}
