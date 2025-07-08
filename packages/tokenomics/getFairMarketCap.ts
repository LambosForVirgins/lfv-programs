/**
 * Calculates the fair market capitalization of a token.
 *
 * @param tokenPrice - Price of one token
 * @param totalSupply - Total token supply (including circulating or max supply)
 * @returns Market capitalization
 */
export function getFairMarketCap(
  tokenPrice: number,
  totalSupply: number
): number {
  if (tokenPrice < 0) throw new Error("Token price cannot be negative");
  if (totalSupply < 0) throw new Error("Total supply cannot be negative");

  return tokenPrice * totalSupply;
}
