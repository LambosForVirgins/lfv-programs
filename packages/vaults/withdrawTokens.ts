import { findSubscriptionAccountAddress, findVaultAccountAddress } from "@/pda";
import { MINT_ADDRESS, program } from "../../client/constants";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

export const withdrawTokens = async (signer: Keypair) => {
  try {
    const subscriptionAccount = findSubscriptionAccountAddress(
        signer.publicKey
      ),
      vaultAccount = findVaultAccountAddress(MINT_ADDRESS, signer);
    // Token accounts
    const vaultTokenAccount = getAssociatedTokenAddressSync(
        MINT_ADDRESS,
        vaultAccount
      ),
      destinationTokenAccount = getAssociatedTokenAddressSync(
        MINT_ADDRESS,
        signer.publicKey
      );

    const withdrawTransaction = await program.methods
      .withdraw()
      .accounts({
        subscription: subscriptionAccount,
        vaultTokenAccount: vaultTokenAccount,
        sourceTokenAccount: destinationTokenAccount,
        mint: MINT_ADDRESS,
        signer: signer.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(withdrawTransaction);
    console.log("Withdraw tx", withdrawTransaction);
  } catch (err) {
    console.error("Failed with error", err);
  }
};
