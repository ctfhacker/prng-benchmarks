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
        self.value = unsafe { std::intrinsics::unchecked_mul(self.value, 0xda942042e4dd58b5) };
        // self.value = self.value.wrapping_mul(0xda942042e4dd58b5);
        (self.value >> 64) as usize
    }
}
