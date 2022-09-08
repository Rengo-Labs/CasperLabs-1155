# CasperLabs ERC1155

Implementation of the ERC1155 token standard for the Casper platform.

## Usage

### Install

Make sure `wasm32-unknown-unknown` is installed.

```
make prepare
```

It's also recommended to have [wasm-strip](https://github.com/WebAssembly/wabt)
available in your PATH to reduce the size of compiled Wasm.

### Build Smart Contract

```
make build-contract
```

### Test

Test logic and smart contract.

```
make test
```

### Test Mock Contract

Test logic and smart contract.

```
make test-mock-contract
```
