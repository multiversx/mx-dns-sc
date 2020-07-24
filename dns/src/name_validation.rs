imports!();

const MIN_LENGTH: usize = 3;

fn check_name_char(ch: u8) -> bool {
    if ch >= b'a' && ch <= b'z' {
        return true;
    }

    if ch >= b'A' && ch <= b'Z' {
        return true;
    }

    if ch >= b'0' && ch <= b'9' {
        return true;
    }

    if ch == b'.' {
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
