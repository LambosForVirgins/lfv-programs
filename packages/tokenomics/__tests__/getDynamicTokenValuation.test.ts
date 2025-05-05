import { getDynamicTokenValuation } from "../getDynamicTokenValuation";

describe(getDynamicTokenValuation, () => {
  it("calculates correct valuation for 25% staked", () => {
    const solPrice = 150;
    const staked = 250_000_000;
    const total = 1_000_000_000;
    const percent = staked / total;
    const expected = ((150 * 0.00203928) / 1000) * percent;
    expect(getDynamicTokenValuation(solPrice, staked, total)).toBeCloseTo(
      expected,
      8
    );
  });

  it("returns 0 when 0 tokens are staked", () => {
    expect(getDynamicTokenValuation(150, 0, 1_000_000_000)).toBe(0);
  });

  it("returns full fair price when 100% of tokens are staked", () => {
    const solPrice = 200;
    const staked = 1_000_000_000;
    const total = 1_000_000_000;
    const expected = (solPrice * 0.00203928) / 1000;
    expect(getDynamicTokenValuation(solPrice, staked, total)).toBeCloseTo(
      expected,
      8
    );
  });

  it("returns lower price for small % staked", () => {
    const solPrice = 100;
    const staked = 10_000_000;
    const total = 1_000_000_000;
    const expected = ((100 * 0.00203928) / 1000) * (staked / total);
    expect(getDynamicTokenValuation(solPrice, staked, total)).toBeCloseTo(
      expected,
      10
    );
  });

  it("supports custom tokensPerEntry", () => {
    const solPrice = 100;
    const staked = 500_000_000;
    const total = 1_000_000_000;
    const tokensPerEntry = 500;
    const expected = ((100 * 0.00203928) / tokensPerEntry) * (staked / total);
    expect(
      getDynamicTokenValuation(solPrice, staked, total, tokensPerEntry)
    ).toBeCloseTo(expected, 10);
  });

  it("handles very large SOL prices", () => {
    const solPrice = 1_000_000;
    const staked = 500_000_000;
    const total = 1_000_000_000;
    const expected = ((solPrice * 0.00203928) / 1000) * 0.5;
    expect(getDynamicTokenValuation(solPrice, staked, total)).toBeCloseTo(
      expected,
      4
    );
  });

  it("returns NaN when totalSupply is 0", () => {
    const result = getDynamicTokenValuation(100, 1_000_000, 0);
    expect(result).toBeNaN(); // totalSupply cannot be 0
  });

  it("handles fractional token entry values correctly", () => {
    const solPrice = 123.456;
    const staked = 123_456_789;
    const total = 987_654_321;
    const percent = staked / total;
    const expected = ((solPrice * 0.00203928) / 1000) * percent;
    expect(getDynamicTokenValuation(solPrice, staked, total)).toBeCloseTo(
      expected,
      10
    );
  });
});
