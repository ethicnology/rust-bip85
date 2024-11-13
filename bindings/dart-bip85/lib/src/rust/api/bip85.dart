// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.6.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Uint8List derive({required String xprv, required String path}) =>
    RustLib.instance.api.crateApiBip85Derive(xprv: xprv, path: path);

String toWif({required String xprv, required int index}) =>
    RustLib.instance.api.crateApiBip85ToWif(xprv: xprv, index: index);

String toXprv({required String xprv, required int index}) =>
    RustLib.instance.api.crateApiBip85ToXprv(xprv: xprv, index: index);

String toHex({required String xprv, required int length, required int index}) =>
    RustLib.instance.api
        .crateApiBip85ToHex(xprv: xprv, length: length, index: index);

String toMnemonic(
        {required String xprv, required int wordCount, required int index}) =>
    RustLib.instance.api.crateApiBip85ToMnemonic(
        xprv: xprv, wordCount: wordCount, index: index);