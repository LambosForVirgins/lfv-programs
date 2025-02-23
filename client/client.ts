import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import spl from "@solana/spl-token";
import { program } from "./constants";
import {
  findSubscriptionAccountAddress,
  findRewardTokenMintAddress,
  findVaultAccountAddress,
  findMetaplexAddress,
  findDrawAccount,
  findTicketAccountAddress,
} from "../utils/pda";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

// Get the provider from the client
const pg = anchor.getProvider();

// Globals
const DECIMALS = 9;
const MINT_BALANCE = 15_050_542;

const memberWallet = new web3.Keypair(),
  appWallet = pg.wallets.app.keypair,
  systemProgram = web3.SystemProgram.programId,
  tokenProgram = spl.TOKEN_PROGRAM_ID,
  associatedTokenProgram = spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  decimalFactor = new BN(Math.pow(10, DECIMALS)),
  amount = new BN(MINT_BALANCE).mul(decimalFactor),
  mint = pg.wallets.lfv.keypair.publicKey,
  giveawayId = new BN(1),
  drawNo = new BN(11),
  rewardMint = findRewardTokenMintAddress(),
  subscriptionAccount = findSubscriptionAccountAddress(memberWallet),
  vaultTokenAccount = findVaultAccountAddress(mint, memberWallet);

console.log(`Wallet: [${memberWallet.secretKey}]\n\n`);

(async () => {
  const rewardsInfo =
    await program.provider.connection.getAccountInfo(rewardMint);
  if (rewardsInfo) {
    console.log("Found mint account", rewardMint.toBase58());
    return; // Skip initializing again
  }
  console.log("Mint not found, attempting to create", rewardMint.toBase58());

  try {
    const metaAddress = findMetaplexAddress(rewardMint);
    const admin = pg.wallets.app.keypair;

    console.log("Meta address", metaAddress.toBase58());

    throw new Error("Mint not created");

    // const transaction = await program.methods
    //   .initializeMint({
    //     name: "Rewards",
    //     symbol: "ENTRY",
    //     uri: "https://cdn.lambosforvirgins.com/meta/entry.json",
    //     decimals: 4,
    //   })
    //   .accounts({
    //     metadata: metaAddress,
    //     mint: rewardMint,
    //     admin: admin.publicKey,
    //     tokenProgram: spl.TOKEN_PROGRAM_ID,
    //     tokenMetadataProgram: META_PROGRAM_ID,
    //     systemProgram: web3.SystemProgram.programId,
    //     rent: web3.SYSVAR_RENT_PUBKEY,
    //   })
    //   .transaction();

    // const txHash = await web3.sendAndConfirmTransaction(
    //   program.provider.connection,
    //   transaction,
    //   [admin],
    //   { skipPreflight: false }
    // );

    // console.log("Reward mint created with transaction:", txHash);
  } catch (error) {
    console.error(error);
  }
})();

(async () => {
  if (await program.provider.connection.getAccountInfo(rewardMint)) {
    console.log("Found mint account", rewardMint.toBase58());
    return; // Skip initializing again
  }

  console.log("Mint not found, attempting to create", rewardMint.toBase58());

  try {
    const payer = appWallet,
      mintAuthority = appWallet,
      lamports = await spl.getMinimumBalanceForRentExemptMint(
        program.provider.connection
      ),
      decimals = 4,
      transaction = new web3.Transaction();
    // Allocate the mint account as PDA
    transaction.add(
      web3.SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: rewardMint,
        space: spl.MINT_SIZE,
        lamports,
        programId: spl.TOKEN_PROGRAM_ID,
      })
    );
    // Initialize the mint with Token2022 program
    transaction.add(
      spl.createInitializeMintInstruction(
        rewardMint,
        decimals,
        mintAuthority.publicKey,
        null,
        spl.TOKEN_PROGRAM_ID
      )
    );

    await web3.sendAndConfirmTransaction(
      program.provider.connection,
      transaction,
      [payer],
      {
        commitment: "confirmed",
      }
    );

    console.log("Mint initialized with PDA at:", rewardMint.toBase58());
  } catch (error) {
    console.error(error);
  }
})();

const sourceTokenAccount = await spl.createAssociatedTokenAccount(
  program.provider.connection,
  appWallet,
  mint,
  memberWallet.publicKey,
  { commitment: "confirmed" },
  tokenProgram,
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

console.log(`Source token account: ${sourceTokenAccount.toBase58()}`);
console.log("Rewards token account", rewardsTokenAccount.toBase58());

const fundMemberWallet = async (wallet: web3.Keypair) => {
  try {
    const fundingTransaction = new web3.Transaction().add(
      web3.SystemProgram.transfer({
        fromPubkey: appWallet.publicKey,
        toPubkey: wallet.publicKey,
        lamports: 1 * web3.LAMPORTS_PER_SOL,
      }),
      spl.createMintToInstruction(
        mint,
        sourceTokenAccount,
        appWallet.publicKey,
        BigInt(MINT_BALANCE * decimalFactor.toNumber()),
        [],
        spl.TOKEN_PROGRAM_ID
      )
    );

    const signature = await program.provider.connection.sendTransaction(
      fundingTransaction,
      [appWallet]
    );

    await program.provider.connection.confirmTransaction(
      signature,
      "confirmed"
    );

    console.log(`Funded: ${wallet.publicKey.toBase58()}`);
  } catch (error) {
    console.error("Funding", error);
  }
};

const initializeMemberAccount = async () => {
  try {
    // Initialize member account
    const initTransaction = await program.methods
      .initialize()
      .accounts({
        subscription: subscriptionAccount,
        vaultTokenAccount,
        mint,
        signer: memberWallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      })
      .signers([memberWallet])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(
      initTransaction,
      "confirmed"
    );
    console.log("Initialize:", subscriptionAccount.toBase58());
  } catch (error) {
    console.error("Initialize", error);
  }
};

// Deposit tokens
const depositTokens = async (add: number, advanceTime?: number) => {
  const amountToAdd = new BN(add).mul(decimalFactor);

  if (advanceTime) await sleep(advanceTime);

  try {
    const transaction = await program.methods
      .deposit(amountToAdd)
      .accounts({
        subscription: subscriptionAccount,
        vaultTokenAccount: vaultTokenAccount,
        sourceTokenAccount,
        mint,
        signer: memberWallet.publicKey,
        systemProgram,
        tokenProgram,
      })
      .signers([memberWallet])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(transaction);
    console.log("Deposited:", add, "tx:", transaction);
  } catch (err) {
    console.error("Deposit", err);
  }
};

const releaseTokens = async (add: number, advanceTime?: number) => {
  const withdrawAmount = new BN(add).mul(decimalFactor);

  if (advanceTime) await sleep(advanceTime);

  try {
    const withdrawTransaction = await program.methods
      .release(withdrawAmount)
      .accounts({
        subscription: subscriptionAccount,
        vaultTokenAccount: vaultTokenAccount,
        sourceTokenAccount,
        mint,
        signer: memberWallet.publicKey,
        systemProgram,
        tokenProgram,
      })
      .signers([memberWallet])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(withdrawTransaction);
    console.log("Release", add, "tx", withdrawTransaction);
  } catch (err) {
    console.error("Failed with error", err);
  }
};

const withdrawTokens = async (advanceTime?: number) => {
  if (advanceTime) await sleep(advanceTime);

  try {
    const withdrawTransaction = await program.methods
      .withdraw()
      .accounts({
        subscription: subscriptionAccount,
        vaultTokenAccount: vaultTokenAccount,
        sourceTokenAccount,
        mint,
        signer: memberWallet.publicKey,
        systemProgram,
        tokenProgram,
      })
      .signers([memberWallet])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(withdrawTransaction);
    console.log("Withdraw tx", withdrawTransaction);
  } catch (err) {
    console.error("Failed with error", err);
  }
};

const claimRewards = async (advanceTime?: number) => {
  if (advanceTime) await sleep(advanceTime);

  try {
    const claimTransaction = await program.methods
      .claim()
      .accounts({
        subscription: subscriptionAccount,
        mint: rewardMint,
        destinationTokenAccount: rewardsTokenAccount,
        signer: memberWallet.publicKey,
        systemProgram,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      })
      .signers([memberWallet])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(claimTransaction);
    console.log("Claimed:", claimTransaction);
  } catch (err) {
    console.error("Claim", err);
  }
};

const openDraw = async (advanceTime?: number) => {
  if (advanceTime) await sleep(advanceTime);

  const draw = findDrawAccount(giveawayId, drawNo);

  try {
    const enterDrawTransaction = await program.methods
      .openDraw(giveawayId, drawNo)
      .accounts({
        draw,
        signer: appWallet.publicKey,
        systemProgram,
      })
      .signers([appWallet])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(enterDrawTransaction);
    console.log("Draw account", draw.toBase58());
  } catch (err) {
    console.error("Open draw error", err);
  }
};

const enterDraw = async (advanceTime?: number, entries: number = 1) => {
  if (advanceTime) await sleep(advanceTime);

  const entryTokenAccount = spl.getAssociatedTokenAddressSync(
    rewardMint,
    memberWallet.publicKey
  );
  const ticket = findTicketAccountAddress(giveawayId, drawNo, memberWallet);
  const draw = findDrawAccount(giveawayId, drawNo);

  try {
    const enterDrawTransaction = await program.methods
      .enter(new BN(entries))
      .accounts({
        draw,
        entries: rewardMint,
        entryTokenAccount,
        ticket,
        signer: memberWallet.publicKey,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([memberWallet])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(enterDrawTransaction);
    console.log("Entered draw:", draw.toBase58());
    console.log("Ticket", ticket.toBase58());
  } catch (err) {
    console.error("Enter error", err);
  }
};

const transactionQueue = [
  () => new Promise((resolve) => fundMemberWallet(memberWallet).then(resolve)),
  () => new Promise((resolve) => initializeMemberAccount().then(resolve)),
  // () => new Promise((resolve) => selfExclude().then(resolve)),
  () => new Promise((resolve) => depositTokens(10000).then(resolve)), // KGEN Airdrop $3.54
  () => new Promise((resolve) => depositTokens(1500, 500).then(resolve)), // Lambo eligability
  () => new Promise((resolve) => claimRewards(6000).then(resolve)),
  // () => new Promise((resolve) => openDraw(1000).then(resolve)),
  () => new Promise((resolve) => enterDraw(1000, 8).then(resolve)),
  // () => new Promise((resolve) => releaseTokens(500).then(resolve)),
  // () => new Promise((resolve) => withdrawTokens().then(resolve)),
  // () => new Promise((resolve) => claimRewards(3500).then(resolve)),
  // () => new Promise((resolve) => depositTokens(123_458, 10000).then(resolve)), // Super chad member
  // () => new Promise((resolve) => depositTokens(2_007_824).then(resolve)), // Mega chad member
  // () => new Promise((resolve) => depositTokens(3_720_345).then(resolve)), // Giga chad member
];

await transactionQueue.reduce((current, next) => {
  return current.then(() => next());
}, Promise.resolve());

const subscriptionState =
  await program.account.subscriptionAccount.fetch(subscriptionAccount);
const vaultAccount = await spl.getAccount(
  program.provider.connection,
  vaultTokenAccount,
  "confirmed",
  tokenProgram
);
const rewardsAccount = await spl.getAccount(
  program.provider.connection,
  rewardsTokenAccount,
  "confirmed",
  tokenProgram
);
const sourceAccount = await spl.getAccount(
  program.provider.connection,
  sourceTokenAccount,
  "confirmed",
  tokenProgram
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
