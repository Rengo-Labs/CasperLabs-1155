prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p erc1155  -p erc1155-proxy --target wasm32-unknown-unknown
	
test-only:
	cargo test -p erc1155-tests

copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm ./erc1155-tests/wasm

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

git-clean:
	git rm -rf --cached .
	git add .