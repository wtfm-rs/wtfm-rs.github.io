#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! # Introduction
//!
//! WTFM is a parody of <a href="https://en.wikipedia.org/wiki/RTFM">RTFM</a>,
//! implemented in Rust.
//!
//! To learn something as broad and deep as Rust ecosystem,
//! we might want to turn R into W, dive deep by writing our own
//! [doctests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) iteratively until we find the solution to our problem.
//!
//! ```
//! assert_eq!("WTFM", "RTFM".replace("R", "W"));
//! ```
//! Doctests can be a good "paper trail" of our thought process.
//!
//! We can also learn about the throught process of other crates by "reviewing"
//! them with our own doctests.
//!
//! ## Rabbit holes of rustdoc
//! Be prepared to fall into rabbit holes of rustdoc and get out
//! where and when you need to. This is a skill to be learned.
//! We are programmers, not librians.
//!
//! <https://rust-lang.github.io/api-guidelines/documentation.html>
//!
//! <https://github.com/sophiajt/rfcs/blob/front_page_styleguide/text/0000-api-doc-frontpage-styleguide.md>
//!
//! <https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md>
//!
//! <https://github.com/rust-lang/rfcs/blob/master/text/0505-api-comment-conventions.md>
//!
//! ## Doctest
//! If `cargo test --doc` passed, we will have have output like
//! ```text
//! all doctests ran in 0.70s; merged doctests compilation took 0.33s
//! ```
//! There is no need to check each one of them in playground but we can explore
//! further by playwith them in the plalyground (that is why we call it playground.)
//!
//! Since doctests are merged and compiled into one binary, we want to make sure
//! they don't interfere with each other. Since doctests are not the real part
//! of the code, we can have cleanup code in the libray body or unit test body.
//! We can see how the source code progress this way.
//!
//! ## assert! driven development
//! Be prepared to [RTFM](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) [std].
//!
//! Suppose we have a simple `assert_true` function that always passes:
//! ```
//! fn assert_true_v1() {
//!     assert!(true);
//! }
//! assert_true_v1();
//! ```
//! Let's make a one line function
//! ```
//! fn is_true() -> bool { true }
//! assert!(is_true());
//! ```
//! Now we can have version 2
//!
//! ```compile_fail
//! fn assert_true_v2() {
//!     assert!(is_true());
//! }
//! ```
//! The reason the doctest above failed is `is_true` is out of the scope.
//!
//! Let's a function [is_true], the following works in `cargo test --doc` but
//! would fail in playground because playground can't resolve our crate.
//! ```
//! use wtfm_rs::is_true;
//! fn assert_true_v2() {
//!     assert!(is_true());
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
//! ## println! driven development
//! If we add `println!` to the doctest
//! ```
//! use wtfm_rs::is_true;
//! fn assert_true_v3() {
//!     assert!(is_true());
//!     println!("{}", is_true());
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

//! ## External crates
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
//! (This will fail in playwright as before, but will pass `cargo test`
//!
pub fn is_true() -> bool {
    true
}

pub use wtfm_rs_hello_world::hello_world;

#[cfg(test)]
#[test]
fn test_is_true() {
    assert!(is_true());
}

#[test]
fn test_hello_world() {
    assert_eq!(hello_world(), "Hello, world!");
}
