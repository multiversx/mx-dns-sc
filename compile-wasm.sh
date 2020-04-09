#!/bin/sh

# script provided for convenience, to build and extract wasm output to root

cargo build --bin dns --target=wasm32-unknown-unknown --release
wasm-snip target/wasm32-unknown-unknown/release/dns.wasm -o dns.wasm --snip-rust-fmt-code --snip-rust-panicking-code
#twiggy top -n 100 dns.wasm > twiggy-snip.txt

cd deployer
cargo build --bin deployer --target=wasm32-unknown-unknown --release
cd ..
wasm-snip deployer/target/wasm32-unknown-unknown/release/deployer.wasm -o deployer.wasm --snip-rust-fmt-code --snip-rust-panicking-code

cd user-mock
cargo build --bin user-mock --target=wasm32-unknown-unknown --release
cd ..
wasm-snip user-mock/target/wasm32-unknown-unknown/release/user-mock.wasm -o user-mock.wasm --snip-rust-fmt-code --snip-rust-panicking-code
