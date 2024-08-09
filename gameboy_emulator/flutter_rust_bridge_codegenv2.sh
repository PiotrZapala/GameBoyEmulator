#!/bin/bash

flutter_rust_bridge_codegen \
  -r rust_app/src/api.rs \
  -d flutter_app/lib/bridge_generated.dart\
  --dart-decl-output flutter_app/lib/bridge_definitions.dart \
  -c flutter_app/ios/Runner/bridge_generated.h