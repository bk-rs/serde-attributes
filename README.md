## Dev

```
cargo clippy --all-features --tests -- -D clippy::all
cargo +nightly clippy --all-features --tests -- -D clippy::all

cargo fmt -- --check

cargo build-all-features
cargo test-all-features -- --nocapture
```

## Doc

[Extract from](https://github.com/serde-rs/serde/blob/v1.0.127/serde_derive/src/internals/attr.rs#L290)
