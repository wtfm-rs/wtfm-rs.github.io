publish:	
	cargo doc
	cargo test --doc
	cp -r target/doc .

review:
	cargo doc --open

clean:
	cargo clean
