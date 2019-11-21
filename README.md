## Benchmarking various PRNGs in Rust

Currenly checking:

* Xoshiro256StarStar
* Xoshiro128Plus
* Xorshift
* Xorshift (modified with non-dependent seeds)
* Lehmer64

```
Clock cycles per .rand() on average over 0x7fffffff iterations

X256**      21.20 | X128+      20.92 | Xshft      20.93 | xshft_mod      21.01 | Leh64      20.86 |
X256**      20.93 | X128+      21.05 | Xshft      20.79 | xshft_mod      20.94 | Leh64      20.80 |
X256**      20.82 | X128+      20.74 | Xshft      21.03 | xshft_mod      20.78 | Leh64      20.88 |
X256**      21.06 | X128+      20.92 | Xshft      20.98 | xshft_mod      20.87 | Leh64      20.88 |
X256**      21.14 | X128+      20.78 | Xshft      20.84 | xshft_mod      20.78 | Leh64      20.86 |
X256**      21.09 | X128+      20.79 | Xshft      20.90 | xshft_mod      20.94 | Leh64      20.89 |
X256**      21.13 | X128+      20.82 | Xshft      20.91 | xshft_mod      20.78 | Leh64      20.88 |
X256**      21.04 | X128+      20.78 | Xshft      20.89 | xshft_mod      21.01 | Leh64      20.87 |
X256**      21.01 | X128+      20.97 | Xshft      20.75 | xshft_mod      20.93 | Leh64      20.84 |
X256**      21.08 | X128+      20.90 | Xshft      20.75 | xshft_mod      20.74 | Leh64      21.19 |
X256**      21.07 | X128+      21.24 | Xshft      21.21 | xshft_mod      20.88 | Leh64      20.81 |
X256**      21.35 | X128+      20.90 | Xshft      20.87 | xshft_mod      21.00 | Leh64      20.77 |
X256**      21.08 | X128+      20.83 | Xshft      20.83 | xshft_mod      20.80 | Leh64      20.96 |
X256**      20.96 | X128+      20.81 | Xshft      21.01 | xshft_mod      20.81 | Leh64      20.97 |
X256**      21.35 | X128+      21.13 | Xshft      20.83 | xshft_mod      20.81 | Leh64      20.80 |
```

Above results were ran on:

```
Intel(R) Core(TM) i5-6440HQ CPU @ 2.60GHz
```
