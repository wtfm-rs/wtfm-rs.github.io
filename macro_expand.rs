//! # Read tests after macro expansion
//!
//! Let's have place holder tests in this crate:
//! ```
//! assert!(true);
//! ```
//! ```panic
//! assert!(false, "false wasn't true!");
//! ```
//! We can run
//! ```sh
//! cargo expand --lib --tests
//! ```
//! To view the whold crate (introduction) with all the macros expanded.
//! ```sh
//! #![feature(prelude_import)]
//! # Introduction
//! ...
//! #[rustc_main]
//! #[coverage(off)]
//! #[doc(hidden)]
//! pub fn main() -> () {
//!    extern crate test;
//!    test::test_main_static(
//!        &[
//!            &assert_true,
//!            &assert_wtfm,
//!            &macro_expand_assert_false,
//!            &macro_expand_assert_true,
//!        ],
//!    )
//! }
//! ```
//! This is going to be a great opportunity to dive in to the details.
//! You can also confirm the output in GitHub Actions
//! <https://github.com/wtfm-rs/wtfm-rs.github.io/actions?query=branch%3Amain>
//!
//!

#[cfg(test)]
#[test]
fn macro_expand_assert_true() {
    assert!(true);
}

#[test]
#[should_panic]
fn macro_expand_assert_false() {
    assert!(false, "false wasn't true!");
}
