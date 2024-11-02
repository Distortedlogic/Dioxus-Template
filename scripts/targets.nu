#!/usr/bin/env nu

# Android targets
rustup target add aarch64-linux-android      # 64-bit ARM Android
rustup target add armv7-linux-androideabi    # 32-bit ARM Android
rustup target add x86_64-linux-android       # 64-bit x86 Android
rustup target add i686-linux-android         # 32-bit x86 Android
# iOS targets
rustup target add aarch64-apple-ios          # 64-bit ARM iOS
rustup target add x86_64-apple-ios          # iOS Simulator
# WebAssembly targets
rustup target add wasm32-unknown-unknown    # Pure WASM
rustup target add wasm32-wasi              # WASM with system interface
# Windows targets
rustup target add x86_64-pc-windows-msvc    # 64-bit Windows MSVC
rustup target add i686-pc-windows-msvc      # 32-bit Windows MSVC
rustup target add x86_64-pc-windows-gnu     # 64-bit Windows GNU
rustup target add i686-pc-windows-gnu       # 32-bit Windows GNU
# macOS targets
rustup target add x86_64-apple-darwin       # 64-bit Intel macOS
rustup target add aarch64-apple-darwin      # 64-bit ARM macOS (Apple Silicon)
# Linux targets
rustup target add x86_64-unknown-linux-gnu   # 64-bit Linux (GNU)
rustup target add i686-unknown-linux-gnu     # 32-bit Linux (GNU)
rustup target add aarch64-unknown-linux-gnu  # 64-bit ARM Linux (GNU)
rustup target add armv7-unknown-linux-gnueabihf  # 32-bit ARM Linux (GNU)