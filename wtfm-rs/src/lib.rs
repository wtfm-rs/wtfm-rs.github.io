/// ## This can be used by other crates.
///
/// `use wtfm_rs::add;`
///
/// ```sh
/// cargo new --lib --name tet-wtfm-rs /tmp/test-wtfm-rs
/// cd /tmp/test-wtfm-rs && cargo add --git https://github.com/wtfm-rs/wtfm-rs.github.io wtfm-rs && cd -
/// mkdir /tmp/test-wtfm-rs/tests
/// cat > /tmp/test-wtfm-rs/tests/it-works <<EOF
/// #[test]
/// fn it_works() {
///    use wtfm_rs::add;
///    assert_eq!(add(2, 2), 4);
/// }
/// EOF
/// cd /tmp/test-wtfm-rs && cargo test --tests && cd -
/// rm -rf /tmp/test-wtfm-rs
/// ```
///
/// Local test on my mac (`~/github/wtfm-rs/wtfm-rs.github.io`)
/// ```sh
///     Creating library `tet-wtfm-rs` package
/// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
///    Updating git repository `https://github.com/wtfm-rs/wtfm-rs.github.io`
///      Adding wtfm-rs (git) to dependencies
///    Updating git repository `https://github.com/wtfm-rs/wtfm-rs.github.io`
///     Locking 1 package to latest Rust 1.92.0 compatible version
/// ~/github/wtfm-rs/wtfm-rs.github.io
///   Compiling wtfm-rs v0.1.0 (https://github.com/wtfm-rs/wtfm-rs.github.io#362925b5)
///   Compiling tet-wtfm-rs v0.1.0 (/private/tmp/test-wtfm-rs)
///    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.24s
///     Running unittests src/lib.rs (target/debug/deps/tet_wtfm_rs-2fec6c5d019a0c2d)
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
