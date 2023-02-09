use crate::name_cache::NameCache;

multiversx_sc::imports!();

static ERR_INVALID_USERNAME_FORMAT: &[u8] = b"Invalid username format, expected name.domain";

fn get_prefix_domain<M: ManagedTypeApi + ErrorApi>(
    username: &ManagedBuffer<M>,
) -> (ManagedBuffer<M>, ManagedBuffer<M>) {
    let cache = NameCache::try_load(username)
        .unwrap_or_else(|err| M::error_api_impl().signal_error(err.as_bytes()));

    let mut parts_iterator = cache
        .as_slice()
        .split(|c| c == &b'.')
        .map(ManagedBuffer::new_from_bytes);
    let [prefix, domain] = parts_iterator
        .next_chunk()
        .unwrap_or_else(|_| M::error_api_impl().signal_error(ERR_INVALID_USERNAME_FORMAT));

    if parts_iterator.count() != 0 {
        M::error_api_impl().signal_error(ERR_INVALID_USERNAME_FORMAT);
    }

    (prefix, domain)
}

pub fn get_domain<M: ManagedTypeApi + ErrorApi>(username: &ManagedBuffer<M>) -> ManagedBuffer<M> {
    let (_, domain) = get_prefix_domain(username);
    domain
}
