use bip85_extended::*;
use bitcoin::{bip32::Xpriv, key::Secp256k1};
use std::str::FromStr;

#[test]
fn test_pwd_base85() {
    let root = Xpriv::from_str(
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb",
    )
    .unwrap();

    let pwd_base64 = to_pwd_base85(&Secp256k1::new(), &root, 12, 0).unwrap();
    let expected = "_s`{TW89)i4`";
    assert_eq!(expected, pwd_base64);
}