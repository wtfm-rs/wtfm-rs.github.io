#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! # Introduction
//!
//! WTFM is <a href="https://en.wikipedia.org/wiki/RTFM">RTFM</a> in Rust.
//!
//! To master an ecosystem as broad and deep as Rust in a short period of time,
//! we might want to turn R into W, dive deep by writing our own
//! [doctests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) iteratively until we find the solution to our problem.
//!
//! ```
//! assert_eq!("WTFM", "RTFM".replace("R", "W"));
//! ```
//! Doctests can be good "paper trails" of our thought process.
//!
//! We can also learn about the throught process of other crates by "reviewing"
//! them with our own doctests.
//!
//! # Professional rustdoc rabbit holes
//! Be prepared to fall into rabbit holes of rustdoc and get out
//! where and when you need to. This is a skill to be learned.
//! We are programmers, not librarians.
//!
//! <https://rust-lang.github.io/api-guidelines/documentation.html>
//!
//! # Doctest
//! If `cargo test --doc` passed, we will have have output like
//! ```text
//! all doctests ran in 0.70s; merged doctests compilation took 0.33s
//! ```
//! There is no need to check each one of them in playground but we can explore
//! further by tinkering with them in the plalyground.
//! (That is why we call it playground.)
//!
//! Since doctests are merged and compiled into one binary, we want to make sure
//! they don't interfere with each other. Since doctests are not the real part
//! of the code, we can have cleanup code in the libray body or unit test body.
//! We can see how the source code progress this way.
//!
//! # [assert!] driven development
//! Be prepared to [RTFM](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) [std].
//!
//! Let's start with a trivial rust program using
//! macro [assert!] and primitive [true].
//! ```
//! assert!(true);
//! ```
//!
//! We can wrap it with a function `assert_true_v1` and call it in the
//! same doctest block:
//! ```
//! fn assert_true_v1() {
//!     assert!(true);
//! }
//! assert_true_v1();
//! ```
//! To make it a little be more complicated we can wrap [true] with
//! a function as well.
//! ```
//! fn return_true() -> bool { true }
//! assert!(return_true());
//! ```
//! The version 2 of `asser_true` can be
//!
//! ```compile_fail
//! fn assert_true_v2() {
//!     assert!(return_true());
//! }
//! ```
//! The reason the doctest above failed is `return_true` is out of scope.
//!
//! Let's put function [return_true] in the crate,
//! following works in `cargo test --doc` but
//! would fail in playground because playground can't resolve our crate.
//! ```
//! use wtfm_rs::return_true;
//! fn assert_true_v2() {
//!     assert!(return_true());
//! }
//! ```
//! We can add similar test to the unit test of this crate (see the source code)
//! `cargo test` will capture all of them.
//! ```text
//!  cargo test
//!    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
//!     Running unittests src/lib.rs (target/debug/deps/wtfm_rs-ee0d1b96bcb289bd)
//!
//! running 1 test
//! test assert_true_v3 ... ok
//!
//! test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//!
//!   Doc-tests wtfm_rs
//!
//! running 4 tests
//! test src/lib.rs - (line 51) ... ok
//! test src/lib.rs - (line 73) ... ok
//! test src/lib.rs - (line 58) ... ok
//! test src/lib.rs - (line 11) ... ok
//!
//! test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//!
//!
//! running 1 test
//! test src/lib.rs - (line 64) - compile fail ... ok
//!
//! test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
//!
//! all doctests ran in 0.45s; merged doctests compilation took 0.21s
//! ```
//!
//! # [println!] driven development
//! If we add [println!] to the doctest
//! ```
//! use wtfm_rs::return_true;
//! fn assert_true_v3() {
//!     assert!(return_true());
//!     println!("{}", return_true());
//! }
//! ```
//! `cargo test` will not have any println feedback.
//! ```text
//! test src/lib.rs - (line 110) ... ok
//! ```
//!
//! So `println!` driven development wouldn't work with doctests.
//!
//! To do `println!` driven development, we can use example crates.
//! ```text
//! cargo doc --examples
//! ```
//! ```text
//! cargo run --example example-assert-true
//!    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
//!     Running `target/debug/examples/example-assert-true`
//! true
//! ```

//! # External crates
//! Rustdoc is organized by crate and we can also bring external crates in
//! via `Cargo.toml`
//!
//! ```toml
//! [package]
//! name = "wtfm-rs"
//! version = "0.1.0"
//! edition = "2024"
//!
//! [dependencies]
//! wtfm-rs-hello-world = { git = "https://github.com/wtfm-rs/wtfm-rs.github.io", branch = "hello-world", version = "0.1.0" }
//! ```
//!
//! In this case, we pull [wtfm_rs_hello_world] crate from
//! <https://github.com/wtfm-rs/wtfm-rs.github.io/tree/hello-world>.
//!
//! We can use the function `hello_world()` from it
//! ```text
//! pub use wtfm_rs_hello_world::hello_world;
//! ```
//! and use it as `wtfm_rs::hello_world`.
//!
//! ```
//! use wtfm_rs::hello_world;
//! assert_eq!(hello_world(), "Hello, world!");
//! ```
//! This will fail in playwright as before, but will pass `cargo test`.
//!
//! # Get the ouput from echo Hello, world!
//! ```
//! use std::process::Command;
//! let output = Command::new("echo")
//!        .arg("Hello,")
//!        .arg("world!")
//!        .output()
//!        .expect("Failed to execute command");
//! let output_1 = String::from_utf8_lossy(&output.stdout).to_string();
//! assert_eq!(format!("{}", output_1), "Hello, world!\n");
//! let output_2 = String::from_utf8(output.stdout).expect("Format error");
//! assert_eq!(format!("{}", output_2), "Hello, world!\n");
//! ```
//! [std::process::Command]
//!
//! [String::from_utf8]
//!
//! [String::from_utf8_lossy]
//!
//! # Same code in three crates
//!
//! 1. We started it as doctest in this crate [`wtfm_rs`](./index.html).
//! 2. We copy and paste it to an example crate
//!    [`example_echo_hello_world`](../example_echo_hello_world/index.html).
//!
//! 3. We create an external crate [wtfm_rs_echo_hello_world]
//!    in a [branch](https://github.com/wtfm-rs/wtfm-rs.github.io/tree/refs/heads/echo-hello-world) and pull it in with `Cargo.toml`.:
//! ```text
//! wtfm-rs-echo-hello-world = { git = "https://github.com/wtfm-rs/wtfm-rs.github.io", branch = "echo-hello-world", version = "0.1.0" }
//! ```
//! # Why do we do this?
//! This allows us to explore with controlled dependenices.
//! Doctests in a crate will not becoming the dependency of any crate so we are
//! safe to modify and test out ideas.
//! Same with example crates that allows us to add `fn main` but can only be
//! called via `cargto run --example ...`.
//!
//! Once the code is "published" in [wtfm_rs_echo_hello_world],
//! and became part of the dependency tree, it can be called by us
//! (Rollowing code will not run in Playground becuase of scope.)
//!
//! ```
//! use wtfm_rs_echo_hello_world::echo_hello_world;
//! assert_eq!(echo_hello_world(), "Hello, world!\n");
//! ```
//!
//! ```
//! use wtfm_rs::echo_hello_world;
//! assert_eq!(echo_hello_world(), "Hello, world!\n");
//! ```
//!
//! ```text
//! cargo tree
//! wtfm-rs v0.1.0 (/Users/sam/github/wtfm-rs/wtfm-rs.github.io)
//! ├── wtfm-rs-echo-hello-world v0.1.0 (https://github.com/wtfm-rs/wtfm-rs.github.io?branch=echo-hello-world#9244775f)
//! └── wtfm-rs-hello-world v0.1.0 (https://github.com/wtfm-rs/wtfm-rs.github.io?branch=hello-world#98fb5c7e)
//! ```
//! By design, none of these WTFM crates should end up in production.
//! They are "paper trails” of thought process.
//! The purpose of reading, writing, and testing them is to force us to
//! think and reason the choices we made in the context of rust toolchain,
//! i.e., `cargo doc, cargo test, cargo run ...` .
//!
//! We are free to change our mind.
//!
//! # Devil is in the details
//! We have been distracted by the toolchain rabbit holes and it is time
//! to get back to our main trail. Let's take a look at following code again.
//! ```
//! use std::process::Command;
//! let output = Command::new("echo")
//!        .arg("Hello,")
//!        .arg("world!")
//!        .output()
//!        .expect("Failed to execute command");
//! let output_1 = String::from_utf8_lossy(&output.stdout).to_string();
//! assert_eq!(format!("{}", output_1), "Hello, world!\n");
//! let output_2 = String::from_utf8(output.stdout).expect("Format error");
//! assert_eq!(format!("{}", output_2), "Hello, world!\n");
//! ```
//! If we switch `output_1` and `output_2`, it won't compile
//! ```compile_fail
//! use std::process::Command;
//! let output = Command::new("echo")
//!        .arg("Hello,")
//!        .arg("world!")
//!        .output()
//!        .expect("Failed to execute command");
//! let output_2 = String::from_utf8(output.stdout).expect("Format error");
//! assert_eq!(format!("{}", output_2), "Hello, world!\n");
//! let output_1 = String::from_utf8_lossy(&output.stdout).to_string();
//! assert_eq!(format!("{}", output_1), "Hello, world!\n");
//! ```
//! ```text
//!  failures:
//!
//!  ---- src/lib.rs - (line 247) stdout ----
//!  error[E0382]: borrow of moved value: `output.stdout`
//!     --> src/lib.rs:256:40
//!      |
//!    9 | let output_2 = String::from_utf8(output.stdout).expect("Format error");
//!      |                                  ------------- value moved here
//!   10 | assert_eq!(format!("{}", output_2), "Hello, world!\n");
//!   11 | let output_1 = String::from_utf8_lossy(&output.stdout).to_string();
//!      |                                        ^^^^^^^^^^^^^^ value borrowed here after move
//!      |
//!      = note: move occurs because `output.stdout` has type `Vec<u8>`, which does not implement the `Copy` trait
//!      = note: borrow occurs due to deref coercion to `[u8]`
//!  note: deref defined here
//!     --> /Users/sam/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3584:5
//!      |
//! 3584 |     type Target = [T];
//!      |     ^^^^^^^^^^^
//!
//!  error: aborting due to 1 previous error
//!
//!  For more information about this error, try `rustc --explain E0382`.
//!  Couldn't compile the test.
//!
//!  failures:
//!     src/lib.rs - (line 247)
//! ```
//! The error message is a great souce for RTFM so we won't
//! get into another rabbit hole here.
//! ( [String::from_utf8] vs  [String::from_utf8_lossy])

pub fn return_true() -> bool {
    true
}

pub use wtfm_rs_echo_hello_world::echo_hello_world;
pub use wtfm_rs_hello_world::hello_world;

#[cfg(test)]
#[test]
fn test_return_true() {
    assert!(return_true());
}

#[test]
fn test_hello_world() {
    assert_eq!(hello_world(), "Hello, world!");
}

#[test]
fn test_echo_hello_world() {
    assert_eq!(echo_hello_world(), "Hello, world!\n");
}
