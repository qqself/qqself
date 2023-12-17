.PHONY: build

deps:
	brew install swift-format
	swift-format --version

build:
	xcodebuild build -project qqself.xcodeproj -scheme qqself -destination "platform=iOS Simulator,OS=17.0.1,name=iPhone 15 Pro" CODE_SIGNING_REQUIRED=NO

test:
	xcodebuild test  -project qqself.xcodeproj -scheme qqself -destination "platform=iOS Simulator,OS=17.0.1,name=iPhone 15 Pro" CODE_SIGNING_REQUIRED=NO

format-check:
	swift-format lint --recursive --parallel --strict qqself qqselfTests

format-fix:
	swift-format format --recursive --parallel --in-place qqself qqselfTests

bindings:
	# Cleanup
	rm -rf /tmp/qqselfCore
	# Ensure all Rust targets are installed
	rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
	# Compile the bindings for all needed plaoforms. Using release as in debug encryption is very slow
	cargo build --package qqself-core-bindings-c --release --target aarch64-apple-ios \
														   --target aarch64-apple-ios-sim \
														   --target x86_64-apple-ios
	# Uniffi bindgen
	cargo run --bin uniffi-bindgen generate ../core-bindings-c/src/qqself.udl --language swift --out-dir /tmp/qqselfCore
	# Make fat lib for simulators
	lipo -create \
		"../target/aarch64-apple-ios-sim/release/libqqself_core.a" \
		"../target/x86_64-apple-ios/release/libqqself_core.a" \
		-output "/tmp/qqselfCore/libqqself_core_universal.a"
	# Move binaries and headers to the xcframework
	cp "../target/aarch64-apple-ios/release/libqqself_core.a" "qqselfCoreLib/artifacts/qqselfCore.xcframework/ios-arm64/qqselfCore.framework/qqselfCore"
	cp "/tmp/qqselfCore/libqqself_core_universal.a" "qqselfCoreLib/artifacts/qqselfCore.xcframework/ios-arm64_x86_64-simulator/qqselfCore.framework/qqselfCore"
	cp "/tmp/qqselfCore/qqselfCoreFFI.h" "qqselfCoreLib/artifacts/qqselfCore.xcframework/ios-arm64/qqselfCore.framework/Headers/qqselfCoreFFI.h"
	cp "/tmp/qqselfCore/qqselfCoreFFI.h" "qqselfCoreLib/artifacts/qqselfCore.xcframework/ios-arm64_x86_64-simulator/qqselfCore.framework/Headers/qqselfCoreFFI.h"
	# Move swift interface and fix imports to reflect the framework name
	mkdir -p "qqselfCoreLib/Sources/qqselfCoreLib"
	sed "s/qqselfCoreFFI/qqselfCore/g" "/tmp/qqselfCore/qqselfCore.swift" > "qqselfCoreLib/Sources/qqselfCoreLib/qqselfCoreLib.swift"
