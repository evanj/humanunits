all:
	cargo fmt
	cargo test
	cargo check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo verify-project
