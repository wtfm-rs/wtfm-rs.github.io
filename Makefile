all:
	cargo update
	cargo clean --target-dir docs
	cargo test --target-dir docs
	cargo doc --release --target-dir docs
	rm -rf docs/debug
	rm -rf docs/release
	rm -f docs/.rustc_info.json
	rm -f docs/.rustdoc_fingerprint.json
	rm -f docs/CACHEDIR.TAG
