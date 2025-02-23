import { PublicKey } from "@solana/web3.js";
import { program, Seed } from "../../client/constants";

export const findRewardTokenMintAddress = () => {
  // Derive the PDA for the mint
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from(Seed.RewardTokenMint)],
    program.programId
  );

  return pda;
};
