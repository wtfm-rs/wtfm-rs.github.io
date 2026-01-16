all:
	cargo update
	cargo test
	cargo doc --release --target-dir docs
