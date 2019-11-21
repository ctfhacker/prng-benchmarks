use std::cell::Cell;

pub struct Lehmer64 {
    value: u128,
}

impl Lehmer64 {
    pub fn new() -> Lehmer64 {
        let res = Lehmer64 {
            value: unsafe { core::arch::x86_64::_rdtsc() } as u128,
        };

        res
    }

    pub fn rand(&mut self) -> usize {
        self.value = self.value.wrapping_mul(0xda942042e4dd58b5);
        (self.value >> 64) as usize
    }
}

pub fn do_lehmer64() {
    let mut rng = Lehmer64::new();
    for _ in 0..10 {
        rng.rand();
    }

    let mut times: Vec<u64> = Vec::new();
    for _ in 0..0xff_ffff {
        let start = unsafe { core::arch::x86_64::_rdtsc() };
        rng.rand();
        let end = unsafe { core::arch::x86_64::_rdtsc() };
        times.push(end - start);
    }

    print!(
        "Lehmer64 {:10.2} | ",
        (times.iter().sum::<u64>() as f64) / (times.len() as f64)
    );
}

/// Random number generator implementation using xorshift64
pub struct Xorshift {
    /// Interal xorshift seed
    seed: Cell<u64>,
}

impl Xorshift {
    /// Create a new, TSC-seeded random number generator
    pub fn new() -> Self {
        let ret = Xorshift {
            seed: Cell::new(unsafe { core::arch::x86_64::_rdtsc() }),
        };

        for _ in 0..1000 {
            let _ = ret.rand();
        }

        ret
    }

    /// Created a RNG with a fixed `seed` value
    pub fn seeded(seed: u64) -> Self {
        Xorshift {
            seed: Cell::new(seed),
        }
    }

    /// Get a random 64-bit number using xorshift
    pub fn rand(&self) -> usize {
        let mut seed = self.seed.get();
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 43;
        self.seed.set(seed);
        seed as usize
    }
}

pub fn do_xorshift() {
    let rng = Xorshift::new();
    for _ in 0..10 {
        rng.rand();
    }

    let mut times: Vec<u64> = Vec::new();
    for _ in 0..0xff_ffff {
        let start = unsafe { core::arch::x86_64::_rdtsc() };
        rng.rand();
        let end = unsafe { core::arch::x86_64::_rdtsc() };
        times.push(end - start);
    }

    print!(
        "Xorshift {:10.2} | ",
        (times.iter().sum::<u64>() as f64) / (times.len() as f64)
    );
}

#[inline]
fn rotl(x: u32, k: u32) -> u32 {
    (x << k) | (x >> (32 - k))
}

#[inline]
fn rotl64(x: u64, k: u64) -> u64 {
    (x << k) | (x >> (64 - k))
}

/// Implementation from http://prng.di.unimi.it/xoshiro128plusplus.c
struct Xoshiro128Plus {
    val0: u32,
    val1: u32,
    val2: u32,
    val3: u32,
}

impl Xoshiro128Plus {
    pub fn new() -> Xoshiro128Plus {
        Xoshiro128Plus {
            val0: 0,
            val1: 0,
            val2: 0,
            val3: 0,
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

pub fn do_xoshiro128plus() {
    let mut rng = Xoshiro128Plus::new();
    for _ in 0..100 {
        rng.rand();
    }

    let mut times: Vec<u64> = Vec::new();
    for _ in 0..0xff_ffff {
        let start = unsafe { core::arch::x86_64::_rdtsc() };
        rng.rand();
        let end = unsafe { core::arch::x86_64::_rdtsc() };
        times.push(end - start);
    }

    print!(
        "Xoshiro128Plus {:10.2} | ",
        (times.iter().sum::<u64>() as f64) / (times.len() as f64)
    );
}

/// Implementation from http://prng.di.unimi.it/xoshiro128plusplus.c
struct Xoshiro256StarStar {
    val0: u64,
    val1: u64,
    val2: u64,
    val3: u64,
}

impl Xoshiro256StarStar {
    pub fn new() -> Xoshiro256StarStar {
        Xoshiro256StarStar {
            val0: 0,
            val1: 0,
            val2: 0,
            val3: 0,
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

pub fn do_xoshiro256starstar() {
    let mut rng = Xoshiro256StarStar::new();
    for _ in 0..100 {
        rng.rand();
    }

    let mut times: Vec<u64> = Vec::new();
    for _ in 0..0xff_ffff {
        let start = unsafe { core::arch::x86_64::_rdtsc() };
        rng.rand();
        let end = unsafe { core::arch::x86_64::_rdtsc() };
        times.push(end - start);
    }

    print!(
        "Xoshiro256StarStar {:10.2} | ",
        (times.iter().sum::<u64>() as f64) / (times.len() as f64)
    );
}

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

pub fn do_xorshift_mod() {
    let mut rng = Xorshift_mod::new();
    for _ in 0..10 {
        rng.rand();
    }

    let mut times: Vec<u64> = Vec::new();
    for _ in 0..0xff_ffff {
        let start = unsafe { core::arch::x86_64::_rdtsc() };
        rng.rand();
        let end = unsafe { core::arch::x86_64::_rdtsc() };
        times.push(end - start);
    }

    print!(
        "Xorshift_mod {:10.2} | ",
        (times.iter().sum::<u64>() as f64) / (times.len() as f64)
    );
}

pub fn main() {
    print!("Clock cycles per .rand() on average over 0xff_fffff iterations\n");
    loop {
        do_xoshiro256starstar();
        do_xoshiro128plus();
        do_xorshift();
        do_xorshift_mod();
        do_lehmer64();
        print!("\n");
    }
}
