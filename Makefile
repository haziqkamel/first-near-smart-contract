test:
	cargo test -- --nocapture

build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY: test build