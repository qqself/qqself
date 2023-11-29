#!/bin/sh

set -e

# Cleanup
rm -rf /tmp/qqselfCore

# Compile the bindings for all needed plaoforms. Using release as in debug encryption is very slow
cargo build --package qqself-core-bindings-c --release --target aarch64-apple-ios \
                                                       --target aarch64-apple-ios-sim \
                                                       --target x86_64-apple-ios
## Uniffi bindgen
cargo run --bin uniffi-bindgen generate ../core-bindings-c/src/qqself.udl --language swift --out-dir /tmp/qqselfCore

# Make fat lib for simulators
lipo -create \
    "../target/aarch64-apple-ios-sim/release/libqqself_core.a" \
    "../target/x86_64-apple-ios/release/libqqself_core.a" \
    -output "/tmp/qqselfCore/libqqself_core_universal.a"

# Move binaries and headers to the xcframework
cp "../target/aarch64-apple-ios/release/libqqself_core.a" "qqselfCore.xcframework/ios-arm64/qqselfCore.framework/qqselfCore"
cp "/tmp/qqselfCore/libqqself_core_universal.a" "qqselfCore.xcframework/ios-arm64_x86_64-simulator/qqselfCore.framework/qqselfCore"
cp "/tmp/qqselfCore/qqselfCoreFFI.h" "qqselfCore.xcframework/ios-arm64/qqselfCore.framework/Headers/qqselfCoreFFI.h"
cp "/tmp/qqselfCore/qqselfCoreFFI.h" "qqselfCore.xcframework/ios-arm64_x86_64-simulator/qqselfCore.framework/Headers/qqselfCoreFFI.h"

# Move swift interface and fix imports to reflect the framework name
sed "s/qqselfCoreFFI/qqselfCore/g" "/tmp/qqselfCore/qqselfCore.swift" > "qqself/qqselfCore.swift"
