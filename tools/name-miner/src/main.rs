use std::env;
use sha3::{Digest, Keccak256};

fn keccak256(data: &[u8]) -> [u8; 32] {
	let mut hasher = Keccak256::new();
	hasher.update(data);
	hasher.finalize().into()
}

/// Lists names that have a certain pattern and that map to a certain shard id.
/// Argument 1: name prefix
/// Argument 2: shard id in hex
/// Example run (result in the actual tests):
/// `cargo run coolname 87`
fn main() {
    let args: Vec<String> = env::args().collect();
    let prefix = &args[1];
	let desired_shard_raw = &args[2];
	let desired_shard = hex::decode(desired_shard_raw).unwrap();
	assert!(desired_shard.len() == 1, "shard id should be 1 byte");

    for i in 0..5000 {
        let name = format!("{}{:04}.elrond", prefix, i);
		let hash = keccak256(name.as_bytes());
		if hash[31] == desired_shard[0] {
			println!("{}", name);
		}
    }
}
