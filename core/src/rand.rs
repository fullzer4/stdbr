#[cfg(feature = "std")]
pub(crate) fn simple_seed() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish()
}

#[cfg(not(feature = "std"))]
pub(crate) fn simple_seed() -> u64 {
    let stack_var: u8 = 0;
    let addr = &stack_var as *const u8 as u64;
    addr.wrapping_mul(6_364_136_223_846_793_005)
        .wrapping_add(1_442_695_040_888_963_407)
}

pub(crate) fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}
