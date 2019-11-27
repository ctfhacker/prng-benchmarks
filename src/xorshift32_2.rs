pub struct Xorshift32_2 {
    val0: u32,
}

impl Xorshift32_2 {
    /// Create a new, TSC-seeded random number generator
    pub fn new(seed: u32) -> Self {
        Xorshift32_2 { val0: seed }
    }

    /*
    /// Created a RNG with a fixed `seed` value
    pub fn seeded(seed: u64) -> Self {
        Xorshift {
            seed: Cell::new(seed),
        }
    }
    */

    pub fn rand(&mut self) -> u32 {
        self.val0 ^= (self.val0 << 13);
        self.val0 = (self.val0 >> 17);
        self.val0 ^= (self.val0 << 5);
        return self.val0;
    }
}
