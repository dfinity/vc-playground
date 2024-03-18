#!/usr/bin/env bash
set -euo pipefail

# Make sure we always run from the issuer root
META_ISSUER_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$META_ISSUER_DIR"



# Build the canister
cargo build --release --target wasm32-unknown-unknown --manifest-path ./Cargo.toml -j1
ic-wasm "../target/wasm32-unknown-unknown/release/meta_issuer.wasm" -o "./meta_issuer.wasm" shrink
ic-wasm meta_issuer.wasm -o meta_issuer.wasm metadata candid:service -f meta_issuer.did -v public
# indicate support for certificate version 1 and 2 in the canister metadata
ic-wasm meta_issuer.wasm -o meta_issuer.wasm metadata supported_certificate_versions -d "1,2" -v public
gzip --no-name --force "meta_issuer.wasm"
mv meta_issuer.wasm.gz ../

