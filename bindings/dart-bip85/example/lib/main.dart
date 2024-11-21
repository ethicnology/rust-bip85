import 'package:bip85/bip85.dart';
import 'package:flutter/material.dart';
import 'package:bip85/bip85.dart' as bip85;

Future<void> main() async {
  await LibBip85.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    const xprv =
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb";
    final mnemonic = bip85.toMnemonic(xprv: xprv, wordCount: 12, index: 0);
    assert(mnemonic ==
        "girl mad pet galaxy egg matter matrix prison refuse sense ordinary nose");

    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge bip85')),
        body: Center(child: Text(mnemonic)),
      ),
    );
  }
}
