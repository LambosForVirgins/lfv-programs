import { getExchangedRewardValue } from "../getExchangedRewardValue";

describe(getExchangedRewardValue, () => {
  it("returns correct value for default reclaimed SOL amount", () => {
    const solPrice = 150;
    const expected = 150 * 0.00203928;
    expect(getExchangedRewardValue(solPrice)).toBeCloseTo(expected, 6);
  });

  it("returns 0 if SOL price is 0", () => {
    expect(getExchangedRewardValue(0)).toBe(0);
  });

  it("supports custom reclaimed SOL values", () => {
    const solPrice = 100;
    const customReclaimed = 0.003;
    const expected = solPrice * customReclaimed;
    expect(getExchangedRewardValue(solPrice, customReclaimed)).toBeCloseTo(
      expected,
      6
    );
  });
});
