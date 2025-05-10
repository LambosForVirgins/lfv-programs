import { findSubscriptionAccountAddress, findVaultAccountAddress } from "@/pda";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { program } from "../../client/constants";
import { Logger } from "@/tools/Logger";

export const initializeSubscriptionAccount = async (signer: Keypair) => {
  try {
    const subscriptionAccount = findSubscriptionAccountAddress(signer);
    // Initialize member account
    const initTransaction = await program.methods
      .initialize()
      .accounts({
        subscription: subscriptionAccount,
        signer: signer.publicKey,
        systemProgram: SystemProgram.programId, // TODO: Remove this
      })
      .signers([signer])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(
      initTransaction,
      "confirmed"
    );
    Logger.success("Initialized subscription", subscriptionAccount.toBase58());
  } catch (error) {
    console.error("Initialize", error);
  }
};
