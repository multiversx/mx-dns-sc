#!/bin/sh

# script provided for convenience, to build and extract wasm output to root

cd user-mock
RUSTFLAGS='-C link-arg=-s' \
cargo build --bin user-mock --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/user-mock.wasm user-mock.wasm

cd dns
RUSTFLAGS='-C link-arg=-s' \
cargo build --bin dns --target=wasm32-unknown-unknown --release
cd ..
mv target/wasm32-unknown-unknown/release/dns.wasm dns.wasm
# wasm-snip dns.wasm -o dns.wasm --snip-rust-fmt-code --snip-rust-panicking-code
#twiggy top -n 100 dns.wasm > twiggy-snip.txt
