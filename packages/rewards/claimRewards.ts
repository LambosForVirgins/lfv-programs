import { Connection, Keypair, SystemProgram } from "@solana/web3.js";
import { ENTRY_MINT_ADDRESS, program } from "../../client/constants";
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { findSubscriptionAccountAddress } from "@/pda";

export const claimRewards = async (connection: Connection, signer: Keypair) => {
  try {
    const subscriptionAccount = findSubscriptionAccountAddress(signer);

    const rewardTokenAccount = getAssociatedTokenAddressSync(
      ENTRY_MINT_ADDRESS,
      signer.publicKey
    );

    const claimTransaction = await program.methods
      .claim()
      .accounts({
        subscription: subscriptionAccount,
        mint: ENTRY_MINT_ADDRESS,
        destinationTokenAccount: rewardTokenAccount,
        signer: signer.publicKey,
        systemProgram: SystemProgram.programId, // TODO: Remove this
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();
    // Confirm transaction
    await connection.confirmTransaction(claimTransaction);
    console.log("Claimed:", claimTransaction);
  } catch (err) {
    console.error("Claim", err);
  }
};
