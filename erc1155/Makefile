prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p erc1155 --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/erc1155.wasm 2>/dev/null | true

test-only:
	cargo test -p erc1155-tests -- --nocapture

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm erc1155-tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cargo fmt --all -- --check

lint: clippy
	cargo fmt --all

clean:
	cargo clean
	rm -rf erc1155-tests/wasm/*.wasm
