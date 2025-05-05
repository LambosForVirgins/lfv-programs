import { getEntryTokenPrice } from "./getEntryTokenPrice";

/**
 * Calculates the fair price of a security token that can be staked to earn ENTRY tokens.
 * @param solPrice - Current price of SOL in desired currency (e.g., USD)
 * @param tokensPerEntry - How many security tokens are required to earn 1 ENTRY (default 1000)
 * @param reclaimedSolPerEntry - (optional) Constant SOL reclaimed per token account (default ~0.00203928)
 */
export const getSecurityTokenFairPrice = (
  solPrice: number,
  tokensPerEntry = 1000,
  reclaimedSolPerEntry = 0.00203928
): number => {
  const entryValue = getEntryTokenPrice(solPrice, reclaimedSolPerEntry);
  return entryValue / tokensPerEntry;
};
