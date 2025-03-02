import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  PublicKey,
  Keypair,
  TransactionInstruction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import BN from "bn.js";
import { program } from "../../client/constants";

export const getUpdateTicketInstruction = (
  draw: PublicKey,
  ticket: PublicKey,
  entryAccount: PublicKey,
  entries: BN,
  owner: PublicKey
): Promise<TransactionInstruction> => {
  const entryTokenAccount = getAssociatedTokenAddressSync(entryAccount, owner);

  return program.methods
    .updateTicket(entries)
    .accounts({
      draw,
      entries: entryAccount,
      entryTokenAccount,
      ticket,
      signer: owner,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId, // TODO: Remove this
      rent: SYSVAR_RENT_PUBKEY, // TODO: I Think we can remove this too?
    })
    .instruction();
};
