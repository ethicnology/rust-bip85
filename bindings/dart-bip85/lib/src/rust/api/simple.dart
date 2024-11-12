// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

String toWif({required String xpriv, required int index}) =>
    RustLib.instance.api.crateApiSimpleToWif(xpriv: xpriv, index: index);

String toXprv({required String xpriv, required int index}) =>
    RustLib.instance.api.crateApiSimpleToXprv(xpriv: xpriv, index: index);

String toHex(
        {required String xpriv, required int length, required int index}) =>
    RustLib.instance.api
        .crateApiSimpleToHex(xpriv: xpriv, length: length, index: index);

String toMnemonic(
        {required String xpriv, required int wordCount, required int index}) =>
    RustLib.instance.api.crateApiSimpleToMnemonic(
        xpriv: xpriv, wordCount: wordCount, index: index);
