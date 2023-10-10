import 'dart:ffi';
import 'package:ffi/ffi.dart';

String getKeyUsingFfi() {
  DynamicLibrary dl = DynamicLibrary.open("target/debug/libwallet_sdk_rust.so");

  final Pointer<Utf8> Function() getKey =
      dl.lookupFunction<Pointer<Utf8> Function(), Pointer<Utf8> Function()>(
          'get_key');

  return getKey().toDartString();
}

/// Checks if you are awesome. Spoiler: you are.
class Awesome {
  bool get isAwesome => true;
}
