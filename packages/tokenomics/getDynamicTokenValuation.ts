import { getEntryTokenPrice } from "./getEntryTokenPrice";

/**
 * Calculates the dynamic fair price of a security token
 * based on how many tokens are staked.
 * @param solPrice
 * @param stakedTokens
 * @param totalSupply
 * @param tokensPerEntry
 * @returns
 */
export const getDynamicTokenValuation = (
  solPrice: number,
  stakedTokens: number,
  totalSupply: number,
  tokensPerEntry: number = 1000
): number => {
  if (totalSupply === 0) return NaN;

  const entryValue = getEntryTokenPrice(solPrice);
  const percentStaked = stakedTokens / totalSupply;

  // Value increases as fewer tokens are staked (scarcity premium)
  // Base price: entry value divided by how many tokens it takes to get 1 ENTRY
  // Adjusted linearly with percent staked
  const baseTokenPrice = entryValue / tokensPerEntry;

  // Optional: add a scarcity multiplier (e.g., 1 / (1 - percentStaked)) for aggressive pricing
  return baseTokenPrice * percentStaked;
};
