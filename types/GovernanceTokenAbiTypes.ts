export type GovernanceToken = {
  delegateWithSig(delegatee: `0x${string}`, nonce: bigint, expiry: bigint, v: bigint, r: string, s: string): Promise<void>;
  getAccountSnapshot(account: `0x${string}`): Promise<[boolean, bigint, bigint, bigint]>;
  multicall(data: string): Promise<string>;
  setMetadata(key: string, value: string): Promise<boolean>;
}
