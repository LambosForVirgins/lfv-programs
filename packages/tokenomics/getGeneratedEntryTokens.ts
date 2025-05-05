/**
 * Returns how many ENTRY tokens are generated based on currently staked security tokens.
 * @param stakedTokens
 * @param tokensPerEntry
 * @returns
 */
export const getGeneratedEntryTokens = (
  stakedTokens: number,
  tokensPerEntry: number = 1000
): number => {
  return Math.floor(stakedTokens / tokensPerEntry);
};
