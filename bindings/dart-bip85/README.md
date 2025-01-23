# bip85

The [dart-bip85](https://github.com/ethicnology/rust-bip85/tree/master/bindings/dart-bip85) package is a Flutter binding for the [rust-bip85](https://github.com/ethicnology/rust-bip85) which is an updated version of the original [rust-bip85](https://github.com/rikitau/rust-bip85). The original rust-bip85 has been unmaintained since April 2021.



This work is sponsored by [Bull Bitcoin](https://bullbitcoin.com) [<img 
    align="right"
    src="https://github.com/ethicnology/rust-bip85/blob/master/bindings/dart-bip85/bullbitcoin.png" 
    width=100
    title="Sponsor"
    alt="Sponsor"
/>](https://bullbitcoin.com)


## Usage

I recommend checking the `example/` folder which includes a working app at `example/lib/main.dart`  as well as integration tests in `example/integration_test/`.

```dart
import 'package:flutter/material.dart';
import 'package:bip85/bip85.dart' as bip85;

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

    var derived = bip85.toMnemonic(xprv: xprv, wordCount: 12, index: 0);
    var expected =
        "girl mad pet galaxy egg matter matrix prison refuse sense ordinary nose";
    assert(derived == expected);

    derived = bip85.toWif(xprv: xprv, index: 0);
    expected = "Kzyv4uF39d4Jrw2W7UryTHwZr1zQVNk4dAFyqE6BuMrMh1Za7uhp";
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
            bip85.toMnemonic(xprv: xprv, wordCount: 12, index: 0),
          ),
        ),
      ),
    );
  }
}
```


## flutter_rust_bridge documentation

This project is a starting point for a Flutter
[FFI plugin](https://flutter.dev/to/ffi-package),
a specialized package that includes native code directly invoked with Dart FFI.

### Project structure

This template uses the following structure:

* `src`: Contains the native source code, and a CmakeFile.txt file for building
  that source code into a dynamic library.

* `lib`: Contains the Dart code that defines the API of the plugin, and which
  calls into the native code using `dart:ffi`.

* platform folders (`android`, `ios`, `windows`, etc.): Contains the build files
  for building and bundling the native code library with the platform application.

### Building and bundling native code

The `pubspec.yaml` specifies FFI plugins as follows:

```yaml
  plugin:
    platforms:
      some_platform:
        ffiPlugin: true
```

This configuration invokes the native build for the various target platforms
and bundles the binaries in Flutter applications using these FFI plugins.

This can be combined with dartPluginClass, such as when FFI is used for the
implementation of one platform in a federated plugin:

```yaml
  plugin:
    implements: some_other_plugin
    platforms:
      some_platform:
        dartPluginClass: SomeClass
        ffiPlugin: true
```

A plugin can have both FFI and method channels:

```yaml
  plugin:
    platforms:
      some_platform:
        pluginClass: SomeName
        ffiPlugin: true
```

The native build systems that are invoked by FFI (and method channel) plugins are:

* For Android: Gradle, which invokes the Android NDK for native builds.
  * See the documentation in android/build.gradle.
* For iOS and MacOS: Xcode, via CocoaPods.
  * See the documentation in ios/bip85.podspec.
  * See the documentation in macos/bip85.podspec.
* For Linux and Windows: CMake.
  * See the documentation in linux/CMakeLists.txt.
  * See the documentation in windows/CMakeLists.txt.

### Binding to native code

To use the native code, bindings in Dart are needed.
To avoid writing these by hand, they are generated from the header file
(`src/bip85.h`) by `package:ffigen`.
Regenerate the bindings by running `dart run ffigen --config ffigen.yaml`.

### Invoking native code

Very short-running native functions can be directly invoked from any isolate.
For example, see `sum` in `lib/bip85.dart`.

Longer-running functions should be invoked on a helper isolate to avoid
dropping frames in Flutter applications.
For example, see `sumAsync` in `lib/bip85.dart`.

