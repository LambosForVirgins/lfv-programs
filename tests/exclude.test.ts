import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import spl from "@solana/spl-token";
import { program } from "../client/constants";
import { findSubscriptionAccountAddress, findVaultAccountAddress } from "@/pda";
import { assertErrorAsync, mintTokenToAccount } from "@/testing/utils";
const DECIMALS = 9;
const AMOUNT = 100;

describe("Member self exclusion", async () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const wallets = {
    app: new web3.Keypair(),
    mint: new web3.Keypair(),
    gift: new web3.Keypair(),
  };

  const memberWallet = new web3.Keypair(),
    adminWallet = wallets.app,
    tokenMint = wallets.mint;

  const amount = new BN(AMOUNT * Math.pow(10, DECIMALS));

  beforeAll(async () => {
    await mintTokenToAccount(amount, memberWallet.publicKey, adminWallet);
    // Generate member program account keys
    const subscriptionAccount = findSubscriptionAccountAddress(memberWallet);
    const vaultTokenAccount = findVaultAccountAddress(
      tokenMint.publicKey,
      memberWallet
    );
    // Send transaction
    const txHash = await program.methods
      .initialize()
      .accounts({
        subscription: subscriptionAccount,
        vaultTokenAccount,
        mint: tokenMint.publicKey,
        signer: memberWallet.publicKey,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([memberWallet])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(txHash);
    // Fetch the associated token account
    const associatedTokenAccount = await spl.getAccount(
      program.provider.connection,
      vaultTokenAccount
    );
  });

  describe("Self exclusion", () => {
    let memberAccount: web3.PublicKey;

    beforeAll(async () => {
      memberAccount = findSubscriptionAccountAddress(memberWallet);
    });

    it("fails to exclude member when not signer", async () => {
      console.log(memberAccount.toBase58());
      await assertErrorAsync(
        program.methods
          .exclude()
          .accounts({
            memberAccount,
            signer: adminWallet.publicKey,
          })
          .signers([adminWallet])
          .rpc(),
        "Forbidden exclusion signer"
      );
    });

    it("updates the member status when signer", async () => {
      const txHash = await program.methods
        .exclude()
        .accounts({
          memberAccount,
          signer: memberWallet.publicKey,
        })
        .signers([memberWallet])
        .rpc();
      // Confirm transaction
      await program.provider.connection.confirmTransaction(txHash);

      assert(txHash);
    });

    it.skip("fails to update status when excluded");
  });
});
