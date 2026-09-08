//! Small non-cryptographic random sources used by document generators.

const ZERO_SEED_FALLBACK: u64 = 0x9E37_79B9_7F4A_7C15;

/// A minimal source of random bits for document generation.
///
/// Implement this trait to inject an application-provided random source into
/// the `*_with_rng` generation functions. Document generation does not require
/// cryptographically secure randomness, but generated values must not be used
/// as secrets.
pub trait RandomSource {
    /// Returns the next 64 random bits.
    fn next_u64(&mut self) -> u64;
}

/// A small deterministic random source initialized from a `u64` seed.
///
/// Equal seeds produce equal streams. This generator is not cryptographically
/// secure. Seed zero is accepted.
#[derive(Debug, Clone)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    /// Creates a deterministic random source from `seed`.
    pub const fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { ZERO_SEED_FALLBACK } else { seed },
        }
    }
}

impl RandomSource for SeededRng {
    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
}

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

/// Uniformly samples `0..upper` using rejection sampling.
pub(crate) fn below<R: RandomSource + ?Sized>(rng: &mut R, upper: u64) -> u64 {
    assert!(upper > 0, "random upper bound must be positive");
    let threshold = upper.wrapping_neg() % upper;
    loop {
        let value = rng.next_u64();
        if value >= threshold {
            return value % upper;
        }
    }
}

pub(crate) fn below_u32<R: RandomSource + ?Sized>(rng: &mut R, upper: u32) -> u32 {
    u32::try_from(below(rng, u64::from(upper))).expect("bounded random value must fit in u32")
}

pub(crate) fn below_u8<R: RandomSource + ?Sized>(rng: &mut R, upper: u8) -> u8 {
    u8::try_from(below(rng, u64::from(upper))).expect("bounded random value must fit in u8")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_seed_does_not_lock_the_generator() {
        let mut rng = SeededRng::new(0);
        assert_ne!(rng.next_u64(), 0);
        assert_ne!(rng.next_u64(), 0);
    }

    #[test]
    fn bounded_values_stay_below_the_limit() {
        let mut rng = SeededRng::new(42);
        for _ in 0..100 {
            assert!(below(&mut rng, 10) < 10);
        }
    }
}
