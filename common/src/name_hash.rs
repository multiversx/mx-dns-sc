use multiversx_sc::{api::KECCAK256_RESULT_LEN, types::ManagedByteArray};

pub type NameHash<M> = ManagedByteArray<M, KECCAK256_RESULT_LEN>;
