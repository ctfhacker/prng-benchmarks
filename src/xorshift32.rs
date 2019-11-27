/// https://codingforspeed.com/using-faster-psudo-random-generator-xorshift/
pub struct Xorshift32 {
    /// Interal xorshift32 seed
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}

impl Xorshift32 {
    /// Create a new, TSC-seeded random number generator
    pub fn new() -> Self {
        let ret = Xorshift32 {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123,
        };

        ret
    }

    /// Get a random 64-bit number using xorshift32
    pub fn rand(&mut self) -> u64 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8));
        self.w
    }
}
