## 3.0.0

- feat: `toMnemonicIn` generates the mnemonic for all the languages available in bip39 spec
- BREAKING CHANGE: `toMnemonic` return the words of the mnemonic as a `List<String>` instead of a `String` this change is justified by the fact that the `rust-bip39` dependency does not use ideographic spaces instead of ASCII space for japanese mnemonic as expected in the specification/test vectors.
- ci: integration test in dart
- refactor: set `fvm` in the repo with flutter `3.35.0`


## 2.0.0

- Upgrade: flutter_rust_bridge 2.9.0

## 1.0.3

- Improve precompiled binaries
- Bindings for PWD64 and PWD85
- Update docs

## 1.0.1

- Downgrade Flutter_Rust_Bridge version to ^2.0.0 to avoid incompatibility issues with other packages

## 1.0.2

- Publish bip85-extended to crates.io to avoid import issues
- Update readme

## 1.0.0

- Initial version


