import { getSecurityTokenFairPrice } from "../getSecurityTokenPrice";

describe(getSecurityTokenFairPrice, () => {
  it("calculates correct fair price for default parameters and SOL = $150", () => {
    const solPrice = 150;
    const expectedEntryPrice = 150 * 0.00203928;
    const expectedFairPrice = expectedEntryPrice / 1000;

    const result = getSecurityTokenFairPrice(solPrice);
    expect(result).toBeCloseTo(expectedFairPrice, 8);
  });

  it("returns 0 when SOL price is 0", () => {
    const result = getSecurityTokenFairPrice(0);
    expect(result).toBe(0);
  });

  it("supports custom tokensPerEntry values", () => {
    const solPrice = 100;
    const tokensPerEntry = 500;
    const expectedEntryPrice = 100 * 0.00203928;
    const expected = expectedEntryPrice / tokensPerEntry;

    const result = getSecurityTokenFairPrice(solPrice, tokensPerEntry);
    expect(result).toBeCloseTo(expected, 8);
  });

  it("supports high precision with large SOL price", () => {
    const solPrice = 999.999;
    const expectedEntryPrice = solPrice * 0.00203928;
    const expected = expectedEntryPrice / 1000;

    const result = getSecurityTokenFairPrice(solPrice);
    expect(result).toBeCloseTo(expected, 8);
  });
});
