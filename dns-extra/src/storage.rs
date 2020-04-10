
use elrond_wasm::Vec;
use elrond_wasm::{Address, StorageKey};

pub static OWNER_KEY:            [u8; 32] = [0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
pub static SHARD_BITS_KEY:       [u8; 32] = [0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
pub static USER_STORAGE_KEY_KEY: [u8; 32] = [0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
pub static SIBLING_ADDR_PREFIX:  u8 = 3;

pub static RESERVED_TOKEN:       u8 = 1;

// constructs keys for user data
pub fn sibling_addr_key(shard_id: u8) -> StorageKey {
    let mut key = [0u8; 32];
    key[0] = SIBLING_ADDR_PREFIX;
    key[31] = shard_id;
    key.into()
}

pub enum ValueState {
    None,
    Pending(Address),
    Committed(Address),
}

impl ValueState {
    pub fn is_available(&self) -> bool {
        if let ValueState::None = self {
            true
        } else {
            false
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        match bytes.len() {
            33 => ValueState::Pending(Address::from_slice(&bytes[1..])),
            32 => ValueState::Committed(Address::from_slice(&bytes[..])),
            _ => ValueState::None,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            ValueState::None => Vec::with_capacity(0),
            ValueState::Pending(addr) => {
                let mut result = Vec::with_capacity(33);
                result.push(b'p');
                result.extend_from_slice(addr.as_bytes());
                result
            },
            ValueState::Committed(addr) => {
                addr.as_bytes().to_vec()
            },
        }
    }
}
