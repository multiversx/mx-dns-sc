elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const MIN_LENGTH: usize = 3;
pub const MAX_LENGTH: usize = 32;
const ELROND_SUFFIX: &[u8] = b".elrond";
const X_SUFFIX: &[u8] = b".x";

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Debug)]
pub enum SuffixType {
    Elrond,
    X,
}

fn check_name_char(ch: u8) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_digit()
}

pub fn validate_name<M: ManagedTypeApi>(name: &ManagedBuffer<M>) -> Result<(), &'static str> {
    let name_cache = NameCache::try_load(name)?;

    let name_without_suffix = name_cache.check_suffix(X_SUFFIX)?;
    validate_name_without_suffix(name_without_suffix)?;
    Result::Ok(())
}

fn validate_name_without_suffix(name_without_suffix: &[u8]) -> Result<(), &'static str> {
    if name_without_suffix.len() < MIN_LENGTH {
        return Result::Err("name is too short");
    }

    for ch in name_without_suffix.iter() {
        if !check_name_char(*ch) {
            return Result::Err("character not allowed");
        }
    }
    Result::Ok(())
}

// switch the suffix from .x to .elrond for the purpose of name hashing
// all other names (eg. using the .elrond suffix, or some invalid names), are not modified
pub fn prepare_name_for_hash_and_classify<M: ManagedTypeApi>(
    name: &ManagedBuffer<M>,
) -> (ManagedBuffer<M>, SuffixType) {
    try_replace_suffix(name, X_SUFFIX, ELROND_SUFFIX).map_or_else(
        |_| (name.clone(), SuffixType::Elrond),
        |new_name| (new_name, SuffixType::X),
    )
}

fn try_replace_suffix<M: ManagedTypeApi>(
    name: &ManagedBuffer<M>,
    original_suffix: &'static [u8],
    new_suffix: &'static [u8],
) -> Result<ManagedBuffer<M>, &'static str> {
    let name_cache = NameCache::try_load(name)?;
    let name_without_suffix = name_cache.check_suffix(original_suffix)?;
    let new_name = build_name(name_without_suffix, new_suffix);
    Result::Ok(new_name)
}

// ensure the suffix is .elrond and replace it with .x
// validate the name without the prefix as well
pub fn prepare_and_validate_name_for_migration<M: ManagedTypeApi>(
    name: &ManagedBuffer<M>,
) -> Result<ManagedBuffer<M>, &'static str> {
    let name_cache = NameCache::try_load(name)?;
    let name_without_suffix = name_cache.check_suffix(ELROND_SUFFIX)?;
    validate_name_without_suffix(name_without_suffix)?;
    let new_name = build_name(name_without_suffix, X_SUFFIX);
    Result::Ok(new_name)
}

fn build_name<M: ManagedTypeApi>(name_without_suffix: &[u8], suffix: &[u8]) -> ManagedBuffer<M> {
    let mut name = ManagedBuffer::from(name_without_suffix);
    name.append_bytes(suffix);
    name
}

struct NameCache {
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

    pub fn as_slice(&self) -> &[u8] {
        &self.name_bytes[..self.name_len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.name_bytes[..self.name_len]
    }

    pub fn check_suffix(&self, suffix: &[u8]) -> Result<&[u8], &'static str> {
        if self.name_len < suffix.len() {
            return Result::Err("name does not contain suffix");
        }

        let (name_without_suffix, name_suffix) =
            self.as_slice().split_at(self.name_len - suffix.len());

        if name_suffix != suffix {
            return Result::Err("wrong suffix");
        }

        Result::Ok(name_without_suffix)
    }
}
