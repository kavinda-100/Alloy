export type Token = {
  transfer(to: `0x${string}`, amount: bigint): Promise<boolean>;
  balanceOf(account: `0x${string}`): Promise<bigint>;
}
