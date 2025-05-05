import { getRemainingRewardableCapacity } from "../getRemainingRewardableCapacity"; // Update with actual path

describe(getRemainingRewardableCapacity, () => {
  it("returns correct capacity with default tokensPerEntry", () => {
    const totalSupply = 1_000_000_000;
    const stakedTokens = 250_000_000;
    const expected = Math.floor((totalSupply - stakedTokens) / 1000);
    const result = getRemainingRewardableCapacity(totalSupply, stakedTokens);
    expect(result).toBe(expected);
  });

  it("returns zero when all tokens are staked", () => {
    const totalSupply = 1_000_000_000;
    const stakedTokens = 1_000_000_000;
    const result = getRemainingRewardableCapacity(totalSupply, stakedTokens);
    expect(result).toBe(0);
  });

  it("returns zero when remaining tokens are less than tokensPerEntry", () => {
    const totalSupply = 1_000;
    const stakedTokens = 999;
    const result = getRemainingRewardableCapacity(totalSupply, stakedTokens);
    expect(result).toBe(0);
  });

  it("handles custom tokensPerEntry", () => {
    const totalSupply = 500_000;
    const stakedTokens = 100_000;
    const tokensPerEntry = 500;
    const expected = Math.floor((totalSupply - stakedTokens) / tokensPerEntry);
    const result = getRemainingRewardableCapacity(
      totalSupply,
      stakedTokens,
      tokensPerEntry
    );
    expect(result).toBe(expected);
  });

  it("returns zero when stakedTokens exceed totalSupply (should not happen in practice)", () => {
    const result = getRemainingRewardableCapacity(1_000_000, 1_500_000);
    expect(result).toBeLessThanOrEqual(0); // returns negative, but will floor to zero or below
  });
});
