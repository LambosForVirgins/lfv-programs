import { BN } from "bn.js";
import { findDrawAccount } from "../findDrawAccountAddress";
import { PublicKey } from "@solana/web3.js";

const mockGiveawayId = new BN(1),
  mockDrawNo = new BN(1),
  expectedAddress = new PublicKey(
    "6vWuqhrCbTunp3guzp1gMkJ8JNKdMveGX9PrbuoNfdaS"
  );

describe("findDrawAccount", () => {
  it("should return the address of the draw account", () => {
    const result = findDrawAccount(mockGiveawayId, mockDrawNo);
    expect(result.equals(expectedAddress)).toBe(true);
  });
});
