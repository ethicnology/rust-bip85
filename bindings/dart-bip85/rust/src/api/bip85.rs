use std::str::FromStr;

use bip85_extended::bitcoin::bip32::{DerivationPath, Xpriv};
use bip85_extended::bitcoin::secp256k1::Secp256k1;

#[flutter_rust_bridge::frb(sync)]
pub fn derive(xprv: String, path: String) -> Vec<u8> {
    let root = Xpriv::from_str(&xprv).unwrap();
    let derivation_path = DerivationPath::from_str(&path).unwrap();
    let derived = bip85_extended::derive(&Secp256k1::new(), &root, &derivation_path).unwrap();
    return derived;
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_wif(xprv: String, index: u32) -> String {
    let root = Xpriv::from_str(&xprv).unwrap();
    let derived = bip85_extended::to_wif(&Secp256k1::new(), &root, index).unwrap();
    return derived.to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_xprv(xprv: String, index: u32) -> String {
    let root = Xpriv::from_str(&xprv).unwrap();
    let derived = bip85_extended::to_xprv(&Secp256k1::new(), &root, index).unwrap();
    return derived.to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_hex(xprv: String, length: u32, index: u32) -> String {
    let root = Xpriv::from_str(&xprv).unwrap();
    let derived = bip85_extended::to_hex(&Secp256k1::new(), &root, length, index).unwrap();
    return derived;
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_mnemonic(xprv: String, word_count: u32, index: u32) -> String {
    let root = Xpriv::from_str(&xprv).unwrap();
    let derived = bip85_extended::to_mnemonic(&Secp256k1::new(), &root, word_count, index).unwrap();
    return derived.to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_base64(xprv: String, length: u32, index: u32) -> String {
    let root = Xpriv::from_str(&xprv).unwrap();
    let derived = bip85_extended::to_pwd_base64(&Secp256k1::new(), &root, length, index).unwrap();
    return derived.to_string();
}

#[flutter_rust_bridge::frb(sync)]
pub fn to_base85(xprv: String, length: u32, index: u32) -> String {
    let root = Xpriv::from_str(&xprv).unwrap();
    let derived = bip85_extended::to_pwd_base85(&Secp256k1::new(), &root, length, index).unwrap();
    return derived.to_string();
}