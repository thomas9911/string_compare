emit_asm:
    cargo +nightly rustc --release -- --emit asm -C target-cpu=native
    echo "check target/release/deps folder for .s file"

bench:
    RUSTFLAGS="-C target-cpu=native" cargo +nightly bench
    # cargo +nightly bench

test:
    cargo +nightly test