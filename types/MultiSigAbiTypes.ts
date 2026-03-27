export type MultiSig = {
  submitTransaction(_tx: any): Promise<bigint>;
  getConfirmations(arg0: bigint): Promise<`0x${string}`[]>;
}
