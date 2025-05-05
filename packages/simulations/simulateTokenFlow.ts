interface TokenFlowOptions {
  stakedTokens: number;
  tokensPerEntry?: number;
  monthlyBurnRate?: number;
}

interface TokenFlowOutput {
  month: number;
  generated: number;
  burned: number;
  net: number;
  totalAccumulated: number;
}

export class TokenFlowSimulator {
  private stakedTokens: number;
  private tokensPerEntry: number = 1000;
  private monthlyBurnRate: number = 0;
  private history: TokenFlowOutput[] = [];

  constructor({
    stakedTokens,
    tokensPerEntry = 1000,
    monthlyBurnRate = 0,
  }: TokenFlowOptions) {
    this.stakedTokens = stakedTokens;
    this.tokensPerEntry = tokensPerEntry;
    this.monthlyBurnRate = monthlyBurnRate;
  }

  public run(iterations: number = 12): TokenFlowOutput[] {
    const entriesPerMonth = Math.floor(this.stakedTokens / this.tokensPerEntry);
    let totalAccumulated = 0;
    this.history = [];

    for (let month = 1; month <= iterations; month++) {
      const generated = entriesPerMonth;
      const burned = Math.min(
        this.monthlyBurnRate,
        totalAccumulated + generated
      );
      const net = generated - burned;
      totalAccumulated += net;

      this.history.push({
        month,
        generated,
        burned,
        net,
        totalAccumulated,
      });
    }

    return this.history;
  }

  public getHistory(): TokenFlowOutput[] {
    return this.history;
  }

  public reset(): void {
    this.history = [];
  }

  public updateConfig({
    stakedTokens,
    tokensPerEntry,
    monthlyBurnRate,
  }: Partial<TokenFlowOptions>): void {
    if (stakedTokens !== undefined) this.stakedTokens = stakedTokens;
    if (tokensPerEntry !== undefined) this.tokensPerEntry = tokensPerEntry;
    if (monthlyBurnRate !== undefined) this.monthlyBurnRate = monthlyBurnRate;
  }
}
