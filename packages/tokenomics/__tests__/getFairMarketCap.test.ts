import { getFairMarketCap } from "../getFairMarketCap";

describe("getFairMarketCap", () => {
  it("should calculate market cap correctly for valid inputs", () => {
    const result = getFairMarketCap(0.00035513, 1_000_000_000);
    expect(result).toBeCloseTo(355130);
  });

  it("should return 0 market cap if token price is 0", () => {
    const result = getFairMarketCap(0, 1_000_000_000);
    expect(result).toBe(0);
  });

  it("should return 0 market cap if total supply is 0", () => {
    const result = getFairMarketCap(1.5, 0);
    expect(result).toBe(0);
  });

  it("should handle fractional supply and price values", () => {
    const result = getFairMarketCap(0.002, 500_000.5);
    expect(result).toBeCloseTo(1000.001);
  });

  it("should throw if token price is negative", () => {
    expect(() => {
      getFairMarketCap(-1, 100_000);
    }).toThrow("Token price cannot be negative");
  });

  it("should throw if total supply is negative", () => {
    expect(() => {
      getFairMarketCap(1, -100_000);
    }).toThrow("Total supply cannot be negative");
  });
});
