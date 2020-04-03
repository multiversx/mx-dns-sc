
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

#[elrond_wasm_derive::contract(DnsDeployerImpl)]
pub trait DnsDeployer {

    fn init(&self) {
    }

    fn deploy(&self) {
    }

}
