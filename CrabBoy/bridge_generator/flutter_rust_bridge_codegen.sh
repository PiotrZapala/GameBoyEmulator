#!/bin/bash

flutter_rust_bridge_codegen \
  -r ../rust_core/src/api.rs \
  -d ../flutter_interface/lib/bridge_generated.dart\
  --dart-decl-output ../flutter_interface/lib/bridge_definitions.dart
