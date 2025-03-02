import { findDrawAccount } from "@/pda";
import { program } from "../../client/constants";
import BN from "bn.js";
import { Connection, Keypair, SystemProgram } from "@solana/web3.js";

export const openDraw = async (
  connection: Connection,
  giveawayId: BN,
  drawNo: BN,
  signer: Keypair
) => {
  try {
    const draw = findDrawAccount(giveawayId, drawNo);

    console.log(
      "Open draw",
      drawNo.toNumber(),
      "giveaway",
      giveawayId.toNumber()
    );

    const enterDrawTransaction = await program.methods
      .openDraw(giveawayId, drawNo)
      .accounts({
        draw,
        signer: signer.publicKey,
        systemProgram: SystemProgram.programId, // TODO: Remove this
      })
      .signers([signer])
      .rpc();
    // Confirm transaction
    await connection.confirmTransaction(enterDrawTransaction);
    console.log("Draw account", draw.toBase58());
  } catch (err) {
    console.error("Open draw error", err);
  }
};
