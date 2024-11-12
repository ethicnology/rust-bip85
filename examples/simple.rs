extern crate my_bip85;

use std::str::FromStr;

use bitcoin::bip32::Xpriv;
use bitcoin::secp256k1::Secp256k1;

fn main() {
    let root = Xpriv::from_str(
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaL\
         LHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb",
    )
    .unwrap();
    let secp = Secp256k1::new();

    let derived = my_bip85::to_wif(&secp, &root, 0).unwrap();
    println!("WIF key:\n{}", derived);

    let data = my_bip85::to_hex(&secp, &root, 35, 0).unwrap();
    println!("35 bytes of hex entropy:\n{:x?}", data);

    let xprv = my_bip85::to_xprv(&secp, &root, 0).unwrap();
    println!("Derived extended private key:\n{}", xprv);
}
