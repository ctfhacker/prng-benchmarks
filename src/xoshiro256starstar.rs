fn rotl64(x: u64, k: u64) -> u64 {
    (x << k) | (x >> (64 - k))
}

/// Implementation from http://prng.di.unimi.it/xoshiro128plusplus.c
pub struct Xoshiro256StarStar {
    val0: u64,
    val1: u64,
    val2: u64,
    val3: u64,
}

impl Xoshiro256StarStar {
    pub fn new() -> Xoshiro256StarStar {
        Xoshiro256StarStar {
            val0: 0xcafebabe,
            val1: 0xcafebabe,
            val2: 0xcafebabe,
            val3: 0xcafebabe,
        }
    }

    pub fn rand(&mut self) -> u64 {
        let res = rotl64(self.val1.wrapping_mul(5), 7).wrapping_mul(9);
        let t = self.val1 << 17;
        self.val2 ^= self.val0;
        self.val3 ^= self.val1;
        self.val1 ^= self.val2;
        self.val0 ^= self.val3;
        self.val2 ^= t;
        self.val3 = rotl64(self.val3, 45);
        res
    }
}
