import { getGeneratedEntryTokens } from "../getGeneratedEntryTokens";

describe(getGeneratedEntryTokens, () => {
  it("calculates ENTRY tokens correctly", () => {
    expect(getGeneratedEntryTokens(250_000_000)).toBe(250000);
  });

  it("returns 0 if fewer tokens than threshold", () => {
    expect(getGeneratedEntryTokens(999)).toBe(0);
  });

  it("respects custom tokensPerEntry", () => {
    expect(getGeneratedEntryTokens(250_000_000, 500)).toBe(500000);
  });
});
