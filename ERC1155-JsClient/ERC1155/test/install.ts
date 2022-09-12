import { config } from "dotenv";
config();
import { ERC1155Client, utils } from "../src";
import { getDeploy } from "./utils";

import {
  Keys,
  encodeBase16
} from "casper-js-sdk";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  ERC1155_WASM_PATH,
  ERC1155_MASTER_KEY_PAIR_PATH,
  ERC1155_INSTALL_PAYMENT_AMOUNT,
  ERC1155_CONTRACT_NAME,
  URI,
  OPERATOR,
  ERC1155_PACKAGE_HASH,
  ERC1155_PROXY_WASM_PATH,
} = process.env;

const KEYS = Keys.Ed25519.parseKeyFiles(
  `${ERC1155_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${ERC1155_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

const test = async () => {
  const erc1155 = new ERC1155Client(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );

  const installDeployHash = await erc1155.install(
    KEYS,
    URI!,
    ERC1155_CONTRACT_NAME!,
    ERC1155_INSTALL_PAYMENT_AMOUNT!,
    ERC1155_WASM_PATH!
  );

  console.log(`... Contract installation deployHash: ${installDeployHash}`);

  await getDeploy(NODE_ADDRESS!, installDeployHash);

  console.log(`... Contract installed successfully.`);

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS!, KEYS.publicKey);

  console.log(`... Account Info: `);
  console.log(JSON.stringify(accountInfo, null, 2));

  const contractHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${ERC1155_CONTRACT_NAME!}_contract_hash`
  );

  console.log(`... Contract Hash: ${contractHash}`);

  const packageHash = await utils.getAccountNamedKeyValue(
    accountInfo,
    `${ERC1155_CONTRACT_NAME!}_package_hash`
  );

  console.log(`... Package Hash: ${packageHash}`);
};

//test();


const testSessionCode = async () => {
  const erc1155 = new ERC1155Client(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );
  
  const userAccountHash=encodeBase16(KEYS.accountHash());
  const ids=["2","3"];
  const functionName="balance_of_batch";

  const balanceOfBatchsessioncodeDeployHash = await erc1155.balanceOfBatchsessioncode(
    KEYS,
    ERC1155_PACKAGE_HASH!,
    functionName,
    ids,
    [userAccountHash,OPERATOR!],
    ERC1155_INSTALL_PAYMENT_AMOUNT!,
    ERC1155_PROXY_WASM_PATH!
  );
  
  console.log(`... balanceOfBatchsessioncode Function deployHash: ${balanceOfBatchsessioncodeDeployHash}`);

  let result = await getDeploy(NODE_ADDRESS!, balanceOfBatchsessioncodeDeployHash);

  console.log(`... balanceOfBatchsessioncode Function called successfully through sessionCode.`);
  console.log("balanceOfBatchsessioncode Function Result: ",result);

};

testSessionCode();