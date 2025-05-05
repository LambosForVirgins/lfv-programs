import { getSecurityTokenFairPrice } from "./getSecurityTokenPrice";

interface PriceMarginResult {
  fairPrice: number;
  actualMarketPrice: number;
  absoluteMargin: number;
  percentageMargin: number; // positive = premium, negative = discount
}

/**
 * Calculates the margin between fair value (based on reclaimed SOL -> ENTRY) and market price.
 * @param solPrice - Current price of SOL in the desired currency (e.g., USD)
 * @param actualMarketPrice - Observed market price of the token
 * @param tokensPerEntry - How many tokens are required per ENTRY reward
 * @param reclaimedSolPerEntry - How much SOL is reclaimed per ENTRY (default ~0.00203928)
 */
export function getFairMarketPriceMargin(
  solPrice: number,
  actualMarketPrice: number,
  tokensPerEntry = 1000,
  reclaimedSolPerEntry = 0.00203928
): PriceMarginResult {
  const fairPrice = getSecurityTokenFairPrice(
    solPrice,
    tokensPerEntry,
    reclaimedSolPerEntry
  );
  const absoluteMargin = actualMarketPrice - fairPrice;
  const percentageMargin = (absoluteMargin / fairPrice) * 100;

  return {
    fairPrice,
    actualMarketPrice,
    absoluteMargin,
    percentageMargin,
  };
}
