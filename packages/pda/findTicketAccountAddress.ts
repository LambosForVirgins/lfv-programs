import { Signer, PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { Seed, program } from "../../client/constants";

export const findTicketAccountAddress = (
  giveaway_id: BN,
  draw_no: BN,
  signer: Signer
) => {
  const [pda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(Seed.TicketAccount),
      giveaway_id.toArrayLike(Buffer, "le", 8),
      draw_no.toArrayLike(Buffer, "le", 8),
      signer.publicKey.toBuffer(),
    ],
    program.programId
  );

  return pda;
};
