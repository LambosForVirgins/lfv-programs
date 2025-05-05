/**
 * Given a fixed total supply and current staked amount,
 * returns how many more ENTRY tokens can still be awarded.
 * @param totalSupply
 * @param stakedTokens
 * @param tokensPerEntry
 * @returns
 */
export const getRemainingRewardableCapacity = (
  totalSupply: number,
  stakedTokens: number,
  tokensPerEntry: number = 1000
): number => {
  const remainingTokens = totalSupply - stakedTokens;
  return Math.floor(remainingTokens / tokensPerEntry);
};
