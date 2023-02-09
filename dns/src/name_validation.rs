use common::name_cache::NameCache;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

const MIN_LENGTH: usize = 3;
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

    let name_without_suffix = check_suffix_and_get_name(&name_cache, X_SUFFIX)?;
    validate_name_without_suffix(name_without_suffix)
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
    let name_without_suffix = check_suffix_and_get_name(&name_cache, original_suffix)?;
    let new_name = build_name(name_without_suffix, new_suffix);
    Result::Ok(new_name)
}

// ensure the suffix is .elrond and replace it with .x
// validate the name without the prefix as well
pub fn prepare_and_validate_name_for_migration<M: ManagedTypeApi>(
    name: &ManagedBuffer<M>,
) -> Result<ManagedBuffer<M>, &'static str> {
    let name_cache = NameCache::try_load(name)?;
    let name_without_suffix = check_suffix_and_get_name(&name_cache, ELROND_SUFFIX)?;
    validate_name_without_suffix(name_without_suffix)?;
    let new_name = build_name(name_without_suffix, X_SUFFIX);
    Result::Ok(new_name)
}

fn build_name<M: ManagedTypeApi>(name_without_suffix: &[u8], suffix: &[u8]) -> ManagedBuffer<M> {
    let mut name = ManagedBuffer::from(name_without_suffix);
    name.append_bytes(suffix);
    name
}

pub fn check_suffix_and_get_name<'a>(
    name_cache: &'a NameCache,
    suffix: &[u8],
) -> Result<&'a [u8], &'static str> {
    if name_cache.len() < suffix.len() {
        return Result::Err("name does not contain suffix");
    }

    let (name_without_suffix, name_suffix) = name_cache
        .as_slice()
        .split_at(name_cache.len() - suffix.len());

    if name_suffix != suffix {
        return Result::Err("wrong suffix");
    }

    Result::Ok(name_without_suffix)
}
