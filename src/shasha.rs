/// https://gist.github.com/cFigge/70a3d7360d09df1725a9e1e0036aa730
#[inline]
pub fn rotl(x: u64, k: u64) -> u64 {
    (x << k) | (x >> (64 - k))
}

pub struct ShaSha {
    val0: u64,
    val1: u64,
    val2: u64,
    val3: u64,
}

impl ShaSha {
    pub fn new() -> ShaSha {
        let seed = 0xdeadbeef;

        let mut seed = seed + 0x9e3779b97f4a7c15;
        seed = (seed ^ (seed >> 30)) * 0xbf58476d1ce4e5b9;
        seed = (seed ^ (seed >> 27)) * 0x94d049bb133111eb;
        let val0 = seed ^ (seed >> 31);

        seed = seed + 0x9e3779b97f4a7c15;
        seed = (seed ^ (seed >> 30)) * 0xbf58476d1ce4e5b9;
        seed = (seed ^ (seed >> 27)) * 0x94d049bb133111eb;
        let val1 = seed ^ (seed >> 31);

        seed = seed + 0x9e3779b97f4a7c15;
        seed = (seed ^ (seed >> 30)) * 0xbf58476d1ce4e5b9;
        seed = (seed ^ (seed >> 27)) * 0x94d049bb133111eb;
        let val2 = seed ^ (seed >> 31);

        seed = seed + 0x9e3779b97f4a7c15;
        seed = (seed ^ (seed >> 30)) * 0xbf58476d1ce4e5b9;
        seed = (seed ^ (seed >> 27)) * 0x94d049bb133111eb;
        let val3 = seed ^ (seed >> 31);

        ShaSha {
            val0,
            val1,
            val2,
            val3,
        }
    }

    pub fn rand(&mut self) -> u64 {
        let rtn = rotl(self.val1, (self.val0 & 63)) + self.val3;

        let s1_xor = self.val2 ^ (self.val1 >> 7);
        self.val0 += self.val1;
        self.val3 ^= self.val0;
        self.val2 += self.val3;
        self.val1 ^= s1_xor;
        rtn
    }
}
