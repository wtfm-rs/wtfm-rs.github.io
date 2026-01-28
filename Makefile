all:	fmt clippy introduction macro_expand test expand clean

introduction:
	rustdoc introduction.rs
	rustdoc --test introduction.rs
	rustc --test introduction.rs  && ./introduction && rm ./introduction

macro_expand:
	rustdoc macro_expand.rs
	rustdoc --test macro_expand.rs
	rustc --test macro_expand.rs  && ./macro_expand && rm ./macro_expand

expand:
	cargo install cargo-expand
	cargo expand --lib --tests

fmt:
	cargo fmt

clippy:
	cargo clippy

test:
	cargo test

clean:
	cargo clean
