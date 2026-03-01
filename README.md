# String compare

```sh
cargo +nightly test
```

```sh
cargo +nightly bench
```

```txt
running 6 tests
test medium_compare_asm     ... bench:           5.02 ns/iter (+/- 0.23)
test medium_compare_at_home ... bench:          35.24 ns/iter (+/- 0.24)
test medium_compare_rust    ... bench:           4.01 ns/iter (+/- 0.12)
test small_compare_asm      ... bench:           2.46 ns/iter (+/- 0.02)
test small_compare_at_home  ... bench:          10.79 ns/iter (+/- 0.07)
test small_compare_rust     ... bench:           3.41 ns/iter (+/- 0.28)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out; finished in 10.17s
```

NOW WITH SIMD!

```sh
just bench
```

```txt
running 8 tests
test medium_compare_asm     ... bench:           4.56 ns/iter (+/- 0.02)
test medium_compare_at_home ... bench:          34.00 ns/iter (+/- 0.26)
test medium_compare_rust    ... bench:           4.02 ns/iter (+/- 0.07)
test medium_compare_simd    ... bench:           3.22 ns/iter (+/- 0.25)
test small_compare_asm      ... bench:           2.65 ns/iter (+/- 0.04)
test small_compare_at_home  ... bench:          10.92 ns/iter (+/- 0.08)
test small_compare_rust     ... bench:           2.85 ns/iter (+/- 0.02)
test small_compare_simd     ... bench:           2.28 ns/iter (+/- 0.02)

test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out; finished in 2.94s
```

```sh
just emit_asm
```

in the .s file you can search for compare_simd, and it should show the usage of the zmm0 register (if your cpu supports it).
