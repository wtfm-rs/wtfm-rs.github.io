# wtfm-rs

```rust
assert_eq!("WTFM", "RTFM".replace("R", "W"));
```

[Cargo.toml](https://github.com/wtfm-rs/wtfm-rs.github.io/Cargo.toml)

```toml
[workspace]
resolver = "3"
members = ["introduction"]
```

## Introduction

[introdcution/Cargo.toml](https://github.com/wtfm-rs/wtfm-rs.github.io/introduction/Cargo.toml)

```toml
[package]
name = "introduction"
version = "0.1.0"
edition = "2024"

[dependencies]
wtfm-vec = { git = "https://github.com/wtfm-rs/wtfm-vec", version = "0.1.0" }
```
<https://wtfm-rs.github.io/doc/introduction>

## wtfm-vec

[Cargo.toml](https://github.com/wtfm-rs/wtfm-vec)

```toml
[package]
name = "wtfm-vec"
version = "0.1.0"
edition = "2024"

[dependencies]
```

<https://wtfm-rs.github.io/wtfm-vec/doc/wtfm_vec>

linked with

<https://wtfm-rs.github.io/doc/introduction>

