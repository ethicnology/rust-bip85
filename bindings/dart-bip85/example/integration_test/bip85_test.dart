import 'package:flutter_test/flutter_test.dart';
import 'package:bip85/bip85.dart' as bip85;
import 'package:integration_test/integration_test.dart';

void main() async {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  const xprv =
      "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb";

  setUpAll(() async => await bip85.LibBip85.init());

  test('mnemonic', () {
    final derived = bip85.toMnemonic(xprv: xprv, wordCount: 12, index: 0);
    const expected =
        "girl mad pet galaxy egg matter matrix prison refuse sense ordinary nose";

    expect(derived, expected);
  });

  test('wif', () {
    final derived = bip85.toWif(xprv: xprv, index: 0);
    const expected = "Kzyv4uF39d4Jrw2W7UryTHwZr1zQVNk4dAFyqE6BuMrMh1Za7uhp";

    expect(derived, expected);
  });

  test('hex', () {
    final derived = bip85.toHex(xprv: xprv, length: 64, index: 0);
    const expected =
        "492db4698cf3b73a5a24998aa3e9d7fa96275d85724a91e71aa2d645442f878555d078fd1f1f67e368976f04137b1f7a0d19232136ca50c44614af72b5582a5c";

    expect(derived, expected);
  });

  test('xpriv', () {
    final derived = bip85.toXprv(xprv: xprv, index: 0);
    const expected =
        "xprv9s21ZrQH143K2srSbCSg4m4kLvPMzcWydgmKEnMmoZUurYuBuYG46c6P71UGXMzmriLzCCBvKQWBUv3vPB3m1SATMhp3uEjXHJ42jFg7myX";

    expect(derived, expected);
  });

  test('toBase64', () {
    final derived = bip85.toBase64(xprv: xprv, length: 21, index: 0);
    const expected = "dKLoepugzdVJvdL56ogNV";
    expect(derived, expected);
  });

  test('toBase85', () {
    final derived = bip85.toBase85(xprv: xprv, length: 12, index: 0);
    const expected = "_s`{TW89)i4`";
    expect(derived, expected);
  });

  test('derive', () {
    final derived = bip85.derive(xprv: xprv, path: "m/0'/0'");
    final expected = [
      0xef,
      0xec,
      0xfb,
      0xcc,
      0xff,
      0xea,
      0x31,
      0x32,
      0x14,
      0x23,
      0x2d,
      0x29,
      0xe7,
      0x15,
      0x63,
      0xd9,
      0x41,
      0x22,
      0x9a,
      0xfb,
      0x43,
      0x38,
      0xc2,
      0x1f,
      0x95,
      0x17,
      0xc4,
      0x1a,
      0xaa,
      0x0d,
      0x16,
      0xf0,
      0x0b,
      0x83,
      0xd2,
      0xa0,
      0x9e,
      0xf7,
      0x47,
      0xe7,
      0xa6,
      0x4e,
      0x8e,
      0x2b,
      0xd5,
      0xa1,
      0x48,
      0x69,
      0xe6,
      0x93,
      0xda,
      0x66,
      0xce,
      0x94,
      0xac,
      0x2d,
      0xa5,
      0x70,
      0xab,
      0x7e,
      0xe4,
      0x86,
      0x18,
      0xf7
    ];

    expect(derived, expected);
  });
}
