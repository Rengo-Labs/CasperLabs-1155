import { config } from "dotenv";
config();
import { ERC1155Client, utils } from "../src";
import { getDeploy } from "./utils";

import {
  Keys,
  encodeBase16,
  CasperServiceByJsonRPC
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

const cc = new CasperServiceByJsonRPC(process.env.NODE_ADDRESS!);

const fetchUrefValue = async (uref : any) => {
  const stateRootHash = await cc.getStateRootHash();

  const value = await cc.getBlockState(
    stateRootHash, 
    uref,
    []
  );
  return value;
}

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


const balanceOfBatchSessionCode = async () => {
  const erc1155 = new ERC1155Client(
    NODE_ADDRESS!,
    CHAIN_NAME!,
    EVENT_STREAM_ADDRESS!
  );
  
  const userAccountHash=encodeBase16(KEYS.accountHash());
  const ids=["1","2"];
  const functionName="balance_of_batch";

  const balanceOfBatchsessioncodeDeployHash = await erc1155.balanceOfBatchsessioncode(
    KEYS,
    ERC1155_PACKAGE_HASH!,
    functionName,
    ids,
    [userAccountHash,userAccountHash],
    ERC1155_INSTALL_PAYMENT_AMOUNT!,
    ERC1155_PROXY_WASM_PATH!
  );
  
  console.log(`... balanceOfBatchsessioncode Function deployHash: ${balanceOfBatchsessioncodeDeployHash}`);

  await getDeploy(NODE_ADDRESS!, balanceOfBatchsessioncodeDeployHash);

  console.log(`... balanceOfBatchsessioncode Function called successfully through sessionCode.`);

};

const querybalanceOfBatchResult = async () => {
  const userAccountHash=encodeBase16(KEYS.accountHash());
  const ids=["1","2"];
  const functionName="balance_of_batch";
  let accountInfo = await utils.getAccountInfoForBackend(process.env.NODE_ADDRESS!, userAccountHash);
  console.log(accountInfo);
  const data = await utils.getAccountNamedKeyValue(
    accountInfo,
    functionName
  );
  console.log("data in uref: ", data);

  let urefValue= await fetchUrefValue(data);
  console.log("urefValue", urefValue);
  
  for(var i=0;i<ids.length;i++)
  {
    console.log("ids "+ids[i] +" balance on useraccountHash = ", parseInt(urefValue.CLValue?.data[i].data._hex));
  }

};

//balanceOfBatchSessionCode();
querybalanceOfBatchResult(); 