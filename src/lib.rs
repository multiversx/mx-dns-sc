
#![no_std]
#![no_main]
#![allow(non_snake_case)]
#![allow(unused_attributes)]


#[elrond_wasm_derive::contract(DnsImpl)]
pub trait Dns {

    fn init(&self) -> Result<(), &str> {

        Ok(())
    }


}
