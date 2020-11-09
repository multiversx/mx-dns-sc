use elrond_wasm::elrond_codec::*;
use elrond_wasm::Address;
use elrond_wasm::BoxedBytes;

#[derive(PartialEq, Debug)]
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

impl TopEncode for ValueState {
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        match self {
            ValueState::None => {
                output.set_slice_u8(&[]);
            }
            ValueState::Pending(addr) => {
                output.set_slice_u8(
                    BoxedBytes::from_concat(&[&[b'p'], addr.as_bytes()][..]).as_slice(),
                );
            }
            ValueState::Committed(addr) => {
                output.set_slice_u8(addr.as_bytes());
            }
        }
        Ok(())
    }

    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        _: ExitCtx,
        _: fn(ExitCtx, EncodeError) -> !,
    ) {
        match self {
            ValueState::None => {
                output.set_slice_u8(&[]);
            }
            ValueState::Pending(addr) => {
                output.set_slice_u8(
                    BoxedBytes::from_concat(&[&[b'p'], addr.as_bytes()][..]).as_slice(),
                );
            }
            ValueState::Committed(addr) => {
                output.set_slice_u8(addr.as_bytes());
            }
        }
    }
}

impl TopDecode for ValueState {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        match input.byte_len() {
            0 => Ok(ValueState::None),
            33 => {
                let bytes = input.into_boxed_slice_u8();
                let (prefix, address) = BoxedBytes::from(bytes).split(1);
                if prefix.as_slice()[0] == b'p' {
                    Ok(ValueState::Pending(Address::from_slice(address.as_slice())))
                } else {
                    Err(DecodeError::INVALID_VALUE)
                }
            }
            32 => Ok(ValueState::Committed(Address::top_decode(input)?)),
            _ => Err(DecodeError::INVALID_VALUE),
        }
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        match input.byte_len() {
            0 => ValueState::None,
            33 => {
                let bytes = input.into_boxed_slice_u8();
                let (prefix, address) = BoxedBytes::from(bytes).split(1);
                if prefix.as_slice()[0] == b'p' {
                    ValueState::Pending(Address::decode_from_boxed_bytes_or_exit(
                        address.into_box(),
                        c,
                        exit,
                    ))
                } else {
                    exit(c, DecodeError::INVALID_VALUE)
                }
            }
            32 => ValueState::Committed(Address::top_decode_or_exit(input, c, exit)),
            _ => exit(c, DecodeError::INVALID_VALUE),
        }
    }
}
