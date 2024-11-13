use std::str::FromStr;

use bip85_fork::bitcoin::bip32::Xpriv;
use bip85_fork::bitcoin::secp256k1::Secp256k1;

#[flutter_rust_bridge::frb(sync)]
pub fn to_wif(xpriv: String, index: u32) -> String {
    let root = Xpriv::from_str(&xpriv).unwrap();
    let derived = bip85_fork::to_wif(&Secp256k1::new(), &root, index).unwrap();
    return derived.to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_xprv(xpriv: String, index: u32) -> String {
    let root = Xpriv::from_str(&xpriv).unwrap();
    let derived = bip85_fork::to_xprv(&Secp256k1::new(), &root, index).unwrap();
    return derived.to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_hex(xpriv: String, length: u32, index: u32) -> String {
    let root = Xpriv::from_str(&xpriv).unwrap();
    let derived = bip85_fork::to_hex(&Secp256k1::new(), &root, length, index).unwrap();
    return hex::encode(derived);
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_mnemonic(xpriv: String, word_count: u32, index: u32) -> String {
    let root = Xpriv::from_str(&xpriv).unwrap();
    let derived = bip85_fork::to_mnemonic(&Secp256k1::new(), &root, word_count, index).unwrap();
    return derived.to_string();
}
