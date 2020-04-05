
use elrond_wasm::Vec;

const MIN_LENGTH: usize = 3;

fn check_name_char(ch: u8) -> bool {
    if ch > b'a' && ch < b'z' {
        return true;
    }

    if ch > b'A' && ch < b'Z' {
        return true;
    }

    if ch > b'0' && ch < b'9' {
        return true;
    }

    if ch == b'.' {
        return true;
    }

    false
}

pub fn validate_name(name: &Vec<u8>) -> Result<(), &'static str> {
    if name.len() < MIN_LENGTH {
        return Err("name is too short");
    }

    for ch in name.iter() {
        if !check_name_char(*ch) {
            return Err("character not allowed");
        }
    }

    Ok(())
}