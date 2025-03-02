import { findDrawAccount, findTicketAccountAddress } from "@/pda";
import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import BN from "bn.js";
import { Logger } from "@/tools/Logger";
import { getEnterDrawInstruction } from "./getEnterDrawInstruction";
import { getUpdateTicketInstruction } from "./getUpdateTicketInstruction";
import { checkTicketExists } from "./checkTicketExists";

export const enterDraw = async (
  connection: Connection,
  giveawayId: BN,
  drawNo: BN,
  entryAccount: PublicKey,
  entries: number = 1,
  signer: Keypair
) => {
  const ticket = findTicketAccountAddress(giveawayId, drawNo, signer.publicKey);
  const draw = findDrawAccount(giveawayId, drawNo);
  const transaction = new Transaction();

  Logger.info("Enter draw", draw.toBase58());

  if (await checkTicketExists(connection, ticket)) {
    transaction.add(
      await getUpdateTicketInstruction(
        draw,
        ticket,
        entryAccount,
        new BN(entries),
        signer.publicKey
      )
    );
  } else {
    transaction.add(
      await getEnterDrawInstruction(
        draw,
        ticket,
        entryAccount,
        new BN(entries),
        signer.publicKey
      )
    );
  }

  try {
    const txHash = await connection.sendTransaction(transaction, [signer]);
    // Confirm transaction
    await connection.confirmTransaction(txHash, "confirmed");
    console.log("Entered draw:", draw.toBase58());
    console.log("Ticket", ticket.toBase58());
  } catch (err) {
    console.error("Enter error", err);
  }
};
