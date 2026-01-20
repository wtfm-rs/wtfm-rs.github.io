# wtfm-rs

```rust
assert_eq!("WTFM", "RTFM".replace("R", "W"));
```

[Cargo.toml](https://github.com/wtfm-rs/wtfm-rs.github.io/blob/main/Cargo.toml)

```toml
[workspace]
resolver = "3"
members = ["introduction"]
```

## Introduction

[introdcution/Cargo.toml](https://github.com/wtfm-rs/wtfm-rs.github.io/blob/main/introduction/Cargo.toml)

```toml
[package]
name = "introduction"
version = "0.1.0"
edition = "2024"

[dependencies]
wtfm-vec = { git = "https://github.com/wtfm-rs/wtfm-vec", version = "0.1.0" }
```
<https://wtfm-rs.github.io/doc/introduction>

<https://wtfm-rs.github.io/doc/wtfm_vec>

## wtfm-vec

[Cargo.toml](https://github.com/wtfm-rs/blob/main/Cargo.toml)

```toml
[package]
name = "wtfm-vec"
version = "0.1.0"
edition = "2024"

[dependencies]
```

<https://wtfm-rs.github.io/wtfm-vec/doc/wtfm_vec>

This is a [standalone crate](https://github.com/wtfm-rs/wtfm-vec/tree/main) instead of the one 
as the dependency of [introduction](https://github.com/wtfm-rs/wtfm-rs.github.io/tree/main/introduction).

