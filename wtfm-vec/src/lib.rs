#![doc(html_playground_url = "https://play.rust-lang.org/")]

//! Explore `std::vec::Vec` ([RTFM](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html))
//!
//! # [vec_push_pop](fn.vec_push_pop.html)
//! # [vec_extend](fn.vec_extend.html)
//!
//! These functions can also be tested by other crates in integration tests:
//! ```sh
//! cargo new --lib --name test-wtfm-vec /tmp/test-wtfm-vec
//! cd /tmp/test-wtfm-vec && cargo add --git https://github.com/wtfm-rs/wtfm-rs.github.io wtfm-vec && cd -
//! mkdir /tmp/test-wtfm-vec/tests
//! cat > /tmp/test-wtfm-vec/tests/it-works <<EOF
//! #[test]
//! fn it_works() {
//!    wtfm-vec::vec_extend();
//!    wtfm-vec::vec_push_pop();
//! }
//! EOF
//! cd /tmp/test-wtfm-vec && cargo test --tests && cd -
//! rm -rf /tmp/test-wtfm-vec
//! ```
//! Local test on my mac (`~/github/wtfm-rs/wtfm-rs.github.io`)
//!
//! ```sh
//!    Creating library `test-wtfm-vec` package
//! note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
//!    Updating git repository `https://github.com/wtfm-rs/wtfm-rs.github.io`
//!      Adding wtfm-vec (git) to dependencies
//!    Updating git repository `https://github.com/wtfm-rs/wtfm-rs.github.io`
//!     Locking 1 package to latest Rust 1.92.0 compatible version
//! ~/github/wtfm-rs/wtfm-rs.github.io
//!    Compiling wtfm-vec v0.1.0 (https://github.com/wtfm-rs/wtfm-rs.github.io#f3f80f0d)
//!    Compiling test-wtfm-vec v0.1.0 (/private/tmp/test-wtfm-vec)
//!     Finished `test` profile [unoptimized + debuginfo] target(s) in 0.27s
//!      Running unittests src/lib.rs (target/debug/deps/test_wtfm_vec-16e22ab8a53d193d)
//!
//! running 1 test
//! test tests::it_works ... ok
//!
//! test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//!
//! ~/github/wtfm-rs/wtfm-rs.github.io
//! ```
//!
//! GitHub Actions:
//!
//! <https://github.com/wtfm-rs/wtfm-rs-test/actions>
//!

/// ```
/// pub fn vec_push_pop() {
///     let mut vec = std::vec::Vec::new();
///     vec.push(1);
///     vec.push(2);
///
///     assert_eq!(vec.len(), 2);
///     assert_eq!(vec[0], 1);
///
///     assert_eq!(vec.pop(), Some(2));
///     assert_eq!(vec.len(), 1);
/// }
/// vec_push_pop();
/// ```
pub fn vec_push_pop() {
    let mut vec = std::vec::Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
}

/// ```
/// pub fn vec_extend() {
///    let mut vec = std::vec::Vec::new();
///    vec.push(1);
///    assert_eq!(vec[0], 1);
///    vec[0] = 7;
///    assert_eq!(vec[0], 7);
///    vec.extend([1, 2, 3]);
///    assert_eq!(vec, [7, 1, 2, 3]);
/// }
/// vec_extend();
/// ```
pub fn vec_extend() {
    let mut vec = std::vec::Vec::new();
    vec.push(1);
    assert_eq!(vec[0], 1);
    vec[0] = 7;
    assert_eq!(vec[0], 7);
    vec.extend([1, 2, 3]);
    assert_eq!(vec, [7, 1, 2, 3]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_push_pop() {
        crate::vec_push_pop();
    }
    #[test]
    fn test_vec_extend() {
        crate::vec_extend();
    }
}
