import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { program, Seed } from "../../client/constants";

export const findDrawAccount = (giveaway_id: BN, draw_no: BN) => {
  const [pda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(Seed.DrawAccount),
      giveaway_id.toArrayLike(Buffer, "le", 8),
      draw_no.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  return pda;
};
