import {
  CasperClient,
  CLPublicKey,
  CLAccountHash,
  CLByteArray,
  CLKey,
  CLString,
  CLTypeBuilder,
  CLValue,
  CLValueBuilder,
  CLValueParsers,
  CLMap,
  DeployUtil,
  EventName,
  EventStream,
  Keys,
  RuntimeArgs,
  ToBytes,
} from "casper-js-sdk";
import { Some, None } from "ts-results";
import * as blake from "blakejs";
import { concat } from "@ethersproject/bytes";
import * as utils from "./utils";
import { RecipientType, IPendingDeploy } from "./types";
import {createRecipientAddress } from "./utils";
import { byteHash } from "casper-js-sdk/dist/lib/Contracts";

class ERC1155Client {
  private contractName: string = "erc1155";
  private contractHash: string= "erc1155";
  private contractPackageHash: string= "erc1155";
  private namedKeys: {
    balances:string
    metadata: string;
    nonces: string;
    allowances: string;
    ownedTokens: string;
    owners: string;
    paused: string;
    
  };

  private isListening = false;
  private pendingDeploys: IPendingDeploy[] = [];

  constructor(

    private nodeAddress: string,
    private chainName: string,
    private eventStreamAddress?: string,
    
  ) 
  {
    this.namedKeys= {
      balances:"null",
      metadata: "null",
      nonces: "null",
      allowances: "null",
      ownedTokens: "null",
      owners: "null",
      paused: "null"
    }; 
  }

  public async install(
    keys: Keys.AsymmetricKey,
    uri: string,
    contractName: string,
    paymentAmount: string,
    wasmPath: string
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      uri: CLValueBuilder.string(uri),
      contract_name: CLValueBuilder.string(contractName),
    });

    const deployHash = await installWasmFile({
      chainName: this.chainName,
      paymentAmount,
      nodeAddress: this.nodeAddress,
      keys,
      pathToContract: wasmPath,
      runtimeArgs,
    });

    if (deployHash !== null) {
      return deployHash;
    } else {
      throw Error("Problem with installation");
    }
  }

  public async setContractHash(hash: string) {
    const stateRootHash = await utils.getStateRootHash(this.nodeAddress);
    const contractData = await utils.getContractData(
      this.nodeAddress,
      stateRootHash,
      hash
    );

    const { contractPackageHash, namedKeys } = contractData.Contract!;
    this.contractHash = hash;
    this.contractPackageHash = contractPackageHash.replace(
      "contract-package-wasm",
      ""
    );
    const LIST_OF_NAMED_KEYS = [
      'balances',
      'nonces',
      'allowances',
      `${this.contractName}_package_hash`,
      `${this.contractName}_package_hash_wrapped`,
      `${this.contractName}_contract_hash`,
      `${this.contractName}_contract_hash_wrapped`,
      `${this.contractName}_package_access_token`,
    ];
    // @ts-ignore
    this.namedKeys = namedKeys.reduce((acc, val) => {
      if (LIST_OF_NAMED_KEYS.includes(val.name)) {
        return { ...acc, [utils.camelCased(val.name)]: val.key };
      }
      return acc;
    }, {});
  }

  public async uri(
    keys: Keys.AsymmetricKey,
    paymentAmount: string
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
    });
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "uri",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

 
  public async balanceOf(
    keys: Keys.AsymmetricKey,
    tokenId: string,
    owner: RecipientType,
    paymentAmount: string
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      token_id: CLValueBuilder.u256(tokenId),
      owner: utils.createRecipientAddress(owner),
    });
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "balance_of",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async  balanceOfBatch(
    keys: Keys.AsymmetricKey,
    accounts: string[],//issue
    ids: string[],
    paymentAmount: string,
  ) {
    // const _accounts = new CLByteArray(
		// 	Uint8Array.from(Buffer.from(accounts[''], "hex"))
		// );
    const runtimeArgs = RuntimeArgs.fromMap({
      //accounts: CLValueBuilder.list(_accounts.map(ac => utils.createRecipientAddress(ac))),
      ids: CLValueBuilder.list(ids.map(id => CLValueBuilder.u256(id))),
      accounts: CLValueBuilder.list(accounts.map(accounts => CLValueBuilder.string(accounts)))
    });
  
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "balance_of_batch",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });
  
    if (deployHash !== null) {
      
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async setApprovalForAll(
    keys: Keys.AsymmetricKey,
    operator: string,
    approved: boolean,
    paymentAmount: string
  ) {
    const _operator = new CLByteArray(
			Uint8Array.from(Buffer.from(operator, "hex"))
		);
    const runtimeArgs = RuntimeArgs.fromMap({
      operator: utils.createRecipientAddress(_operator),
      approved: CLValueBuilder.bool(approved),
    });
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "set_approval_for_all",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async isApprovedForAll(
    keys: Keys.AsymmetricKey,
    account: RecipientType,
    operator: string,
    paymentAmount: string
  ) {
    const _operator = new CLByteArray(
			Uint8Array.from(Buffer.from(operator, "hex"))
		);
    const runtimeArgs = RuntimeArgs.fromMap({
      account: utils.createRecipientAddress(account), 
      operator: utils.createRecipientAddress(_operator),
    });
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "is_approved_for_all",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async safeTransferFrom(
    keys: Keys.AsymmetricKey,
    from: RecipientType,
    to: string,
    id:string,
    amount:string,
    data:string,
    paymentAmount: string
  ) {
    const _to = new CLByteArray(
			Uint8Array.from(Buffer.from(to, "hex"))
		);
    const runtimeArgs = RuntimeArgs.fromMap({
      from: utils.createRecipientAddress(from), 
      to: utils.createRecipientAddress(_to),
      id: CLValueBuilder.u256(id),
      amount: CLValueBuilder.u256(amount),
      //_data:data.ToBytes(data), //issue byte
    });
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "safe_transfer_from",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }

  public async safeBatchTransferFrom(
    keys: Keys.AsymmetricKey,
    from: RecipientType,
    to: string,
    ids:string[],
    amounts:string[],
    data:string,
    paymentAmount: string
  ) {
    const _to = new CLByteArray(
			Uint8Array.from(Buffer.from(to, "hex"))
		);
    const runtimeArgs = RuntimeArgs.fromMap({
      from: utils.createRecipientAddress(from), 
      to: utils.createRecipientAddress(_to),
      ids: CLValueBuilder.list(ids.map(id => CLValueBuilder.u256(id))),
      amounts: CLValueBuilder.list(amounts.map(amounts => CLValueBuilder.u256(amounts))),
      _data:CLValueBuilder.u256(data), //issue byte
    });
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "safe_batch_transfer_from",
      keys,
      nodeAddress: this.nodeAddress,
      paymentAmount,
      runtimeArgs,
    });

    if (deployHash !== null) {
      return deployHash;
    } else {
      throw Error("Invalid Deploy");
    }
  }
}

interface IInstallParams {
  nodeAddress: string;
  keys: Keys.AsymmetricKey;
  chainName: string;
  pathToContract: string;
  runtimeArgs: RuntimeArgs;
  paymentAmount: string;
}

const installWasmFile = async ({
  nodeAddress,
  keys,
  chainName,
  pathToContract,
  runtimeArgs,
  paymentAmount,
}: IInstallParams): Promise<string> => {
  const client = new CasperClient(nodeAddress);

  // Set contract installation deploy (unsigned).
  let deploy = DeployUtil.makeDeploy(
    new DeployUtil.DeployParams(
      CLPublicKey.fromHex(keys.publicKey.toHex()),
      chainName
    ),
    DeployUtil.ExecutableDeployItem.newModuleBytes(
      utils.getBinary(pathToContract),
      runtimeArgs
    ),
    DeployUtil.standardPayment(paymentAmount)
  );

  // Sign deploy.
  deploy = client.signDeploy(deploy, keys);

  // Dispatch deploy to node.
  return await client.putDeploy(deploy);
};

interface IContractCallParams {
  nodeAddress: string;
  keys: Keys.AsymmetricKey;
  chainName: string;
  entryPoint: string;
  runtimeArgs: RuntimeArgs;
  paymentAmount: string;
  contractHash: string;
}

const contractCall = async ({
  nodeAddress,
  keys,
  chainName,
  contractHash,
  entryPoint,
  runtimeArgs,
  paymentAmount,
}: IContractCallParams) => {
  const client = new CasperClient(nodeAddress);
  const contractHashAsByteArray = utils.contractHashToByteArray(contractHash);

  let deploy = DeployUtil.makeDeploy(
    new DeployUtil.DeployParams(keys.publicKey, chainName),
    DeployUtil.ExecutableDeployItem.newStoredContractByHash(
      contractHashAsByteArray,
      entryPoint,
      runtimeArgs
    ),
    DeployUtil.standardPayment(paymentAmount)
  );

  // Sign deploy.
  deploy = client.signDeploy(deploy, keys);

  // Dispatch deploy to node.
  const deployHash = await client.putDeploy(deploy);

  return deployHash;
};

const contractSimpleGetter = async (
  nodeAddress: string,
  contractHash: string,
  key: string[]
) => {
  const stateRootHash = await utils.getStateRootHash(nodeAddress);
  const clValue = await utils.getContractData(
    nodeAddress,
    stateRootHash,
    contractHash,
    key
  );

  if (clValue && clValue.CLValue instanceof CLValue) {
    return clValue.CLValue!;
  } else {
    throw Error("Invalid stored value");
  }
};

const toCLMap = (map: Map<string, string>) => {
  const clMap = CLValueBuilder.map([
    CLTypeBuilder.string(),
    CLTypeBuilder.string(),
  ]);
  for (const [key, value] of Array.from(map.entries())) {
    clMap.set(CLValueBuilder.string(key), CLValueBuilder.string(value));
  }
  return clMap;
};

const fromCLMap = (map: Map<CLString, CLString>) => {
  const jsMap = new Map();
  for (const [key, value] of Array.from(map.entries())) {
    jsMap.set(key.value(), value.value());
  }
  return jsMap;
};

export default ERC1155Client;
