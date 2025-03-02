import { Signer, PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { Seed, program } from "../../client/constants";

/**
 * Derives the unique ticket account address for the
 * ticket owners entries into the giveaway draw.
 *
 * ```ts
 * seeds = [ TICKET_SEED + giveaway_id + draw_no + owner ]
 * ```
 *
 * @param giveaway_id Unique giveaway identifier
 * @param draw_no Draw number for the giveaway
 * @param owner Owner of the ticket
 * @returns Program derived ticket account address
 */
export const findTicketAccountAddress = (
  giveaway_id: BN,
  draw_no: BN,
  owner: PublicKey
) => {
  const [pda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(Seed.TicketAccount),
      giveaway_id.toArrayLike(Buffer, "le", 8),
      draw_no.toArrayLike(Buffer, "le", 8),
      owner.toBuffer(),
    ],
    program.programId
  );

  return pda;
};
