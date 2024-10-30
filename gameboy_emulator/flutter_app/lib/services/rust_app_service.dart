import 'dart:ffi' as ffi;
import 'dart:io' show Platform;
import 'package:flutter_app/bridge_generated.dart';

class RustAppService {
  static final RustAppImpl _instance = RustAppImpl(
    Platform.isIOS
        ? ffi.DynamicLibrary.process()
        : ffi.DynamicLibrary.open('librust_app.so'),
  );

  static RustAppImpl get instance => _instance;
}
