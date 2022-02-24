elrond_wasm::imports!();

const MIN_LENGTH: usize = 3;
const MAX_LENGTH: usize = 32;
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

pub fn validate_name(name: &[u8]) -> Result<(), &'static str> {
    if name.len() <= NAME_SUFFIX.len() {
        return Result::Err("name does not contain suffix");
    }

    if name.len() > MAX_LENGTH {
        return Result::Err("name too long");
    }

    let (name_without_suffix, suffix) = name.split_at(name.len() - NAME_SUFFIX.len());

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name() {
        // ok
        assert!(validate_name(&*b"aaaaaaaaaa.elrond").is_ok());
        assert!(validate_name(&*b"zzzzzzzzzz.elrond").is_ok());
        assert!(validate_name(&*b"0000000000.elrond").is_ok());
        assert!(validate_name(&*b"9999999999.elrond").is_ok());
        assert!(validate_name(&*b"coolname0001.elrond").is_ok());

        // too short
        assert!(!validate_name(&*b".elrond").is_ok());
        assert!(!validate_name(&*b"aa.elrond").is_ok());

        // lowercase only
        assert!(!validate_name(&*b"Aaaaaaaaaa.elrond").is_ok());

        // no other chars
        assert!(!validate_name(&*b"Aaaaa.aaaa.elrond").is_ok());
        assert!(!validate_name(&*b"Aaaaa@aaaa.elrond").is_ok());
        assert!(!validate_name(&*b"Aaaaa+aaaa.elrond").is_ok());
        assert!(!validate_name(&*b"Aaaaa-aaaa.elrond").is_ok());
        assert!(!validate_name(&*b"Aaaaa_aaaa.elrond").is_ok());

        // without suffix
        assert!(!validate_name(&*b"aaaaaaaaaa").is_ok());
        assert!(!validate_name(&*b"zzzzzzzzzz").is_ok());
        assert!(!validate_name(&*b"0000000000").is_ok());
        assert!(!validate_name(&*b"9999999999").is_ok());
        assert!(!validate_name(&*b"coolname0001").is_ok());

        // name too long
        assert!(!validate_name(&*b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.elrond").is_ok());
    }
}
