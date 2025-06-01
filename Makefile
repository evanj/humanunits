all:
	cargo fmt
	cargo test --all-targets
	cargo check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo verify-project
