multiversx_sc::imports!();

const MAX_LENGTH: usize = 32;

pub struct NameCache {
    name_bytes: [u8; MAX_LENGTH],
    name_len: usize,
}

impl NameCache {
    pub fn try_load<M: ManagedTypeApi>(name: &ManagedBuffer<M>) -> Result<Self, &'static str> {
        let name_len = name.len();
        if name_len > MAX_LENGTH {
            return Result::Err("name too long");
        }

        let mut name_cache = Self {
            name_bytes: [0u8; MAX_LENGTH],
            name_len,
        };

        let name_slice = name_cache.as_mut_slice();
        if name.load_slice(0, name_slice).is_err() {
            return Result::Err("error loading name bytes");
        }
        Result::Ok(name_cache)
    }

    pub fn len(&self) -> usize {
        self.name_len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.name_bytes[..self.name_len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.name_bytes[..self.name_len]
    }
}
