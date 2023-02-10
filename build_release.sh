#!/usr/bin/env sh

# Helper one-liner to build a release-ready version of the provider

cargo build --release && wasm-opt -Oz -c -o "./catnip.wasm" "target/wasm32-unknown-unknown/release/catnip_provider.wasm"
