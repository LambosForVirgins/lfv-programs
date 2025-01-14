import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import spl from "@solana/spl-token";
import type { Constants } from "../target/types/constants";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Constants as anchor.Program<Constants>;

// Globals
const DECIMALS = 9;
const MINT_BALANCE = 15_050_542;
// Account seeds
enum Seed {
  MemberAccount = "member_account",
  VaultTokenAccount = "vault_token_account",
}
const findMemberAccountAddress = (signer: web3.Keypair) => {
  const [pda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from(Seed.MemberAccount), signer.publicKey.toBuffer()],
    program.programId
  );

  return pda;
};

const findVaultTokenAccountAddress = (
  mintKey: web3.PublicKey,
  signer: web3.Keypair
) => {
  const [pda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(Seed.VaultTokenAccount),
      mintKey.toBuffer(),
      signer.publicKey.toBuffer(),
    ],
    program.programId
  );

  return pda;
};

const memberWallet = new web3.Keypair(),
  appWallet = pg.wallets.app.keypair,
  program = program.programId,
  systemProgram = web3.SystemProgram.programId,
  tokenProgram = spl.TOKEN_PROGRAM_ID,
  associatedTokenProgram = spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  decimalFactor = new BN(Math.pow(10, DECIMALS)),
  amount = new BN(MINT_BALANCE).mul(decimalFactor),
  mint = pg.wallets.mint.keypair.publicKey,
  memberAccount = findMemberAccountAddress(memberWallet),
  vaultTokenAccount = findVaultTokenAccountAddress(mint, memberWallet);

console.log(`[${memberWallet.secretKey}]`);

const sourceTokenAccount = await spl.createAssociatedTokenAccount(
  program.provider.connection,
  appWallet,
  mint,
  memberWallet.publicKey,
  { commitment: "confirmed" },
  tokenProgram,
  associatedTokenProgram
);

const fundingTransaction = new web3.Transaction().add(
  web3.SystemProgram.transfer({
    fromPubkey: appWallet.publicKey,
    toPubkey: memberWallet.publicKey,
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

const signature = await program.provider.connection.sendTransaction(fundingTransaction, [
  appWallet,
]);

await program.provider.connection.confirmTransaction(signature, "confirmed");

// Initialize member account
const initTransaction = await program.methods
  .initialize()
  .accounts({
    memberAccount,
    vaultTokenAccount,
    mint,
    signer: memberWallet.publicKey,
    tokenProgram: spl.TOKEN_PROGRAM_ID,
    systemProgram: web3.SystemProgram.programId,
  })
  .signers([memberWallet])
  .rpc();
// Confirm transaction
await program.provider.connection.confirmTransaction(initTransaction, "confirmed");

// Self exclude member account
// const excludeTransaction = await program.methods
//   .exclude()
//   .accounts({
//     memberAccount,
//     signer: memberWallet.publicKey,
//   })
//   .signers([memberWallet])
//   .rpc();
// // Confirm transaction
// await program.provider.connection.confirmTransaction(excludeTransaction, "confirmed");

// Deposit tokens
const depositTokens = async (add: number, advanceTime?: number) => {
  const amountToAdd = new BN(add).mul(decimalFactor);

  if (advanceTime) await sleep(advanceTime);

  try {
    const transaction = await program.methods
      .deposit(amountToAdd)
      .accounts({
        memberAccount,
        vaultTokenAccount,
        sourceTokenAccount,
        mint,
        signer: memberWallet.publicKey,
        systemProgram,
        tokenProgram,
      })
      .signers([memberWallet])
      .rpc();
    console.log("Deposit", add, "TX", transaction);
    // Confirm transaction
    await program.provider.connection.confirmTransaction(transaction);
  } catch (err) {
    console.error("Failed with error", err);
  }
};

const withdrawTokens = async (add: number, advanceTime?: number) => {
  const withdrawAmount = new BN(add).mul(decimalFactor);

  if (advanceTime) await sleep(advanceTime);

  try {
    const withdrawTransaction = await program.methods
      .withdraw(withdrawAmount)
      .accounts({
        memberAccount,
        vaultTokenAccount,
        sourceTokenAccount,
        mint,
        signer: memberWallet.publicKey,
        systemProgram,
        tokenProgram,
      })
      .signers([memberWallet])
      .rpc();
    console.log("Withdraw", add, "tx", withdrawTransaction);
    // Confirm transaction
    await program.provider.connection.confirmTransaction(withdrawTransaction);
  } catch (err) {
    console.error("Failed with error", err);
  }
};

const depositQueue = [
  () => new Promise((resolve) => depositTokens(1000).then(resolve)), // KGEN Airdrop $3.54
  () => new Promise((resolve) => depositTokens(1500, 500).then(resolve)), // Lambo eligability
  () => new Promise((resolve) => withdrawTokens(500, 3000).then(resolve)),
  () => new Promise((resolve) => depositTokens(123_458, 10000).then(resolve)), // Super chad member
  // () => new Promise((resolve) => depositTokens(2_007_824).then(resolve)), // Mega chad member
  // () => new Promise((resolve) => depositTokens(3_720_345).then(resolve)), // Giga chad member
];

await depositQueue.reduce((current, next) => {
  return current.then(() => next());
}, Promise.resolve());

const memberState = await program.account.memberAccount.fetch(memberAccount);
const vaultAccount = await spl.getAccount(
  program.provider.connection,
  vaultTokenAccount,
  "confirmed",
  tokenProgram
);
const sourceAccount = await spl.getAccount(
  program.provider.connection,
  sourceTokenAccount,
  "confirmed",
  tokenProgram
);
// Log addresses
console.log("----------- ADDRESSES -------------");
console.log("Token Mint        :", mint.toBase58());
console.log("Entry Mint        :", "Not set");
console.log("  | Source account:", sourceTokenAccount.toBase58());
console.log("     > Original   :", amount.div(decimalFactor).toString());
console.log("     > Remaining  :", sourceAccount.amount);
console.log("Program           :", program.toBase58());
console.log("Admin             :", appWallet.publicKey.toBase58());
console.log("  | Global pool   :", "Not set");
console.log("Member            :", memberWallet.publicKey.toBase58());
console.log("  | Member state  :", memberAccount.toBase58());
console.log("     > Status     :", memberState.status);
console.log("     > Tier       :", memberState.tier);
console.log(
  "     > Locked     :",
  memberState.totalAmount.div(decimalFactor).toString()
);
console.log(
  "     > Matured    :",
  memberState.totalMatured.div(decimalFactor).toString()
);
console.log("     > Pending    :", "Not set");
console.log("     > Available  :", "Not set");
console.log("     > Entries    :", memberState.totalVouchers.toString());
console.log("     > Slots      :", memberState.slots.length);
console.log("  | Vault account :", vaultTokenAccount.toBase58());
console.log("     > Amount     :", vaultAccount.amount);
console.log("-----------------------------------");

console.log("DISPLAY TRANSACTION HISTORY");
console.log("DISPLAY LOCKED DURATION");
