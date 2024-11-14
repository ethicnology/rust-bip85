use ::bip85_fork::error::Error;

#[cfg(feature = "mnemonic")]
use bip39::Mnemonic;
use bip85_fork::*;
use bitcoin::bip32::DerivationPath;
use bitcoin::bip32::Xpriv;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::PrivateKey;
use std::str::FromStr;

// test vectors from https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki
#[test]
fn test_raw() {
    let root = Xpriv::from_str(
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaL\
             LHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb",
    )
    .unwrap();
    let secp = Secp256k1::new();

    let path = DerivationPath::from_str("m/0'/0'").unwrap();
    let derived = derive(&secp, &root, &path).unwrap();
    let expected = vec![
        0xef, 0xec, 0xfb, 0xcc, 0xff, 0xea, 0x31, 0x32, 0x14, 0x23, 0x2d, 0x29, 0xe7, 0x15, 0x63,
        0xd9, 0x41, 0x22, 0x9a, 0xfb, 0x43, 0x38, 0xc2, 0x1f, 0x95, 0x17, 0xc4, 0x1a, 0xaa, 0x0d,
        0x16, 0xf0, 0x0b, 0x83, 0xd2, 0xa0, 0x9e, 0xf7, 0x47, 0xe7, 0xa6, 0x4e, 0x8e, 0x2b, 0xd5,
        0xa1, 0x48, 0x69, 0xe6, 0x93, 0xda, 0x66, 0xce, 0x94, 0xac, 0x2d, 0xa5, 0x70, 0xab, 0x7e,
        0xe4, 0x86, 0x18, 0xf7,
    ];
    assert_eq!(expected, derived);

    let path = DerivationPath::from_str("m/0'/1'").unwrap();
    let derived = derive(&secp, &root, &path).unwrap();
    let expected = vec![
        0x70, 0xc6, 0xe3, 0xe8, 0xeb, 0xee, 0x8d, 0xc4, 0xc0, 0xdb, 0xba, 0x66, 0x07, 0x68, 0x19,
        0xbb, 0x8c, 0x09, 0x67, 0x25, 0x27, 0xc4, 0x27, 0x7c, 0xa8, 0x72, 0x95, 0x32, 0xad, 0x71,
        0x18, 0x72, 0x21, 0x8f, 0x82, 0x69, 0x19, 0xf6, 0xb6, 0x72, 0x18, 0xad, 0xde, 0x99, 0x01,
        0x8a, 0x6d, 0xf9, 0x09, 0x5a, 0xb2, 0xb5, 0x8d, 0x80, 0x3b, 0x5b, 0x93, 0xec, 0x98, 0x02,
        0x08, 0x5a, 0x69, 0x0e,
    ];
    assert_eq!(expected, derived);
}

#[test]
fn test_priv() {
    let root = Xpriv::from_str(
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaL\
             LHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb",
    )
    .unwrap();
    let secp = Secp256k1::new();
    let derived = to_wif(&secp, &root, 0).unwrap();
    let expected =
        PrivateKey::from_str("Kzyv4uF39d4Jrw2W7UryTHwZr1zQVNk4dAFyqE6BuMrMh1Za7uhp").unwrap();

    assert_eq!(expected, derived);

    let index = 0x80000000 + 1;
    let derived = to_wif(&secp, &root, index);
    assert_eq!(derived, Err(Error::InvalidIndex(index)));
}

#[test]
fn test_xprv() {
    let root = Xpriv::from_str(
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaL\
             LHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb",
    )
    .unwrap();
    let secp = Secp256k1::new();

    let derived = to_xprv(&secp, &root, 0).unwrap();

    let expected = Xpriv::from_str(
        "xprv9s21ZrQH143K2srSbCSg4m4kLvPMzcWydgmKEnMmoZUurYuBuYG46c6P71UG\
             XMzmriLzCCBvKQWBUv3vPB3m1SATMhp3uEjXHJ42jFg7myX",
    )
    .unwrap();

    assert_eq!(expected, derived);
}

#[test]
fn test_hex() {
    let root = Xpriv::from_str(
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb",
    )
    .unwrap();
    let secp = Secp256k1::new();
    let derived = to_hex(&secp, &root, 64, 0).unwrap();
    let expected = "492db4698cf3b73a5a24998aa3e9d7fa96275d85724a91e71aa2d645442f878555d078fd1f1f67e368976f04137b1f7a0d19232136ca50c44614af72b5582a5c";
    assert_eq!(expected, &derived);
}

#[cfg(feature = "mnemonic")]
#[test]
fn test_mnemonic() {
    use mnemonic::to_mnemonic;

    let root = Xpriv::from_str(
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaL\
             LHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb",
    )
    .unwrap();
    let secp = Secp256k1::new();

    let derived = to_mnemonic(&secp, &root, 12, 0).unwrap();
    let expected = Mnemonic::from_str(
        "girl mad pet galaxy egg matter matrix prison refuse sense ordinary nose",
    )
    .unwrap();
    assert_eq!(derived, expected);

    let derived = to_mnemonic(&secp, &root, 18, 0).unwrap();
    let expected = Mnemonic::from_str(
        "near account window bike charge season chef number sketch tomorrow excuse sniff \
             circle vital hockey outdoor supply token",
    )
    .unwrap();
    assert_eq!(derived, expected);

    let derived = to_mnemonic(&secp, &root, 24, 0).unwrap();
    let expected = Mnemonic::from_str(
        "puppy ocean match cereal symbol another shed magic wrap hammer bulb intact gadget \
             divorce twin tonight reason outdoor destroy simple truth cigar social volcano",
    )
    .unwrap();
    assert_eq!(derived, expected);
}
