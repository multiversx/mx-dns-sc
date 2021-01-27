use dns::*;
use elrond_wasm_debug::*;

fn main() {
    let contract = DnsImpl::new(TxContext::dummy());
    print!("{}", abi_json::contract_abi(&contract));
}
