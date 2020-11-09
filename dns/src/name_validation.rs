imports!();

const MIN_LENGTH: usize = 10;

fn check_name_char(ch: u8) -> bool {
    if ch >= b'a' && ch <= b'z' {
        return true;
    }

    if ch >= b'0' && ch <= b'9' {
        return true;
    }

    false
}

pub fn validate_name(name: &[u8]) -> SCResult<()> {
    if name.len() < MIN_LENGTH {
        return sc_error!("name is too short");
    }

    for ch in name.iter() {
        if !check_name_char(*ch) {
            return sc_error!("character not allowed");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name() {
        // ok
        assert!(validate_name(&*b"aaaaaaaaaa").is_ok());
        assert!(validate_name(&*b"zzzzzzzzzz").is_ok());
        assert!(validate_name(&*b"0000000000").is_ok());
        assert!(validate_name(&*b"9999999999").is_ok());
        assert!(validate_name(&*b"coolname0001").is_ok());

        // too short
        assert!(!validate_name(&*b"").is_ok());
        assert!(!validate_name(&*b"aaaaaaaaa").is_ok());

        // lowercase only
        assert!(!validate_name(&*b"Aaaaaaaaaa").is_ok());

        // no other chars
        assert!(!validate_name(&*b"Aaaaa.aaaa").is_ok());
        assert!(!validate_name(&*b"Aaaaa@aaaa").is_ok());
        assert!(!validate_name(&*b"Aaaaa+aaaa").is_ok());
        assert!(!validate_name(&*b"Aaaaa-aaaa").is_ok());
        assert!(!validate_name(&*b"Aaaaa_aaaa").is_ok());
    }
}
