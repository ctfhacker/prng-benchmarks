use std::cell::Cell;

/// Random number generator implementation using xorshift64
pub struct Xorshift_nocell {
    /// Interal xorshift seed
    seed: u64,
}

impl Xorshift_nocell {
    /// Create a new, TSC-seeded random number generator
    pub fn new() -> Self {
        let mut ret = Xorshift_nocell {
            seed: unsafe { core::arch::x86_64::_rdtsc() },
        };

        for _ in 0..1000 {
            let _ = ret.rand();
        }

        ret
    }

    /// Created a RNG with a fixed `seed` value
    pub fn seeded(seed: u64) -> Self {
        Xorshift_nocell {
            seed: seed,
        }
    }

    /// Get a random 64-bit number using xorshift
    pub fn rand(&mut self) -> usize {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 17;
        self.seed ^= self.seed << 43;
        self.seed as usize
    }
}
