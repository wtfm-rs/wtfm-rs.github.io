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
//! ## Refactor driven development
//! Be prepared to [RTFM](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html).
//!
//! ### assert!
//!
//! You can find the doc about them in [std].
//! 
//! Suppose we have a simple `assert_true` function that always passes:
//! ```
//! fn assert_true() {
//!     assert!(true);
//! }
//! assert_true();
//! ```
