import BN from "bn.js";
import { MINT_ADDRESS, program } from "../../client/constants";
import { findSubscriptionAccountAddress, findVaultAccountAddress } from "@/pda";
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

const decimalFactor = new BN(Math.pow(10, 9));

export const releaseTokens = async (amount: number, signer: Keypair) => {
  const withdrawAmount = new BN(amount).mul(decimalFactor); // TODO: Make this a utility function with the decimalFactor

  try {
    const subscriptionAccount = findSubscriptionAccountAddress(signer),
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

    const releaseTransaction = await program.methods
      .release(withdrawAmount)
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
    await program.provider.connection.confirmTransaction(releaseTransaction);
    console.log("Release", amount, "tx", releaseTransaction);
  } catch (err) {
    console.error("Failed with error", err);
  }
};
