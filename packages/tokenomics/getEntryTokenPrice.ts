/**
 * Calculates the value of an ENTRY token based on SOL reclaimed per token account.
 * @param solPrice - Current price of SOL in the desired currency (e.g., USD)
 * @param reclaimedSolPerEntry - (optional) Constant SOL reclaimed per token account (default ~0.00203928)
 */
export const getEntryTokenPrice = (
  solPrice: number,
  reclaimedSolPerEntry = 0.00203928
): number => {
  return solPrice * reclaimedSolPerEntry;
};
