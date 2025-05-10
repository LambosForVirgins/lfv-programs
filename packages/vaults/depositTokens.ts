import BN from "bn.js";
import { MINT_ADDRESS, MINT_DECIMALS, program } from "../../client/constants";
import { findSubscriptionAccountAddress, findVaultAccountAddress } from "@/pda";
import {
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Logger } from "@/tools/Logger";

export const depositTokens = async (amount: number, signer: Keypair) => {
  const decimalFactor = new BN(Math.pow(10, 9));
  const amountToAdd = new BN(amount).mul(decimalFactor); // TODO: Make this a utility function with the decimalFactor

  try {
    const subscriptionAccount = findSubscriptionAccountAddress(signer),
      vaultAccount = findVaultAccountAddress(MINT_ADDRESS, signer);
    // Token accounts
    const destinationTokenAccount = getAssociatedTokenAddressSync(
      MINT_ADDRESS,
      signer.publicKey
    );

    const transaction = await program.methods
      .deposit(amountToAdd)
      .accounts({
        subscription: subscriptionAccount,
        vaultTokenAccount: vaultAccount,
        sourceTokenAccount: destinationTokenAccount,
        mint: MINT_ADDRESS,
        signer: signer.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([signer])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(transaction);
    Logger.success("Deposited bond", amount.toString());
  } catch (err) {
    console.error("Deposit", err);
  }
};
