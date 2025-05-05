import { getSecurityTokenFairPrice } from "./getSecurityTokenPrice";

/**
 * Calculates the market cap of the security token based on ENTRY token
 * price derived from reclaimed SOL.
 * @param solPrice - Current SOL price in desired currency (e.g., USD)
 * @param tokensPerEntry - Number of security tokens required per ENTRY (default 1000)
 * @param supply - Total supply of the security token (default 1_000_000_000)
 * @returns Market cap of the security token in the same denomination as solPrice
 */
export const getSecurityTokenMarketCap = (
  solPrice: number,
  tokensPerEntry = 1000,
  supply = 1_000_000_000
): number => {
  const securityTokenPrice = getSecurityTokenFairPrice(
    solPrice,
    tokensPerEntry
  );
  return securityTokenPrice * supply;
};
