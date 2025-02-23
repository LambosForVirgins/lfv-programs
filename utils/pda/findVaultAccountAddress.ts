import { PublicKey, Keypair } from "@solana/web3.js";
import { Seed, program } from "../../client/constants";

export const findVaultAccountAddress = (
  mintKey: PublicKey,
  signer: Keypair
) => {
  const [pda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(Seed.VaultAccount),
      mintKey.toBuffer(),
      signer.publicKey.toBuffer(),
    ],
    program.programId
  );

  return pda;
};
