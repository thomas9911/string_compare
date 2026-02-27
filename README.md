# String compare

```
cargo test
```

```
cargo +nightly bench
```

```
running 6 tests
test medium_compare_asm     ... bench:           5.02 ns/iter (+/- 0.23)
test medium_compare_at_home ... bench:          35.24 ns/iter (+/- 0.24)
test medium_compare_rust    ... bench:           4.01 ns/iter (+/- 0.12)
test small_compare_asm      ... bench:           2.46 ns/iter (+/- 0.02)
test small_compare_at_home  ... bench:          10.79 ns/iter (+/- 0.07)
test small_compare_rust     ... bench:           3.41 ns/iter (+/- 0.28)

test result: ok. 0 passed; 0 failed; 0 ignored; 6 measured; 0 filtered out; finished in 10.17s
```
