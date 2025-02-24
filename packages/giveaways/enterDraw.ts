import { findDrawAccount, findTicketAccountAddress } from "@/pda";
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Connection,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import BN from "bn.js";
import { ENTRY_MINT_ADDRESS, program } from "../../client/constants";

export const enterDraw = async (
  connection: Connection,
  giveawayId: BN,
  drawNo: BN,
  entries: number = 1,
  signer: Keypair
) => {
  const entryTokenAccount = getAssociatedTokenAddressSync(
    ENTRY_MINT_ADDRESS,
    signer.publicKey
  );
  const ticket = findTicketAccountAddress(giveawayId, drawNo, signer);
  const draw = findDrawAccount(giveawayId, drawNo);

  try {
    const enterDrawTransaction = await program.methods
      .enter(new BN(entries))
      .accounts({
        draw,
        entries: ENTRY_MINT_ADDRESS,
        entryTokenAccount,
        ticket,
        signer: signer.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId, // TODO: Remove this
        rent: SYSVAR_RENT_PUBKEY, // TODO: I Think we can remove this too?
      })
      .signers([signer])
      .rpc();
    // Confirm transaction
    await connection.confirmTransaction(enterDrawTransaction);
    console.log("Entered draw:", draw.toBase58());
    console.log("Ticket", ticket.toBase58());
  } catch (err) {
    console.error("Enter error", err);
  }
};
