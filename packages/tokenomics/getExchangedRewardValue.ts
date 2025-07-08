/**
 * Calculates the value of a reward token based on the amount of
 * SOL reclaimed in the token exchange process.
 *
 * @param solPrice - Current price of SOL in the desired currency (e.g., USD)
 * @param reclaimedSolPerEntry - (optional) Constant SOL reclaimed per token account (default ~0.00203928)
 */
export const getExchangedRewardValue = (
  solPrice: number,
  reclaimedSolPerEntry = 0.00203928
): number => {
  return solPrice * reclaimedSolPerEntry;
};
