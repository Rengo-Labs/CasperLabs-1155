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
  CLList,
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
    balanceOf:string
    metadata: string;
    nonces: string;
    allowances: string;
    ownedTokens: string;
    owners: string;
    paused: string;
    Operator_Approvals: string;
    balances:string;
    
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
      balanceOf:"null",
      metadata: "null",
      nonces: "null",
      allowances: "null",
      ownedTokens: "null",
      owners: "null",
      paused: "null",
      Operator_Approvals:"null",
      balances:"null",
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


  public async balanceOfBatchsessioncode(
    keys: Keys.AsymmetricKey,
    packageHash: string,
    entrypointName:string,
    ids:string[],
    accounts: string[],
    paymentAmount: string,
    wasmPath: string
  ) {
    const _packageHash = new CLByteArray(
			Uint8Array.from(Buffer.from(packageHash, "hex"))
		);
      
    let _accounts : CLString[] = [];
    
    for (let i = 0; i < accounts.length; i++) {
      const p = new CLString("account-hash-".concat(accounts[i]));
      _accounts.push(p);
    }
    
    const runtimeArgs = RuntimeArgs.fromMap({
      package_hash: utils.createRecipientAddress(_packageHash),
      entrypoint: CLValueBuilder.string(entrypointName),
      ids: CLValueBuilder.list(ids.map(id => CLValueBuilder.u256(id))),
      accounts: new CLList(_accounts)
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
      'balance_of',
      'balances',
      'nonces',
      'allowances',
      'Operator_Approvals',
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


  public async uri() {
    const result = await contractSimpleGetter(
      this.nodeAddress,
      this.contractHash,
      ["URI"]
    );
    return result.value();
  }

  public async balanceOf(tokenId: string,account:string) {
     try {
      const keyOwner=new CLKey(new CLAccountHash(Uint8Array.from(Buffer.from(account, "hex"))));
      const token = CLValueBuilder.u256(tokenId);
      const finalBytes = concat([CLValueParsers.toBytes(token).unwrap(), CLValueParsers.toBytes(keyOwner).unwrap()]);
      const blaked = blake.blake2b(finalBytes, undefined, 32);
      const encodedBytes = Buffer.from(blaked).toString("hex");

      const result = await utils.contractDictionaryGetter(
        this.nodeAddress,
        encodedBytes,
        this.namedKeys.balances
      );
      const maybeValue = result.value().unwrap();
      return maybeValue.value().toString();

    } catch (error) {
      return "0";
    }
    
  }

  public async isApprovedForAll(account:string, operator:string) {
    try {
      const _operator = new CLByteArray(
        Uint8Array.from(Buffer.from(operator, "hex"))
      );

      const keyOwner=new CLKey(new CLAccountHash(Uint8Array.from(Buffer.from(account, "hex"))));
      const keyOperator = createRecipientAddress(_operator);
      const finalBytes = concat([CLValueParsers.toBytes(keyOwner).unwrap(), CLValueParsers.toBytes(keyOperator).unwrap()]);
      const blaked = blake.blake2b(finalBytes, undefined, 32);
      const encodedBytes = Buffer.from(blaked).toString("hex");

      const result = await utils.contractDictionaryGetter(
        this.nodeAddress,
        encodedBytes,
        this.namedKeys.Operator_Approvals
      );

      const maybeValue = result.value().unwrap();
      return maybeValue.value().toString();
    } catch (error) {
      return "0";
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

  public async safeTransferFrom(
    keys: Keys.AsymmetricKey,
    from: RecipientType,
    to: string,
    id:string,
    amount:string,
    data:string,
    paymentAmount: string
  ) {

    const _to=new CLKey(new CLAccountHash(Uint8Array.from(Buffer.from(to, "hex"))));
    const runtimeArgs = RuntimeArgs.fromMap({
      from: utils.createRecipientAddress(from), 
      to: _to,
      id: CLValueBuilder.u256(id),
      amount: CLValueBuilder.u256(amount),
      data:CLValueBuilder.string(data)
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
    const _to=new CLKey(new CLAccountHash(Uint8Array.from(Buffer.from(to, "hex"))));
    const runtimeArgs = RuntimeArgs.fromMap({
      from: utils.createRecipientAddress(from), 
      to: _to,
      ids: CLValueBuilder.list(ids.map(id => CLValueBuilder.u256(id))),
      amounts: CLValueBuilder.list(amounts.map(amounts => CLValueBuilder.u256(amounts))),
      data:CLValueBuilder.string(data),
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

  public async mint(
    keys: Keys.AsymmetricKey,
    to: RecipientType,
    id:string,
    amount:string,
    data:string,
    paymentAmount: string
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      to: utils.createRecipientAddress(to),
      id: CLValueBuilder.u256(id),
      amount: CLValueBuilder.u256(amount),
      data:CLValueBuilder.string(data)
    });
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "mint",
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

  public async burn(
    keys: Keys.AsymmetricKey,
    from: RecipientType,
    id:string,
    amount:string,
    paymentAmount: string
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      from: utils.createRecipientAddress(from),
      id: CLValueBuilder.u256(id),
      amount: CLValueBuilder.u256(amount),
    });
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "burn",
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

  public async burnBatch(
    keys: Keys.AsymmetricKey,
    from: RecipientType,
    ids:string[],
    amounts:string[],
    paymentAmount: string
  ) {
    const runtimeArgs = RuntimeArgs.fromMap({
      from: utils.createRecipientAddress(from), 
      ids: CLValueBuilder.list(ids.map(ids => CLValueBuilder.u256(ids))),
      amounts: CLValueBuilder.list(amounts.map(amounts => CLValueBuilder.u256(amounts))),
    });
    const deployHash = await contractCall({
      chainName: this.chainName,
      contractHash: this.contractHash,
      entryPoint: "burn_batch",
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
