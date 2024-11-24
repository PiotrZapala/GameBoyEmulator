#!/bin/bash

cargo lipo --release
cp target/universal/release/librust_app.a ../flutter_app/ios/Runner