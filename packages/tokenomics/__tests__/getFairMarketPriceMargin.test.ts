import { getFairMarketPriceMargin } from "../getFairMarketPriceMargin";

describe(getFairMarketPriceMargin, () => {
  it("returns correct margin when market price equals fair price", () => {
    const solPrice = 150;
    const tokensPerEntry = 1000;
    const reclaimedSolPerEntry = 0.00203928;
    const fairPrice = (solPrice * reclaimedSolPerEntry) / tokensPerEntry;

    const result = getFairMarketPriceMargin(
      solPrice,
      fairPrice,
      tokensPerEntry,
      reclaimedSolPerEntry
    );

    expect(result.fairPrice).toBeCloseTo(fairPrice, 8);
    expect(result.absoluteMargin).toBeCloseTo(0, 8);
    expect(result.percentageMargin).toBeCloseTo(0, 8);
  });

  it("returns positive margin when market price is higher than fair price", () => {
    const solPrice = 150;
    const marketPrice = 0.0005;
    const result = getFairMarketPriceMargin(solPrice, marketPrice);

    expect(result.fairPrice).toBeLessThan(result.actualMarketPrice);
    expect(result.absoluteMargin).toBeGreaterThan(0);
    expect(result.percentageMargin).toBeGreaterThan(0);
  });

  it("returns negative margin when market price is lower than fair price", () => {
    const solPrice = 150;
    const marketPrice = 0.0001;
    const result = getFairMarketPriceMargin(solPrice, marketPrice);

    expect(result.fairPrice).toBeGreaterThan(result.actualMarketPrice);
    expect(result.absoluteMargin).toBeLessThan(0);
    expect(result.percentageMargin).toBeLessThan(0);
  });

  it("returns 0 fair price if SOL is 0", () => {
    const solPrice = 0;
    const marketPrice = 0.0005;
    const result = getFairMarketPriceMargin(solPrice, marketPrice);

    expect(result.fairPrice).toBe(0);
    expect(result.absoluteMargin).toBe(marketPrice);
    expect(result.percentageMargin).toBe(Infinity); // division by zero
  });

  it("handles custom tokensPerEntry and reclaimedSolPerEntry", () => {
    const solPrice = 200;
    const marketPrice = 0.0008;
    const tokensPerEntry = 500;
    const reclaimedSolPerEntry = 0.003;
    const expectedFairPrice =
      (solPrice * reclaimedSolPerEntry) / tokensPerEntry;

    const result = getFairMarketPriceMargin(
      solPrice,
      marketPrice,
      tokensPerEntry,
      reclaimedSolPerEntry
    );

    expect(result.fairPrice).toBeCloseTo(expectedFairPrice, 8);
    expect(result.actualMarketPrice).toBe(marketPrice);
  });
});
