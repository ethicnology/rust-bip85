# BIP-85 Deterministic Entropy From BIP32 Keychains

[![codecov](https://codecov.io/gh/ethicnology/rust-bip85/branch/main/graph/badge.svg?token=RNIA9IIRB6)](https://codecov.io/gh/ethicnology/rust-bip85)

Derives entropy from the extended private key according to [BIP-85](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki).

This work is sponsored by [Bull Bitcoin](https://bullbitcoin.com) [<img 
    align="right"
    src="https://github.com/ethicnology/rust-bip85/blob/master/bindings/dart-bip85/bullbitcoin.png" 
    width=100
    title="Sponsor"
    alt="Sponsor"
/>](https://bullbitcoin.com)


## Features
- [x] [BIP39 mnemonic](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#user-content-BIP39)
- [x] [WIF](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#user-content-HDSeed_WIF)
- [x] [XPRV](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#user-content-XPRV)
- [x] [HEX](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#hex)
- [x] [DRNG](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#bip85-drng)
- [x] [PWD base64](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#user-content-PWD_BASE64)
- [x] [PWD base85](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#user-content-PWD_BASE85)
- [ ] [DICE](https://github.com/bitcoin/bips/blob/master/bip-0085.mediawiki#user-content-DICE)


## Flutter bindings
Thanks to flutter_rust_bridge, I've ported this rust library to [dart-bip85](https://pub.dev/packages/bip85) for flutter applications.

## Examples
There are a few examples in the `examples/` folder.

Running examples:
```sh
cargo run --example simple
```

```sh
cargo run --example mnemonic --features japanese
```

## Optional features

By default the library can derive entropy in any format specified by the standard except
mnemonics. To use mnemonics enable feature "mnemonic".

All bip-39 languages except english are also optional, so if you plan generating mnemonics in
japanese enable feature "japanese", and so on.
