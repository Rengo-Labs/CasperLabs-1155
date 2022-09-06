import { config } from "dotenv";
config();
import { ERC1155Client ,utils} from "../src";
import { sleep, getDeploy } from "./utils";

import {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  CLAccountHash,
  CLPublicKeyType,
  Contracts,
  CLByteArray
} from "casper-js-sdk";

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  ERC1155_MASTER_KEY_PAIR_PATH,
  ERC1155_CONTRACT_HASH,
  TOKEN_ID,
  PAYMENT_AMOUNT,
  OPERATOR,
  TO,
  ID,
  AMOUNT,
  DATA,
} = process.env;


const KEYS = Keys.Ed25519.parseKeyFiles(
  `${ERC1155_MASTER_KEY_PAIR_PATH}/public_key.pem`,
  `${ERC1155_MASTER_KEY_PAIR_PATH}/secret_key.pem`
);

function splitdata(data:string)
{
    var temp=data.split('(');
    var result=temp[1].split(')');
    return result[0];
}

const erc1155 = new ERC1155Client(
  NODE_ADDRESS!,
  CHAIN_NAME!,
  EVENT_STREAM_ADDRESS!
);

const test = async () => {

  await erc1155.setContractHash(ERC1155_CONTRACT_HASH!);

  // //uri
  // const uri = await erc1155.uri();
  // console.log(`... Contract name: ${uri}`);

  //balanceOf
  // const balanceOf = await erc1155.balanceOf(TOKEN_ID!,"24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1");
  // console.log(`... Contract balanceOf: ${balanceOf}`);


  //isApproveForAll
  // const isApprovedForAll = await erc1155.isApprovedForAll("24a56544c522eca7fba93fb7a6cef83e086706fd87b2f344f5c3dad3603d11f1",OPERATOR!);
  // console.log(`... Contract isApproveForAll: ${isApprovedForAll}`);


  //uri
  // const uriDeployHash = await erc1155.uri(
  //   KEYS,
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... uri deploy hash: ", uriDeployHash);

  // await getDeploy(NODE_ADDRESS!, uriDeployHash);
  // console.log("... uri function called successfully.");
 
  // //balanceOf
  // const balanceOfDeployHash = await erc1155.balanceOf(
  //   KEYS,
  //   TOKEN_ID!,
  //   KEYS.publicKey!,
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... balanceOf deploy hash: ", balanceOfDeployHash);

  // await getDeploy(NODE_ADDRESS!, balanceOfDeployHash);
  // console.log("... balanceOf function called successfully.");

  // //balanceOfBatch
  // const balanceOfBatchDeployHash = await erc1155.balanceOfBatch(
  //   KEYS,
  //   [''],
  //   [''],
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... balanceOfBatch deploy hash: ", balanceOfBatchDeployHash);

  // await getDeploy(NODE_ADDRESS!, balanceOfBatchDeployHash);
  // console.log("... balanceOfBatch function called successfully.");

  // //setApprovalForAll
  // const setApprovalForAllDeployHash = await erc1155.setApprovalForAll(
  //   KEYS,
  //   OPERATOR!,
  //   //KEYS.publicKey,
  //   true,
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... setApprovalForAll deploy hash: ", setApprovalForAllDeployHash);

  // await getDeploy(NODE_ADDRESS!, setApprovalForAllDeployHash);
  // console.log("... setApprovalForAll function called successfully.");

  // //isApprovedForAll
  // const isApprovedForAllDeployHash = await erc1155.isApprovedForAll(
  //   KEYS,
  //   KEYS.publicKey,
  //   OPERATOR!,
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... isApprovedForAll deploy hash: ", isApprovedForAllDeployHash);

  // await getDeploy(NODE_ADDRESS!, isApprovedForAllDeployHash);
  // console.log("... isApprovedForAll function called successfully.");

  //safeTransferFrom
  // const safeTransferFromDeployHash = await erc1155.safeTransferFrom(
  //   KEYS,
  //   //"781d4ebe2ec8451f52deede21d54b495edb5d1325153c1453a8504cab77824fd",
  //   KEYS.publicKey,
  //   "8427ea5d527fb775a7d82624b44120abaa7b5c37d7da7fbbe09448d8641da225",
  //   ID!,
  //   AMOUNT!,
  //   DATA!,
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... safeTransferFrom deploy hash: ", safeTransferFromDeployHash);

  // await getDeploy(NODE_ADDRESS!, safeTransferFromDeployHash);
  // console.log("... safeTransferFrom function called successfully.");

  // //safeBatchTransferFrom
  // const safeBatchTransferFromDeployHash = await erc1155.safeBatchTransferFrom(
  //   KEYS,
  //   KEYS.publicKey,
  //   TO!,
  //   ['4','5']!,
  //   ['50','50']!,
  //   DATA!,
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... safeBatchTransferFrom deploy hash: ", safeBatchTransferFromDeployHash);

  // await getDeploy(NODE_ADDRESS!, safeBatchTransferFromDeployHash);
  // console.log("... safeBatchTransferFrom function called successfully.");

  //mint
  // const mintDeployHash = await erc1155.mint(
  //   KEYS,
  //   KEYS.publicKey,
  //   //"781d4ebe2ec8451f52deede21d54b495edb5d1325153c1453a8504cab77824fd",
  //   ID!,
  //   AMOUNT!,
  //   DATA!,
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... mint deploy hash: ", mintDeployHash);

  // await getDeploy(NODE_ADDRESS!, mintDeployHash);
  // console.log("... mint function called successfully.");

  // //burn
  // const burnDeployHash = await erc1155.burn(
  //   KEYS,
  //   KEYS.publicKey,
  //   ID!,
  //   AMOUNT!,
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... burn deploy hash: ", burnDeployHash);

  // await getDeploy(NODE_ADDRESS!, burnDeployHash);
  // console.log("... burn function called successfully.");

  //  //burnBatch
  //  const burnBatchDeployHash = await erc1155.burnBatch(
  //   KEYS,
  //   KEYS.publicKey,
  //   ["2","3"]!,
  //   ["50","50"],
  //   PAYMENT_AMOUNT!
  // );
  // console.log("... burnBatch deploy hash: ", burnBatchDeployHash);

  // await getDeploy(NODE_ADDRESS!, burnBatchDeployHash);
  // console.log("... burnBatch function called successfully.");

};


test();
