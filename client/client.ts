import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as spl from "@solana/spl-token";
import { appWallet, MINT_ADDRESS, program } from "./constants";
import {
  findSubscriptionAccountAddress,
  findRewardTokenMintAddress,
  findVaultAccountAddress,
  findMetaplexAddress,
} from "@/pda";
import { loadKeypairFileSync, sleep } from "@/testing/utils";
import { initializeSubscriptionAccount } from "@/subscriptions";
import { claimRewards } from "@/rewards";
import { depositTokens } from "@/vaults/depositTokens";
import { enterDraw } from "@/giveaways/enterDraw";
import { fundMemberWallet } from "@/tools/fundMemberWallet";
import { isMintInitialized } from "@/tools/isMintInitialized";
import { Logger } from "@/tools/Logger";

// Globals
const MINT_BALANCE = 15_050_542;

const memberWallet = new web3.Keypair(),
  associatedTokenProgram = spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  decimalFactor = new BN(Math.pow(10, 9)),
  amount = new BN(MINT_BALANCE).mul(decimalFactor),
  mint = MINT_ADDRESS,
  giveawayId = new BN(1),
  drawNo = new BN(11),
  rewardMint = findRewardTokenMintAddress(),
  subscriptionAccount = findSubscriptionAccountAddress(memberWallet),
  vaultTokenAccount = findVaultAccountAddress(mint, memberWallet);

(async () => {
  Logger.info("Provider RPC endpoint", program.provider.connection.rpcEndpoint);

  if (await isMintInitialized(program.provider.connection, rewardMint)) {
    Logger.success(`Found mint account`, rewardMint.toBase58());
  } else {
    Logger.warn(`Mint not found, attempting to create`, rewardMint.toBase58());
  }

  // try {
  //   const payer = appWallet,
  //     mintAuthority = appWallet,
  //     lamports = await spl.getMinimumBalanceForRentExemptMint(
  //       program.provider.connection
  //     ),
  //     decimals = 4,
  //     transaction = new web3.Transaction();
  //   // Allocate the mint account as PDA
  //   transaction.add(
  //     web3.SystemProgram.createAccount({
  //       fromPubkey: payer.publicKey,
  //       newAccountPubkey: rewardMint,
  //       space: spl.MINT_SIZE,
  //       lamports,
  //       programId: spl.TOKEN_PROGRAM_ID,
  //     })
  //   );
  //   // Initialize the mint with Token2022 program
  //   transaction.add(
  //     spl.createInitializeMintInstruction(
  //       rewardMint,
  //       decimals,
  //       mintAuthority.publicKey,
  //       null,
  //       spl.TOKEN_PROGRAM_ID
  //     )
  //   );

  //   await web3.sendAndConfirmTransaction(
  //     program.provider.connection,
  //     transaction,
  //     [payer],
  //     {
  //       commitment: "confirmed",
  //     }
  //   );

  //   console.log("Mint initialized with PDA at:", rewardMint.toBase58());
  // } catch (error) {
  //   console.error(error);
  // }

  const sourceTokenAccount = await spl.createAssociatedTokenAccount(
    program.provider.connection,
    appWallet,
    mint,
    memberWallet.publicKey,
    { commitment: "confirmed" },
    spl.TOKEN_PROGRAM_ID,
    associatedTokenProgram
  );

  const rewardsTokenAccount = await spl.createAssociatedTokenAccount(
    program.provider.connection,
    appWallet,
    rewardMint,
    memberWallet.publicKey,
    { commitment: "confirmed" },
    spl.TOKEN_PROGRAM_ID,
    associatedTokenProgram
  );

  Logger.success("Created mint token account", sourceTokenAccount.toBase58());
  Logger.success(
    "Created rewards token account",
    rewardsTokenAccount.toBase58()
  );

  const transactionQueue = [
    () =>
      new Promise((resolve) =>
        fundMemberWallet(MINT_BALANCE, memberWallet.publicKey, appWallet).then(
          resolve
        )
      ),
    () =>
      new Promise((resolve) =>
        initializeSubscriptionAccount(memberWallet).then(resolve)
      ),
    // // () => new Promise((resolve) => selfExclude().then(resolve)),
    () =>
      new Promise((resolve) => depositTokens(1000, memberWallet).then(resolve)),
    // () =>
    //   new Promise((resolve) =>
    //     depositTokens(1500, memberWallet).then(resolve)
    //   ),
    // () =>
    //   new Promise((resolve) =>
    //     claimRewards(program.provider.connection, memberWallet).then(resolve)
    //   ),
    // // () => new Promise((resolve) => openDraw(1000).then(resolve)),
    // () =>
    //   new Promise((resolve) =>
    //     enterDraw(
    //       program.provider.connection,
    //       giveawayId,
    //       drawNo,
    //       1000,
    //       memberWallet
    //     ).then(resolve)
    //   ),
    // () => new Promise((resolve) => releaseTokens(500).then(resolve)),
    // () => new Promise((resolve) => withdrawTokens().then(resolve)),
    // () => new Promise((resolve) => claimRewards(3500).then(resolve)),
    // () => new Promise((resolve) => depositTokens(123_458, 10000).then(resolve)), // Super chad member
    // () => new Promise((resolve) => depositTokens(2_007_824).then(resolve)), // Mega chad member
    // () => new Promise((resolve) => depositTokens(3_720_345).then(resolve)), // Giga chad member
  ];

  // @ts-ignore
  await transactionQueue.reduce((current, next) => {
    return current.then(() => next());
  }, Promise.resolve());

  const subscriptionState =
    await program.account.subscriptionAccount.fetch(subscriptionAccount);
  const vaultAccount = await spl.getAccount(
    program.provider.connection,
    vaultTokenAccount,
    "confirmed",
    spl.TOKEN_PROGRAM_ID
  );
  const rewardsAccount = await spl.getAccount(
    program.provider.connection,
    rewardsTokenAccount,
    "confirmed",
    spl.TOKEN_PROGRAM_ID
  );
  const sourceAccount = await spl.getAccount(
    program.provider.connection,
    sourceTokenAccount,
    "confirmed",
    spl.TOKEN_PROGRAM_ID
  );

  console.log("----------- ADDRESSES -------------");
  console.log("Token Mint        :", mint.toBase58());
  console.log("  | Source account:", sourceTokenAccount.toBase58());
  console.log("     > Original   :", amount.div(decimalFactor).toString());
  console.log("     > Remaining  :", sourceAccount.amount);
  console.log("Entry Mint        :", rewardMint.toBase58());
  console.log("  | Source account:", rewardsTokenAccount.toBase58());
  console.log(
    "     > Balance    :",
    Number(rewardsAccount.amount) / Math.pow(10, 4)
  );
  console.log("Program           :", program.programId.toBase58());
  console.log("Admin             :", appWallet.publicKey.toBase58());
  console.log("Subscription      :", memberWallet.publicKey.toBase58());
  console.log("  | Member state  :", subscriptionAccount.toBase58());
  console.log("     > Status     :", subscriptionState.status);
  console.log("     > Tier       :", subscriptionState.tier);
  console.log(
    "     > Locked     :",
    subscriptionState.totalAmount.div(decimalFactor).toString()
  );
  console.log(
    "     > Matured    :",
    subscriptionState.totalMatured.div(decimalFactor).toString()
  );
  console.log(
    "     > Released   :",
    subscriptionState.totalReleased.div(decimalFactor).toString()
  );
  console.log("     > Available  :", "Not set");
  console.log("     > Unclaimed  :", subscriptionState.totalRewards.toNumber());
  console.log(
    "     > Entries    :",
    Number(rewardsAccount.amount) / Math.pow(10, 4)
  );
  console.log("     > Slots      :", subscriptionState.slots.length);
  console.log("  | Vault account :", vaultTokenAccount.toBase58());
  console.log("     > Amount     :", vaultAccount.amount);
  console.log("-----------------------------------");
})();
