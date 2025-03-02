import { Keypair, PublicKey } from "@solana/web3.js";
import { Seed, program } from "../../client/constants";

export const findSubscriptionAccountAddress = (owner: PublicKey) => {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from(Seed.SubscriptionAccount), owner.toBuffer()],
    program.programId
  );

  return pda;
};
