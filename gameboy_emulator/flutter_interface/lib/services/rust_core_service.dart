import 'dart:ffi' as ffi;
import 'dart:io' show Platform;
import 'package:flutter_interface/bridge_generated.dart';

class RustCoreService {
  static final RustCoreImpl _instance = RustCoreImpl(
    _loadDynamicLibrary(),
  );

  static RustCoreImpl get instance => _instance;

  static ffi.DynamicLibrary _loadDynamicLibrary() {
    if (Platform.isIOS) {
      return ffi.DynamicLibrary.process();
    } else if (Platform.isAndroid) {
      return ffi.DynamicLibrary.open('librust_core.so');
    } else {
      throw UnsupportedError('Platform not supported');
    }
  }
}
