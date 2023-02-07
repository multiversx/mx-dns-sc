#!/bin/sh

# cleans all wasm targets

set -e
sc-meta all clean

# not wasm, but worth cleaning from time to time

cargo clean
cd elrond-wasm-node
cargo clean
cd ..
cd elrond-wasm-output
cargo clean
cd ..
