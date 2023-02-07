# sc-dns-rs
MultiversX DNS smart contract, written in Rust



# Build

Temporarily until erdpy is updated, use the following to build:
```
./build-wasm.sh 
```
It will produce dns.wasm and user-mock.wasm (a dummy used in tests instead of the user API).

# Rust configuration

```
rustup toolchain install nightly
rustup target add wasm32-unknown-unknown
rustc --version
```

# Advanced

Wasm is built with this line:
```
cargo build --bin wasm --target=wasm32-unknown-unknown --release
```

To debug macros:
- wasm mode:
```
cargo +nightly rustc --bin wasm -- -Z unstable-options --pretty=expanded > demacroed.rs
```

- debug mode:
```
cargo +nightly rustc --lib -- -Z unstable-options --pretty=expanded > demacroed.rs
```

To check wasm size:
```
twiggy top -n 20 target/wasm32-unknown-unknown/release/wasm.wasm
```

To work with unpublished mx-sdk-rs crates, clone https://github.com/multiversx/mx-sdk-rs in the same parent directory and replace dependencies in Cargo.toml with:
```
multiversx-sc = { path = "../mx-sdk-rs/framework/base" }
multiversx-sc-wasm-adapter = { path = "../mx-sdk-rs/framework/wasm-adapter" }
```

And the same for debug:
```
multiversx-sc-scenario = { path = "../mx-sdk-rs/framework/scenario" }
```
