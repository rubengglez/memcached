integration-tests:
	cargo test --test '*' -- --nocapture

clippy:
	cargo clippy