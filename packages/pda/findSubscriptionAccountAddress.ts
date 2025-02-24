import { Keypair, PublicKey } from "@solana/web3.js";
import { Seed, program } from "../../client/constants";

export const findSubscriptionAccountAddress = (signer: Keypair) => {
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from(Seed.SubscriptionAccount), signer.publicKey.toBuffer()],
    program.programId
  );

  return pda;
};
