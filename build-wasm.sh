#!/bin/sh

# until we have the new version of erdpy

cd dns/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/dns_wasm.wasm output/dns.wasm
cd ..

cd user-mock/wasm
RUSTFLAGS='-C link-arg=-s' \
cargo build --target=wasm32-unknown-unknown --release
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/user_mock_wasm.wasm output/user-mock.wasm
cd ..
