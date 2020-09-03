#![no_main]
use libfuzzer_sys::fuzz_target;

use elrond_wasm::elrond_codec::*;
use dns::value_state::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(decoded) = ValueState::top_decode(&mut &data[..]) {
        let re_encoded = decoded.top_encode().unwrap();
        assert_eq!(re_encoded, data);
    }
});
