export type ERC20 = {
  totalSupply(): Promise<bigint>;
  transfer(recipient: `0x${string}`, amount: bigint): Promise<boolean>;
}
