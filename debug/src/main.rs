
#![allow(unused_attributes)]

use sc_dns_rs::*;
use elrond_wasm_debug::*;
use elrond_wasm_debug::HashMap;

#[cfg(test)]
mod name_test;

static ADDR1: [u8; 32] = [0x11u8; 32];
static ADDR2: [u8; 32] = [0x01u8; 32];

fn main() {
    let mock_ref = ArwenMockState::new();

    mock_ref.add_account(AccountData{
        address: ADDR1.into(),
        nonce: 0,
        balance: 0.into(),
        storage: HashMap::new(),
        contract: None,
    });

    // tx 1: init
    let mut tx1 = TxData::new_create(
        Box::new(DnsImpl::new(mock_ref.clone())), 
        ADDR1.into(), 
        ADDR2.into());
    tx1.add_arg(vec![2u8]);
    mock_ref.execute_tx(tx1);

    // tx 2: execute
    let mut tx2 = TxData::new_call(
        "register", 
        ADDR1.into(),
        ADDR2.into());
    tx2.add_arg(b"coolname0001".to_vec());
    tx2.add_arg(vec![1u8; 32]);
    let result2 = mock_ref.execute_tx(tx2);
    assert_eq!(0, result2.result_status);
    result2.print();

}
