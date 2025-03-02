import { findSubscriptionAccountAddress, findVaultAccountAddress } from "@/pda";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { MINT_ADDRESS, program } from "../../client/constants";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Logger } from "@/tools/Logger";

export const initializeSubscriptionAccount = async (signer: Keypair) => {
  try {
    const subscriptionAccount = findSubscriptionAccountAddress(
        signer.publicKey
      ),
      vaultAccount = findVaultAccountAddress(MINT_ADDRESS, signer);
    // Initialize member account
    const initTransaction = await program.methods
      .initialize()
      .accounts({
        subscription: subscriptionAccount,
        vaultTokenAccount: vaultAccount,
        mint: MINT_ADDRESS,
        signer: signer.publicKey,
        systemProgram: SystemProgram.programId, // TODO: Remove this
        tokenProgram: TOKEN_PROGRAM_ID,
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
