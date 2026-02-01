all: 	check clippy fmt test doc
	cp -r target/doc .

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
	cargo run --example example-assert-true

clean:
	cargo clean
	rm -rf doc
