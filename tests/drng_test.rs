use std::str::FromStr;

use bip85_extended::drng::DRNG;
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    hex::DisplayHex,
    key::Secp256k1,
};

#[test]
fn test_drng_80() {
    let root = Xpriv::from_str(
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaL\
             LHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb",
    )
    .unwrap();
    let path = DerivationPath::from_str("m/0'/0'").unwrap();
    let entropy = bip85_extended::derive(&Secp256k1::new(), &root, &path).unwrap();

    let entropy_hex = entropy.to_lower_hex_string();
    let expected = "efecfbccffea313214232d29e71563d941229afb4338c21f9517c41aaa0d16f00b83d2a09ef747e7a64e8e2bd5a14869e693da66ce94ac2da570ab7ee48618f7";
    assert_eq!(expected, entropy_hex);

    let entropy_64_bytes: [u8; 64] = entropy.try_into().unwrap();
    let mut drng_reader = DRNG::new(entropy_64_bytes);
    let mut drng_80_bytes = [0u8; 80];
    drng_reader.read(&mut drng_80_bytes);

    let drng_80_hex = drng_80_bytes.to_lower_hex_string();
    let expected = "b78b1ee6b345eae6836c2d53d33c64cdaf9a696487be81b03e822dc84b3f1cd883d7559e53d175f243e4c349e822a957bbff9224bc5dde9492ef54e8a439f6bc8c7355b87a925a37ee405a7502991111";
    assert_eq!(expected, drng_80_hex);

    let mut drng_25_more = [0u8; 25];
    drng_reader.read(&mut drng_25_more);
    let expected = "cd2dddaf1883f4e962abf4fb4b31cd28d5cf6b14f6ddcc9c19";
    assert_eq!(expected, drng_25_more.to_lower_hex_string());
}
