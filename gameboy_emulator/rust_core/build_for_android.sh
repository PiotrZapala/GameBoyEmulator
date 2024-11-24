#!/bin/bash

cargo ndk -t arm64-v8a -t armeabi-v7a -t x86 -t x86_64 -o ../flutter_interface/android/app/src/main/jniLibs build --release