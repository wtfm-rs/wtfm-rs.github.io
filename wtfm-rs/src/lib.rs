//! See <https://en.wikipedia.org/wiki/RTFM>
//!
//! ```
//! assert_eq!("WTFM", "RTFM".replace("R", "W"));
//! ```
//!

/// ## This can be used by other crates.
///
/// `use wtfm_rs::add;`
///
/// ```sh
/// cargo new --lib --name wtfm-test /tmp/wtfm-test
/// cd /tmp/wtfm-test && cargo add --git https://github.com/wtfm-rs/wtfm-rs.github.io  && cd -
/// mkdir /tmp/wtfm-test/tests
/// cat > /tmp/wtfm-test/tests/it-works <<EOF
/// #[test]
/// fn it_works() {
///     use wtfm_rs::add;
///     assert_eq!(add(2, 2), 4);
/// }
/// EOF
/// cd /tmp/wtfm-test && cargo test --tests && cd -
/// rm -rf /tmp/wtfm-test
/// ```
///
/// Local test on my mac (`~/github/wtfm-rs/wtfm-rs.github.io`)
/// ```sh
/// This has been tested on MacOS
///    Creating library `wtfm-test` package
/// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
///    Updating git repository `https://github.com/wtfm-rs/wtfm-rs.github.io`
///    Updating git repository `https://github.com/wtfm-rs/wtfm-rs.github.io`
///      Adding wtfm-rs (git) to dependencies
///    Updating git repository `https://github.com/wtfm-rs/wtfm-rs.github.io`
///     Locking 1 package to latest Rust 1.92.0 compatible version
/// ~/github/wtfm-rs/wtfm-rs.github.io
///   Compiling wtfm-rs v0.1.0 (https://github.com/wtfm-rs/wtfm-rs.github.io#e9c6d190)
///   Compiling wtfm-test v0.1.0 (/private/tmp/wtfm-test)
///    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.24s
///     Running unittests src/lib.rs (target/debug/deps/wtfm_test-c3f9b0c7c40b2943)
///
/// running 1 test
/// test tests::it_works ... ok
///
/// test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
///
/// ~/github/wtfm-rs/wtfm-rs.github.io
/// ```
///
/// GitHub Action CI tests: 
///
/// <https://github.com/wtfm-rs/wtfm-rs-test>
///
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
