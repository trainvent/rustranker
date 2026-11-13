#!/usr/bin/env bash
# Builds the WebAssembly module for Node into pkg/.
# Needs the wasm32-unknown-unknown target and wasm-bindgen-cli at the same
# version as the wasm-bindgen crate in Cargo.lock.
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target nodejs --out-dir pkg target/wasm32-unknown-unknown/release/rustranker.wasm
