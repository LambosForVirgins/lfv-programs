import { TokenFlowSimulator } from "../simulateTokenFlow";

describe(TokenFlowSimulator, () => {
  it("simulates basic reward accumulation without burning", () => {
    const simulator = new TokenFlowSimulator({
      stakedTokens: 250_000_000,
    });

    const result = simulator.run(3);
    expect(result.length).toBe(3);
    result.forEach((month) => {
      expect(month.generated).toBe(250_000);
      expect(month.burned).toBe(0);
      expect(month.net).toBe(250_000);
    });

    expect(result[2].totalAccumulated).toBe(750_000);
  });

  it("applies fixed monthly burn rate", () => {
    const simulator = new TokenFlowSimulator({
      stakedTokens: 250_000_000,
      monthlyBurnRate: 100_000,
    });

    const result = simulator.run(2);
    expect(result[0].burned).toBe(100_000);
    expect(result[1].burned).toBe(100_000);
    expect(result[1].totalAccumulated).toBe(300_000); // 150k + 150k
  });

  it("does not burn more than accumulated", () => {
    const simulator = new TokenFlowSimulator({
      stakedTokens: 250_000_000,
      monthlyBurnRate: 1_000_000, // intentionally high
    });

    const result = simulator.run(1);
    expect(result[0].generated).toBe(250_000);
    expect(result[0].burned).toBe(250_000); // cannot burn more than available
    expect(result[0].totalAccumulated).toBe(0);
  });

  it("resets state correctly", () => {
    const simulator = new TokenFlowSimulator({
      stakedTokens: 250_000_000,
    });

    simulator.run(1);
    simulator.reset();
    expect(simulator.getHistory()).toEqual([]);
  });

  it("updates config and re-runs correctly", () => {
    const simulator = new TokenFlowSimulator({
      stakedTokens: 100_000_000,
    });

    let result = simulator.run(2);
    expect(result[0].generated).toBe(100_000);

    simulator.updateConfig({ stakedTokens: 200_000_000 });
    result = simulator.run();
    expect(result[0].generated).toBe(200_000);
  });

  it("uses default tokensPerEntry and burn rate if not specified", () => {
    const simulator = new TokenFlowSimulator({
      stakedTokens: 1_000_000,
    });

    const result = simulator.run(1);
    expect(result[0].generated).toBe(1000);
    expect(result[0].burned).toBe(0);
  });

  it("accumulates correctly over time with net changes", () => {
    const simulator = new TokenFlowSimulator({
      stakedTokens: 1_000_000,
      tokensPerEntry: 1000,
      monthlyBurnRate: 500,
    });

    const result = simulator.run(3);
    expect(result[0].totalAccumulated).toBe(500); // 1000 - 500
    expect(result[1].totalAccumulated).toBe(1000); // 500 + (1000 - 500)
    expect(result[2].totalAccumulated).toBe(1500); // etc
  });
});
