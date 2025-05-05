import { getSecurityTokenMarketCap } from "../getMarketCap";

describe(getSecurityTokenMarketCap, () => {
  it("returns correct market cap with default inputs", () => {
    const solPrice = 150;
    const tokensPerEntry = 1000;
    const supply = 1_000_000_000;
    const expected = ((150 * 0.00203928) / tokensPerEntry) * supply;
    expect(getSecurityTokenMarketCap(solPrice)).toBeCloseTo(expected, 0);
  });
});
