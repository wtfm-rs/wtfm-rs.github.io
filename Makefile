all:	fmt clippy introduction clean

introduction:
	rustdoc introduction.rs
	rustdoc --test introduction.rs
	rustc --test introduction.rs  && ./introduction

fmt:
	cargo fmt

clippy:
	cargo clippy

clean:
	cargo clean
	rm -rf introduction
