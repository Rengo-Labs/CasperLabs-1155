# ERC1155-jsClient


#### Generate the keys

Paste this command on the ubuntu terminal, that will create a keys folder for you containing public key , public key hex and secret key.

```
casper-client keygen keys

```
#### Paste the keys

Paste the keys folder created by the above command to ERC1155 folder.

#### Fund the key

We can fund the keys from casper live website faucet page on testnet.


## Testing

Use the script file in package.json to perform the testing
```
"scripts": {
    "test:erc1155install": "ts-node ERC1155/test/install.ts",
    "test:erc1155installed": "ts-node ERC1155/test/installed.ts",
  },
```

Use the following commands to perform testing
```
npm run test:erc1155install
npm run test:erc1155installed

```

* CONFIGURE .env BEFORE TESTING

