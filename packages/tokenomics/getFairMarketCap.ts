/**
 * Calculates the fair market capitalization of a token.
 *
 * @param tokenPriceUSD - Price of one token in USD
 * @param totalSupply - Total token supply (including circulating or max supply)
 * @returns Market capitalization in USD
 */
export function getFairMarketCap(
  tokenPriceUSD: number,
  totalSupply: number
): number {
  if (tokenPriceUSD < 0) throw new Error("Token price cannot be negative");
  if (totalSupply < 0) throw new Error("Total supply cannot be negative");

  return tokenPriceUSD * totalSupply;
}
