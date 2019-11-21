## Benchmarking various PRNGs in Rust

Currenly checking:

* Xoshiro256StarStar
* Xoshiro128Plus
* Xorshift
* Xorshift (modified with non-dependent seeds)
* Lehmer64

```
Clock cycles per .rand() on average over 0x7fffffff iterations

Xoshiro256StarStar      19.15 | Xoshiro128Plus      19.08 | Xorshift      18.94 | Xorshift_mod      18.86 | Lehmer64      19.13 |
Xoshiro256StarStar      19.02 | Xoshiro128Plus      19.15 | Xorshift      18.85 | Xorshift_mod      19.07 | Lehmer64      19.28 |
Xoshiro256StarStar      19.25 | Xoshiro128Plus      19.27 | Xorshift      18.99 | Xorshift_mod      18.92 | Lehmer64      19.16 |
Xoshiro256StarStar      19.06 | Xoshiro128Plus      18.92 | Xorshift      18.81 | Xorshift_mod      19.10 | Lehmer64      18.84 |
Xoshiro256StarStar      19.14 | Xoshiro128Plus      18.99 | Xorshift      18.92 | Xorshift_mod      20.07 | Lehmer64      20.17 |
Xoshiro256StarStar      20.15 | Xoshiro128Plus      20.05 | Xorshift      19.51 | Xorshift_mod      18.88 | Lehmer64      19.46 |
Xoshiro256StarStar      19.12 | Xoshiro128Plus      19.02 | Xorshift      18.87 | Xorshift_mod      18.89 | Lehmer64      19.04 |
Xoshiro256StarStar      19.06 | Xoshiro128Plus      19.02 | Xorshift      19.06 | Xorshift_mod      18.89 | Lehmer64      18.96 |
Xoshiro256StarStar      19.05 | Xoshiro128Plus      19.12 | Xorshift      18.94 | Xorshift_mod      18.99 | Lehmer64      19.03 |
Xoshiro256StarStar      19.10 | Xoshiro128Plus      18.86 | Xorshift      19.11 | Xorshift_mod      18.83 | Lehmer64      19.33 |
Xoshiro256StarStar      18.97 | Xoshiro128Plus      19.14 | Xorshift      18.97 | Xorshift_mod      18.97 | Lehmer64      18.94 |
Xoshiro256StarStar      19.13 | Xoshiro128Plus      18.81 | Xorshift      19.00 | Xorshift_mod      18.84 | Lehmer64      18.90 |
Xoshiro256StarStar      19.02 | Xoshiro128Plus      19.08 | Xorshift      18.82 | Xorshift_mod      18.79 | Lehmer64      18.99 |
```

Above results were ran on:

```
Intel(R) Core(TM) i5-6440HQ CPU @ 2.60GHz
```
