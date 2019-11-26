#[inline]
pub fn rotl(x: u32, k: u32) -> u32 {
    (x << k) | (x >> (32 - k))
}

/// Implementation from http://prng.di.unimi.it/xoshiro128plusplus.c
pub struct Xoshiro128Plus {
    val0: u32,
    val1: u32,
    val2: u32,
    val3: u32,
}

impl Xoshiro128Plus {
    pub fn new() -> Xoshiro128Plus {
        Xoshiro128Plus {
            val0: 0xdeadbeef,
            val1: 0xdeadbeef,
            val2: 0xdeadbeef,
            val3: 0xdeadbeef,
        }
    }

    pub fn rand(&mut self) -> u32 {
        let res = rotl(self.val0.wrapping_add(self.val3), 7) + self.val0;
        let t = self.val1 << 9;
        self.val2 ^= self.val0;
        self.val3 ^= self.val1;
        self.val1 ^= self.val2;
        self.val0 ^= self.val3;
        self.val2 ^= t;
        self.val3 = rotl(self.val3, 11);
        res
    }
}
