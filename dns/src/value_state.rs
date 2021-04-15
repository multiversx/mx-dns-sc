use elrond_wasm::types::Address;

elrond_wasm::derive_imports!();

/// Copied from elrond-wasm serialization tests.
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Debug)]
pub enum ValueState {
    None,
    Pending(Address),
    Committed(Address),
}

impl ValueState {
    pub fn is_available(&self) -> bool {
        matches!(self, ValueState::None)
    }
}
