import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as spl from "@solana/spl-token";
import { appWallet, MINT_ADDRESS, program } from "./constants";
import {
  findSubscriptionAccountAddress,
  findRewardMintAddress,
  findVaultAccountAddress,
  findDrawAccount,
  findTicketAccountAddress,
} from "@/pda";
import { lamportsToNumber, sleep } from "@/testing/utils";
import { initializeSubscriptionAccount } from "@/subscriptions";
import { claimRewards } from "@/rewards";
import { depositTokens } from "@/vaults/depositTokens";
import { enterDraw } from "@/giveaways/enterDraw";
import { fundMemberWallet } from "@/tools/fundMemberWallet";
import { isMintInitialized } from "@/tools/isMintInitialized";
import { Logger } from "@/tools/Logger";
import { openDraw } from "@/giveaways/openDraw";
import { findAllTickets } from "@/giveaways/findAllTickets";

// Globals
const MINT_BALANCE = 500_000;

const memberWallet = new web3.Keypair(),
  associatedTokenProgram = spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  decimalFactor = new BN(Math.pow(10, 9)),
  initialMintBalance = new BN(MINT_BALANCE).mul(decimalFactor),
  mint = MINT_ADDRESS,
  rewardMint = findRewardMintAddress(),
  subscriptionAccount = findSubscriptionAccountAddress(memberWallet.publicKey),
  vaultTokenAccount = findVaultAccountAddress(mint, memberWallet);

(async () => {
  Logger.info("Provider RPC endpoint", program.provider.connection.rpcEndpoint);

  console.log("Secret", memberWallet.secretKey.toString());

  if (await isMintInitialized(program.provider.connection, rewardMint)) {
    Logger.success(`Found reward mint account`, rewardMint.toBase58());
  } else {
    Logger.warn(
      `Reward mint not found, attempting to create`,
      rewardMint.toBase58()
    );
  }

  const sourceTokenAccount = await spl.getOrCreateAssociatedTokenAccount(
    program.provider.connection,
    appWallet,
    mint,
    memberWallet.publicKey,
    false,
    "confirmed",
    { commitment: "confirmed" },
    spl.TOKEN_PROGRAM_ID,
    associatedTokenProgram
  );

  const rewardsTokenAccount = await spl.getOrCreateAssociatedTokenAccount(
    program.provider.connection,
    appWallet,
    rewardMint,
    memberWallet.publicKey,
    false,
    "confirmed",
    { commitment: "confirmed" },
    spl.TOKEN_PROGRAM_ID,
    associatedTokenProgram
  );

  Logger.success(
    "Created mint token account",
    sourceTokenAccount.address.toBase58()
  );
  Logger.success(
    "Created rewards token account",
    rewardsTokenAccount.address.toBase58()
  );

  const giveawayId = new BN(1),
    drawNo = new BN(14);

  const transactionQueue = [
    () =>
      new Promise((resolve) =>
        fundMemberWallet(
          initialMintBalance,
          memberWallet.publicKey,
          appWallet
        ).then(resolve)
      ),
    () =>
      new Promise((resolve) =>
        initializeSubscriptionAccount(memberWallet).then(resolve)
      ),
    // // () => new Promise((resolve) => selfExclude().then(resolve)),
    () =>
      new Promise((resolve) => depositTokens(6000, memberWallet).then(resolve)),
    () =>
      new Promise((resolve) => depositTokens(5000, memberWallet).then(resolve)),
    () => new Promise((resolve) => sleep(1000).then(resolve)),
    () =>
      new Promise((resolve) =>
        claimRewards(
          program.provider.connection,
          memberWallet,
          rewardsTokenAccount.address
        ).then(resolve)
      ),
    // () =>
    //   new Promise((resolve) =>
    //     openDraw(
    //       program.provider.connection,
    //       giveawayId,
    //       drawNo,
    //       appWallet
    //     ).then(resolve)
    //   ),
    () =>
      new Promise((resolve) =>
        enterDraw(
          program.provider.connection,
          giveawayId,
          drawNo,
          rewardMint,
          1,
          memberWallet
        ).then(resolve)
      ),
    () =>
      new Promise((resolve) =>
        enterDraw(
          program.provider.connection,
          giveawayId,
          drawNo,
          rewardMint,
          3,
          memberWallet
        ).then(resolve)
      ),
    () =>
      new Promise((resolve) =>
        enterDraw(
          program.provider.connection,
          giveawayId,
          drawNo,
          rewardMint,
          1,
          memberWallet
        ).then(resolve)
      ),
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
    rewardsTokenAccount.address,
    "confirmed",
    spl.TOKEN_PROGRAM_ID
  );
  const sourceAccount = await spl.getAccount(
    program.provider.connection,
    sourceTokenAccount.address,
    "confirmed",
    spl.TOKEN_PROGRAM_ID
  );
  const drawAccount = findDrawAccount(giveawayId, drawNo);
  const drawState = await program.account.drawAccount.fetch(drawAccount);
  const ticketAddress = findTicketAccountAddress(
    giveawayId,
    drawNo,
    memberWallet.publicKey
  );
  const ticketState = await program.account.ticketAccount.fetch(ticketAddress);
  const allTickets = await findAllTickets(memberWallet.publicKey);

  console.log("----------- ADDRESSES -------------");
  console.log("Token Mint        :", mint.toBase58());
  console.log("  | Source account:", sourceTokenAccount.address.toBase58());
  console.log(
    "     > Original   :",
    initialMintBalance.div(decimalFactor).toString()
  );
  console.log("     > Remaining  :", lamportsToNumber(sourceAccount.amount));
  console.log("Entry Mint        :", rewardMint.toBase58());
  console.log("  | Source account:", rewardsTokenAccount.address.toBase58());
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
  console.log("     > Amount     :", lamportsToNumber(vaultAccount.amount));
  console.log("Draw              :", drawAccount.toBase58());
  console.log("   > Giveaway ID  :", drawState.giveawayId.toNumber());
  console.log("   > Draw ID      :", drawState.drawId.toNumber());
  console.log("   > Entries      :", drawState.totalEntries.toNumber());
  console.log("   > Status       :", drawState.status.toString());
  console.log("Ticket            :", ticketAddress.toBase58());
  console.log("   > Giveaway ID  :", ticketState.giveawayId.toNumber());
  console.log("   > Draw ID      :", ticketState.drawId.toNumber());
  console.log("   > Entries      :", ticketState.totalEntries.toNumber());
  console.log("Total Tickets     :", allTickets.length);
  console.log("-----------------------------------");
})();
