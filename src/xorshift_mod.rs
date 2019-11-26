/// Random number generator implementation using xorshift_mod64
pub struct Xorshift_mod {
    /// Interal xorshift_mod seed
    val0: u64,
    val1: u64,
    val2: u64,
    val3: u64,
}

impl Xorshift_mod {
    /// Create a new, TSC-seeded random number generator
    pub fn new() -> Self {
        let mut ret = Xorshift_mod {
            val0: 0xabcd,
            val1: 0,
            val2: 0,
            val3: 0,
        };

        for i in 0..100 {
            let _ = ret.rand();
        }

        ret
    }

    /// Get a random 64-bit number using xorshift_mod
    pub fn rand(&mut self) -> u64 {
        self.val1 = self.val0 << 13;
        self.val2 = self.val1 >> 17;
        self.val3 = self.val2 << 43;
        self.val3
    }
}

