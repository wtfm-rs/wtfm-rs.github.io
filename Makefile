all:
	cargo update
	cargo clippy
	cargo fmt
	cargo test 
	rustdoc introduction.rs
	cargo clean
	
