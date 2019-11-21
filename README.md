## Benchmarking various PRNGs in Rust

Currenly checking:

* Xoshiro256StarStar
* Xoshiro128Plus
* Xorshift
* Lehmer64

```
Clock cycles per .rand() on average over 0x7fffffff iterations

Xoshiro256StarStar      20.85 | Xoshiro128Plus      20.82 | Xorshift      20.64 | Lehmer64      20.72 |
Xoshiro256StarStar      21.13 | Xoshiro128Plus      20.81 | Xorshift      20.95 | Lehmer64      20.98 |
Xoshiro256StarStar      21.01 | Xoshiro128Plus      21.10 | Xorshift      21.29 | Lehmer64      21.14 |
Xoshiro256StarStar      21.02 | Xoshiro128Plus      20.86 | Xorshift      20.95 | Lehmer64      20.80 |
Xoshiro256StarStar      20.86 | Xoshiro128Plus      20.82 | Xorshift      21.12 | Lehmer64      20.87 |
Xoshiro256StarStar      21.03 | Xoshiro128Plus      21.07 | Xorshift      20.80 | Lehmer64      20.90 |
Xoshiro256StarStar      21.22 | Xoshiro128Plus      21.11 | Xorshift      21.16 | Lehmer64      20.86 |
Xoshiro256StarStar      21.17 | Xoshiro128Plus      21.07 | Xorshift      20.85 | Lehmer64      20.96 |
Xoshiro256StarStar      23.49 | Xoshiro128Plus      27.35 | Xorshift      27.99 | Lehmer64      27.41 |
Xoshiro256StarStar      28.22 | Xoshiro128Plus      29.26 | Xorshift      31.60 | Lehmer64      27.84 |
Xoshiro256StarStar      27.72 | Xoshiro128Plus      24.41 | Xorshift      23.36 | Lehmer64      23.39 |
Xoshiro256StarStar      23.88 | Xoshiro128Plus      23.34 | Xorshift      23.55 | Lehmer64      23.24 |
Xoshiro256StarStar      25.24 | Xoshiro128Plus      26.35 | Xorshift      24.95 | Lehmer64      23.18 |
Xoshiro256StarStar      22.45 | Xoshiro128Plus      22.31 | Xorshift      22.14 | Lehmer64      22.25 |
Xoshiro256StarStar      22.45 | Xoshiro128Plus      22.14 | Xorshift      22.14 | Lehmer64      22.17 |
Xoshiro256StarStar      22.58 | Xoshiro128Plus      22.21 | Xorshift      22.09 | Lehmer64      22.05 |
Xoshiro256StarStar      21.41 | Xoshiro128Plus      21.18 | Xorshift      21.41 | Lehmer64      21.30 |
Xoshiro256StarStar      21.31 | Xoshiro128Plus      21.33 | Xorshift      21.20 | Lehmer64      21.13 |
Xoshiro256StarStar      21.41 | Xoshiro128Plus      21.22 | Xorshift      21.23 | Lehmer64      21.09 |
Xoshiro256StarStar      20.88 | Xoshiro128Plus      20.75 | Xorshift      20.86 | Lehmer64      20.75 |
```

Above results were ran on:

```
Intel(R) Core(TM) i5-6440HQ CPU @ 2.60GHz
```
