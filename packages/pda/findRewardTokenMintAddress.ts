import { PublicKey } from "@solana/web3.js";
import { program, Seed } from "../../client/constants";

export const findRewardTokenMintAddress = () => {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from(Seed.RewardMint)],
    program.programId
  );

  return pda;
};
