import { getMintedTokenPrice } from "../getMintedTokenPrice";

describe("getMintedTokenPrice", () => {
  it("should correctly calculate minted token price for normal values", () => {
    const result = getMintedTokenPrice(0.36, 1000);
    expect(result).toBeCloseTo(0.00036);
  });

  it("should return the same price if reward ratio is 1", () => {
    const result = getMintedTokenPrice(1.23, 1);
    expect(result).toBeCloseTo(1.23);
  });

  it("should handle small entry prices", () => {
    const result = getMintedTokenPrice(0.01, 1000);
    expect(result).toBeCloseTo(0.00001);
  });

  it("should handle large reward ratios", () => {
    const result = getMintedTokenPrice(10, 1_000_000);
    expect(result).toBeCloseTo(0.00001);
  });

  it("should throw an error if reward ratio is zero", () => {
    expect(() => {
      getMintedTokenPrice(1, 0);
    }).toThrow("Reward ratio must be greater than zero");
  });

  it("should throw an error if reward ratio is negative", () => {
    expect(() => {
      getMintedTokenPrice(1, -500);
    }).toThrow("Reward ratio must be greater than zero");
  });

  it("should return 0 if entry price is 0", () => {
    const result = getMintedTokenPrice(0, 1000);
    expect(result).toBe(0);
  });
});
