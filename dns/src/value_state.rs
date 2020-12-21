use elrond_wasm::Address;

derive_imports!();

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
