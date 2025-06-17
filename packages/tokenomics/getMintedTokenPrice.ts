/**
 * Calculates the price of 1 LFV token in USD
 * based on the price of an entry token and staking ratio.
 *
 * @param rewardTokenPrice - Price of 1 reward token
 * @param rewardRatio - Number of minted tokens staked to earn 1 reward token
 * @returns Price of 1 minted token in USD
 */
export function getMintedTokenPrice(
  rewardTokenPrice: number,
  rewardRatio: number
): number {
  return rewardTokenPrice / rewardRatio;
}
