import { CLAccountHash, CLByteArray, CLPublicKey } from "casper-js-sdk";

export type RecipientType = CLPublicKey | CLAccountHash | CLByteArray;

export interface IPendingDeploy {
  deployHash: string;
}
