//! # Introduction
//!
//! To an experienced programmer who can [RTFM](https://en.wikipedia.org/wiki/RTFM),
//! the best entry point of Rust documentation is
//! <https://doc.rust-lang.org/stable>.
//!
//! To learn something as broad and deep as Rust ecosystem, we might also want to turn R into W,
//! dive deep by writing our own
//! [doc-tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html),
//! [unit and integration tests](https://doc.rust-lang.org/book/ch11-03-test-organization.html).
//!
//!
//! ```
//! assert_eq!("WTFM", "RTFM".replace("R", "W"));
//! ```
//!
//! # Tests as documentation
//!
//! Since the main source of the M in RTFM is from
//! <https://doc.rust-lang.org/stable/rustdoc/>
//! we are going to use write test and docs only crates
//! and run following commands:
//!
//! ```sh
//! rustdoc introduction.rs
//! rustdoc --test introduction.rs
//! rustc --test introduction.rs && ./introduction
//! ```
//!
//! ```sh
//! % rustdoc --test introduction.rs
//!
//! running 1 test
//! test introduction.rs - (line 13) ... ok
//!
//! test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
//! ```
//! ```sh
//! % rustc --test introduction.rs  && ./introduction
//!
//! running 2 tests
//! test assert_true ... ok
//! test assert_wtfm ... ok
//!
//! test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//! ```
//! You can also confirm the output of these tests in GitHub Actions
//! <https://github.com/wtfm-rs/wtfm-rs.github.io/actions?query=branch%3Amain>
//!

#[cfg(test)]
#[test]
fn assert_true() {
    assert!(true);
}

#[test]
fn assert_wtfm() {
    assert_eq!("WTFM", "RTFM".replace("R", "W"));
}

mod macro_expand;
