import { config } from "dotenv";
config();
import { ERC1155Client ,utils} from "../src";
import { getDeploy } from "./utils";

import {
  Keys,
  encodeBase16
} from "casper-js-sdk";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  ERC1155_MASTER_KEY_PAIR_PATH,
  ERC1155_CONTRACT_HASH,
  PAYMENT_AMOUNT,
  OPERATOR,
  TO,
  ID,
  AMOUNT,
  MINTAMOUNT,
  DATA,
} = process.env;


const KEYS = Keys.Ed25519.parseKeyFiles(
  `${ERC1155_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${ERC1155_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);


const erc1155 = new ERC1155Client(
  NODE_ADDRESS!,
  CHAIN_NAME!,
  EVENT_STREAM_ADDRESS!
);

const test = async () => {

  await erc1155.setContractHash(ERC1155_CONTRACT_HASH!);
  const userAccountHash=encodeBase16(KEYS.accountHash());
  const ids=["1","2"];
  const amounts=["50","50"];

  // //uri
  const uri = await erc1155.uri();
  console.log(`... uri: ${uri}`);

  // //isApproveForAll
  const isApprovedForAll = await erc1155.isApprovedForAll(userAccountHash,OPERATOR!);
  console.log(`... isApproveForAll: ${isApprovedForAll}`);

  //mint
  const mintDeployHash1 = await erc1155.mint(
    KEYS,
    KEYS.publicKey,
    ids[0],
    MINTAMOUNT!,
    DATA!,
    PAYMENT_AMOUNT!
  );
  console.log("... mint1 deploy hash: ", mintDeployHash1);

  await getDeploy(NODE_ADDRESS!, mintDeployHash1);
  console.log("... mint1 function called successfully.");

  //balanceOf
  const balanceOfAfterMint1 = await erc1155.balanceOf(ids[0],userAccountHash);
  console.log(`... balanceOf AfterMint1: ${balanceOfAfterMint1}`);
  
  //mint
  const mintDeployHash2 = await erc1155.mint(
    KEYS,
    KEYS.publicKey,
    ids[1],
    MINTAMOUNT!,
    DATA!,
    PAYMENT_AMOUNT!
  );
  console.log("... mint2 deploy hash: ", mintDeployHash2);

  await getDeploy(NODE_ADDRESS!, mintDeployHash2);
  console.log("... mint2 function called successfully.");

  //balanceOf
  const balanceOfAfterMint2 = await erc1155.balanceOf(ids[1],userAccountHash);
  console.log(`... balanceOf AfterMint2: ${balanceOfAfterMint2}`);

  //You must have minted the amount first on the id, which you want to burn
  //burn
  const burnDeployHash = await erc1155.burn(
    KEYS,
    KEYS.publicKey,
    ids[0]!,
    AMOUNT!,
    PAYMENT_AMOUNT!
  );
  console.log("... burn deploy hash: ", burnDeployHash);

  await getDeploy(NODE_ADDRESS!, burnDeployHash);
  console.log("... burn function called successfully.");

  //balanceOf
  const balanceOfAfterBurn = await erc1155.balanceOf(ids[0]!,userAccountHash);
  console.log(`... balanceOf AfterBurn: ${balanceOfAfterBurn}`);
  
  
  //You must have minted the amount first on the ids, which you want to burn
  //burnBatch
   const burnBatchDeployHash = await erc1155.burnBatch(
    KEYS,
    KEYS.publicKey,
    ids!,
    amounts!,
    PAYMENT_AMOUNT!
  );
  console.log("... burnBatch deploy hash: ", burnBatchDeployHash);

  await getDeploy(NODE_ADDRESS!, burnBatchDeployHash);
  console.log("... burnBatch function called successfully.");

  //balanceOf
  const balanceOfAfterBurnBatchid1 = await erc1155.balanceOf(ids[0],userAccountHash);
  console.log(`... balanceOf AfterBurnBatchid1: ${balanceOfAfterBurnBatchid1}`);

  //balanceOf
  const balanceOfAfterBurnBatchid2 = await erc1155.balanceOf(ids[1],userAccountHash);
  console.log(`... balanceOf AfterBurnBatchid2: ${balanceOfAfterBurnBatchid2}`);


  //setApprovalForAll
  const setApprovalForAllDeployHash = await erc1155.setApprovalForAll(
    KEYS,
    OPERATOR!,
    true,
    PAYMENT_AMOUNT!
  );
  console.log("... setApprovalForAll deploy hash: ", setApprovalForAllDeployHash);

  await getDeploy(NODE_ADDRESS!, setApprovalForAllDeployHash);
  console.log("... setApprovalForAll function called successfully.");

  //call setApprovalForAll before transferFrom and pass true in the approved param
  //safeTransferFrom
  const safeTransferFromDeployHash = await erc1155.safeTransferFrom(
    KEYS,
    KEYS.publicKey,
    TO!,
    ids[0]!,
    AMOUNT!,
    DATA!,
    PAYMENT_AMOUNT!
  );
  console.log("... safeTransferFrom deploy hash: ", safeTransferFromDeployHash);

  await getDeploy(NODE_ADDRESS!, safeTransferFromDeployHash);
  console.log("... safeTransferFrom function called successfully.");
  
  //balanceOf
  const balanceOfAfterTransferFrom = await erc1155.balanceOf(ids[0]!,TO!);
  console.log(`... balanceOf AfterTransferFrom : ${balanceOfAfterTransferFrom }`);
  
  //call setApprovalForAll before transferFrom and pass true in the approved param
  //safeBatchTransferFrom
  const safeBatchTransferFromDeployHash = await erc1155.safeBatchTransferFrom(
    KEYS,
    KEYS.publicKey,
    TO!,
    ids!,
    amounts!,
    DATA!,
    PAYMENT_AMOUNT!
  );
  console.log("... safeBatchTransferFrom deploy hash: ", safeBatchTransferFromDeployHash);

  await getDeploy(NODE_ADDRESS!, safeBatchTransferFromDeployHash);
  console.log("... safeBatchTransferFrom function called successfully.");
  
  //balanceOf
  const balanceOfAfterTransferFromBatchid1 = await erc1155.balanceOf(ids[0],TO!);
  console.log(`... balanceOf AfterTransferFromBatchid1: ${balanceOfAfterTransferFromBatchid1 }`);

  //balanceOf
  const balanceOfAfterTransferFromBatchid2 = await erc1155.balanceOf(ids[1],TO!);
  console.log(`... balanceOf AfterTransferFromBatchid2 : ${balanceOfAfterTransferFromBatchid2 }`);

};

test();
