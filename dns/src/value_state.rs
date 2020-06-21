
use elrond_wasm::esd_light::*;
use elrond_wasm::Vec;
use elrond_wasm::Address;

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
}

impl Encode for ValueState {
	#[inline]
	fn using_top_encoded<F: FnOnce(&[u8])>(&self, f: F) {
        match self {
            ValueState::None => {},
            ValueState::Pending(addr) => {
                let mut result = Vec::with_capacity(33);
                result.push(b'p');
                result.extend_from_slice(addr.as_bytes());
                f(result.as_slice());
            },
            ValueState::Committed(addr) => {
                f(addr.as_bytes());
            },
        }
	}
}

impl Decode for ValueState {
	#[inline]
	fn top_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        let bytes = input.flush()?;
        match bytes.len() {
            0 => Ok(ValueState::None),
            33 => {
                if bytes[0] == b'p' {
                    Ok(ValueState::Pending(Address::from_slice(&bytes[1..])))
                } else {
                    Err(DecodeError::InvalidValue)
                }
            },
            32 => Ok(ValueState::Committed(Address::from_slice(&bytes[..]))),
            _ => Err(DecodeError::InvalidValue),
        }
    }
    
    #[inline]
	fn dep_decode<I: Input>(_input: &mut I) -> Result<Self, DecodeError> {
        Err(DecodeError::UnsupportedOperation)
    }
}
