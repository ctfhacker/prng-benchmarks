#![feature(core_intrinsics)]

mod lehmer64;
pub use lehmer64::*;

mod xorshift;
pub use xorshift::*;

mod xorshift_nocell;
pub use xorshift_nocell::*;

mod xorshift32_2;
pub use xorshift32_2::*;

mod xoshiro128plus;
pub use xoshiro128plus::*;

mod xoshiro256starstar;
pub use xoshiro256starstar::*;

mod xorshift32;
pub use xorshift32::*;

macro_rules! benchmark {
    ($name:ident) => {
        let mut rng = $name::new();
        let counter: u64 = 0xff_ffff;
        let mut times = 0;
        let mut result = 0;

        for _ in 0..counter {
            let start = unsafe { core::arch::x86_64::_rdtsc() };
            result |= rng.rand();
            let end = unsafe { core::arch::x86_64::_rdtsc() };
            times += (end - start);
        }

        print!(
            "{:20} {:10.2} | {:#x}\n", stringify!($name), times as f64 / counter as f64, 
            result
        );
    }
}

pub fn main() {
    print!("Clock cycles per .rand() on average over 0xff_fffff iterations\n");
    benchmark!(Xorshift);
    benchmark!(Xorshift_nocell);
    benchmark!(Xorshift32);
    benchmark!(Xorshift32_2);
    benchmark!(Xoshiro256StarStar);
    benchmark!(Xoshiro128Plus);
    benchmark!(Lehmer64);
}
