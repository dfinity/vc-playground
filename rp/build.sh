#!/usr/bin/env bash
set -euo pipefail

# Make sure we always run from the issuer root
RP_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$RP_DIR"

cd frontend/
npm ci
npm run build
cp -r ./static/images ./frontend/dist/
cd ..

# Build the canister
cargo build --release --target wasm32-unknown-unknown --manifest-path ./Cargo.toml -j1
ic-wasm "../target/wasm32-unknown-unknown/release/relying_party.wasm" -o "./relying_party.wasm" shrink
ic-wasm relying_party.wasm -o relying_party.wasm metadata candid:service -f rp.did -v public
# indicate support for certificate version 1 and 2 in the canister metadata
ic-wasm relying_party.wasm -o relying_party.wasm metadata supported_certificate_versions -d "1,2" -v public
gzip --no-name --force "relying_party.wasm"
mv relying_party.wasm.gz ../
