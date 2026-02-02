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
	cargo test --examples
	cargo run --example example-assert-true
	cargo run --example example-hello-world
	cargo run --example example-echo-hello-world

clean:
	cargo clean
	rm -rf doc
