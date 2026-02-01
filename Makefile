all: 	check clippy fmt test doc

check:
	cargo check

clippy:
	cargo clippy

fmt:
	cargo fmt

doc:
	cargo doc --examples

review:
	cargo doc --examples --open

test:
	cargo test
	cargo run --example example-hello-world

clean:
	cargo clean
