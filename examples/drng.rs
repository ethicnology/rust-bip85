use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    hex::DisplayHex,
    key::Secp256k1,
};
use std::str::FromStr;

use bip85_extended::*;

fn main() {
    let xprv = Xpriv::from_str("xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb").unwrap();
    let path = DerivationPath::from_str("m/0'/0'").unwrap();
    let derived = bip85_extended::derive(&Secp256k1::new(), &xprv, &path).unwrap();
    let entropy: [u8; 64] = derived.try_into().unwrap();
    println!("Entropy: {}", entropy.clone().to_lower_hex_string());

    let mut drng_reader = bip85_extended::DRNG::new(entropy);
    let mut drng_80_bytes = [0u8; 80];
    drng_reader.read(&mut drng_80_bytes);
    println!("DRNG: {}", drng_80_bytes.to_lower_hex_string());
}
