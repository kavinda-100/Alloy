export type Vault = {
  batchDeposit(assets: bigint, receiver: `0x${string}`): Promise<void>;
  getVaultStats(): Promise<[bigint, boolean, `0x${string}`]>;
}
