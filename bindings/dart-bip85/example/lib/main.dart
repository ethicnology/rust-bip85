import 'package:flutter/material.dart';
import 'package:bip85/bip85.dart' as bip85;

Future<void> main() async {
  await bip85.RustLib.init(); // mandatory
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    const xpriv =
        "xprv9s21ZrQH143K2LBWUUQRFXhucrQqBpKdRRxNVq2zBqsx8HVqFk2uYo8kmbaLLHRdqtQpUm98uKfu3vca1LqdGhUtyoFnCNkfmXRyPXLjbKb";

    var mnemonic = bip85.toMnemonic(
      xpriv: xpriv,
      wordCount: 12,
      index: 0,
    );

    print("bip85: $mnemonic");

    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(child: Text(mnemonic)),
      ),
    );
  }
}
