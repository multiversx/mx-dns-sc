#!/bin/sh

# alternative to erdpy

export RUSTFLAGS=${RUSTFLAGS-'-C link-arg=-s'}

cd dns/wasm
cargo build --target=wasm32-unknown-unknown --release | return 1
cd ..
mkdir -p output
cp wasm/target/wasm32-unknown-unknown/release/dns_wasm.wasm output/dns.wasm
cd ..
