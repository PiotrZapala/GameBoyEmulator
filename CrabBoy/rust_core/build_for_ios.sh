#!/bin/bash

cargo lipo --release
cp target/universal/release/librust_core.a ../flutter_interface/ios/Runner