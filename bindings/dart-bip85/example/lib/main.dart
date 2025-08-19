import 'package:bip85/bip85.dart';
import 'package:flutter/material.dart';
import 'package:bip85/bip85.dart' as bip85;
import 'package:collection/collection.dart';

Future<void> main() async {
  await bip85.LibBip85.init(); // mandatory
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    const xprv =
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb";

    const language = bip85.Language.english;
    var derivedMnemonic = bip85.toMnemonicIn(
        xprv: xprv, language: language.label, wordCount: 12, index: 0);
    var expectedMnemonic = Mnemonic.fromSentence(
        "girl mad pet galaxy egg matter matrix prison refuse sense ordinary nose",
        language);
    assert(ListEquality().equals(derivedMnemonic, expectedMnemonic.words));

    var derived = bip85.toWif(xprv: xprv, index: 0);
    var expected = "Kzyv4uF39d4Jrw2W7UryTHwZr1zQVNk4dAFyqE6BuMrMh1Za7uhp";
    assert(derived == expected);

    derived = bip85.toHex(xprv: xprv, length: 64, index: 0);
    expected =
        "492db4698cf3b73a5a24998aa3e9d7fa96275d85724a91e71aa2d645442f878555d078fd1f1f67e368976f04137b1f7a0d19232136ca50c44614af72b5582a5c";
    assert(derived == expected);

    derived = bip85.toXprv(xprv: xprv, index: 0);
    expected =
        "xprv9s21ZrQH143K2srSbCSg4m4kLvPMzcWydgmKEnMmoZUurYuBuYG46c6P71UGXMzmriLzCCBvKQWBUv3vPB3m1SATMhp3uEjXHJ42jFg7myX";
    assert(derived == expected);

    derived = bip85.toBase64(xprv: xprv, length: 21, index: 0);
    expected = "dKLoepugzdVJvdL56ogNV";
    assert(derived == expected);

    derived = bip85.toBase85(xprv: xprv, length: 12, index: 0);
    expected = "_s`{TW89)i4`";
    assert(derived == expected);

    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge bip85')),
        body: Center(
          child: Text(
            Mnemonic.fromWords(
              words: bip85.toMnemonic(xprv: xprv, wordCount: 12, index: 0),
              language: language,
            ).sentence,
          ),
        ),
      ),
    );
  }
}
