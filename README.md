# CSPR-1155-DEVXDAO-GRANT-RUST-BASE

Implementation of ERC1155 token for the CasperLabs platform.

Compiled and tested using 
rustc 1.54.0-nightly (8cf990c9b 2021-05-15)
cargo 1.54.0-nightly (070e459c2 2021-05-11)

## Install
Make sure `wasm32-unknown-unknown` is installed.
```bash
$ make prepare
```

## Build Smart Contract
```bash
$ make build-contract
```

## Test
Test logic and smart contract.
```bash
$ make test
```
