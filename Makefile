prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build --release -p erc1155 -p erc1155-session-code --target wasm32-unknown-unknown
build-contract-mock-contract:
	cargo build --release -p mock-contract -p erc1155-session-code --target wasm32-unknown-unknown
	wasm-strip target/wasm32-unknown-unknown/release/mock-contract.wasm 2>/dev/null | true
test-only:
	cargo test -p erc1155-tests
test-only-mock-contract:
	cargo test -p mock-contract-tests
copy-wasm-file-to-test:
	cp target/wasm32-unknown-unknown/release/*.wasm ./erc1155-tests/wasm
copy-wasm-file-to-mock-contract:
	cp target/wasm32-unknown-unknown/release/*.wasm mock-contract/mock-contract-tests/wasm

test: build-contract copy-wasm-file-to-test test-only
test-mock-contract: build-contract-mock-contract copy-wasm-file-to-mock-contract test-only-mock-contract
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