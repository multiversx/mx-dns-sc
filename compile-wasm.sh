#!/bin/sh

# script provided for convenience, to build and extract wasm output to root

cargo build --bin dns --target=wasm32-unknown-unknown --release
mv target/wasm32-unknown-unknown/release/dns.wasm dns.wasm
wasm-snip dns.wasm -o dns.wasm --snip-rust-fmt-code --snip-rust-panicking-code
#twiggy top -n 100 dns.wasm > twiggy-snip.txt

cd deployer
cargo build --bin deployer --target=wasm32-unknown-unknown --release
cd ..
mv deployer/target/wasm32-unknown-unknown/release/deployer.wasm deployer.wasm
wasm-snip deployer.wasm -o deployer.wasm --snip-rust-fmt-code --snip-rust-panicking-code
