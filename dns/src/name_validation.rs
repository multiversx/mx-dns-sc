elrond_wasm::imports!();

const MIN_LENGTH: usize = 3;
pub const MAX_LENGTH: usize = 32;
const NAME_SUFFIX: &[u8] = b".elrond";

#[allow(clippy::manual_range_contains)]
fn check_name_char(ch: u8) -> bool {
    if ch >= b'a' && ch <= b'z' {
        return true;
    }

    if ch >= b'0' && ch <= b'9' {
        return true;
    }

    false
}

pub fn validate_name<M: ManagedTypeApi>(name: &ManagedBuffer<M>) -> Result<(), &'static str> {
    let name_len = name.len();
    if name_len <= NAME_SUFFIX.len() {
        return Result::Err("name does not contain suffix");
    }

    if name_len > MAX_LENGTH {
        return Result::Err("name too long");
    }

    let mut name_bytes = [0u8; MAX_LENGTH];
    let name_slice = &mut name_bytes[..name_len];
    if name.load_slice(0, name_slice).is_err() {
        return Result::Err("error loading name bytes");
    }

    let (name_without_suffix, suffix) = name_slice.split_at(name.len() - NAME_SUFFIX.len());

    if suffix != NAME_SUFFIX {
        return Result::Err("wrong suffix");
    }

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
