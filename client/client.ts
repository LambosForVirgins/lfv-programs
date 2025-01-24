import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import spl from "@solana/spl-token";
import {
  createCreateMetadataAccountV3Instruction,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
} from "@metaplex-foundation/mpl-token-metadata";
import type { Constants } from "../target/types/constants";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.Constants as anchor.Program<Constants>;

// Globals
const DECIMALS = 9;
const MINT_BALANCE = 15_050_542;
const META_PROGRAM_ID = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
const rewardMint = new web3.PublicKey(
  "CoiNPkhS1a3RWpd6DELx2h4CSWkp489yb3s4KhDdzS8"
);
// Account seeds
enum Seed {
  SubscriptionAccount = "subscription",
  MemberAccount = "member",
  VaultTokenAccount = "vault",
  RewardTokenAccount = "reward",
}

const findSubscriptionAccountAddress = (signer: web3.Keypair) => {
  const [pda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from(Seed.SubscriptionAccount), signer.publicKey.toBuffer()],
    program.programId
  );

  return pda;
};

const findMemberAccountAddress = (signer: web3.Keypair) => {
  const [pda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from(Seed.MemberAccount), signer.publicKey.toBuffer()],
    program.programId
  );

  return pda;
};

const findRewardTokenAccountAddress = (
  mintKey: web3.PublicKey,
  signer: web3.Keypair
) => {
  const [pda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from(Seed.RewardTokenAccount),
      mintKey.toBuffer(),
      signer.publicKey.toBuffer(),
    ],
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

const getMetadataAddress = (mintAddress: web3.PublicKey) => {
  const [metadataAddress] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      new web3.PublicKey(META_PROGRAM_ID).toBuffer(),
      mintAddress.toBuffer(),
    ],
    new web3.PublicKey(META_PROGRAM_ID)
  );

  return metadataAddress;
};

const memberWallet = new web3.Keypair(),
  appWallet = pg.wallets.app.keypair,
  program = program.programId,
  systemProgram = web3.SystemProgram.programId,
  tokenProgram = spl.TOKEN_PROGRAM_ID,
  associatedTokenProgram = spl.ASSOCIATED_TOKEN_PROGRAM_ID,
  decimalFactor = new BN(Math.pow(10, DECIMALS)),
  amount = new BN(MINT_BALANCE).mul(decimalFactor),
  mint = pg.wallets.lfv.keypair.publicKey,
  subscriptionAccount = findSubscriptionAccountAddress(memberWallet),
  vaultTokenAccount = findVaultTokenAccountAddress(mint, memberWallet);

console.log(`Wallet: [${memberWallet.secretKey}]\n\n`);

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
  spl.TOKEN_2022_PROGRAM_ID,
  associatedTokenProgram
);

console.log(`Source token account: ${sourceTokenAccount.toBase58()}`);
console.log("Rewards token account", rewardsTokenAccount.toBase58());

const createMetadataTransaction = () => {
  const address = getMetadataAddress(mint);

  const accounts: CreateMetadataAccountV3InstructionAccounts = {
    metadata: address,
    mint,
    mintAuthority: appWallet.publicKey,
    payer: appWallet.publicKey,
    updateAuthority: appWallet.publicKey,
  };

  const args: CreateMetadataAccountV3InstructionArgs = {
    createMetadataAccountArgsV3: {
      data: {
        name: "NotLambosForVirgins",
        symbol: "VERGEN",
        uri: "https://peach-metadata.s3.ap-southeast-2.amazonaws.com/meta.json",
        sellerFeeBasisPoints: 0,
        creators: [
          { address: appWallet.publicKey, verified: true, share: 100 },
        ],
        collection: null,
        uses: null,
      },
      isMutable: true,
      collectionDetails: null,
    },
  };

  return new web3.Transaction().add(
    createCreateMetadataAccountV3Instruction(
      accounts,
      args,
      new web3.PublicKey(META_PROGRAM_ID)
    )
  );
};

const setMetadata = async () => {
  console.log("Setting metadata");
  const transaction = createMetadataTransaction();
  const signature = await program.provider.connection.sendTransaction(
    transaction,
    [appWallet],
    {
      skipPreflight: false,
      preflightCommitment: "processed",
    }
  );

  await program.provider.connection.confirmTransaction(signature, "processed");
  console.log("Metadata created with transaction:", signature);
};

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

    const signature = await program.provider.connection.sendTransaction(fundingTransaction, [
      appWallet,
    ]);

    await program.provider.connection.confirmTransaction(signature, "confirmed");

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
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([memberWallet])
      .rpc();
    // Confirm transaction
    await program.provider.connection.confirmTransaction(initTransaction, "confirmed");
    console.log("Initialize:", subscriptionAccount.toBase58());
  } catch (error) {
    console.error("Initialize", error);
  }
};

const selfExclude = async () => {
  // Self exclude member account
  const excludeTransaction = await program.methods
    .exclude()
    .accounts({
      subscription: subscriptionAccount,
      signer: memberWallet.publicKey,
    })
    .signers([memberWallet])
    .rpc();
  // Confirm transaction
  await program.provider.connection.confirmTransaction(excludeTransaction, "confirmed");
  console.log(`Excluded subscription: ${subscriptionAccount.toBase58()}`);
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
        vaultTokenAccount,
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
        vaultTokenAccount,
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
        vaultTokenAccount,
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

  console.log("Reward mint", rewardMint.toBase58());
  console.log("Reward token program", spl.TOKEN_2022_PROGRAM_ID.toBase58());

  try {
    const claimTransaction = await program.methods
      .claim()
      .accounts({
        subscription: subscriptionAccount,
        mint: rewardMint,
        authority: appWallet.publicKey,
        tokenAccount: rewardsTokenAccount,
        signer: memberWallet.publicKey,
        systemProgram,
        tokenProgram: spl.TOKEN_2022_PROGRAM_ID,
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

const transactionQueue = [
  // () => new Promise((resolve) => setMetadata().then(resolve)),
  () => new Promise((resolve) => fundMemberWallet(memberWallet).then(resolve)),
  () => new Promise((resolve) => initializeMemberAccount().then(resolve)),
  // () => new Promise((resolve) => selfExclude().then(resolve)),
  () => new Promise((resolve) => depositTokens(1000).then(resolve)), // KGEN Airdrop $3.54
  () => new Promise((resolve) => depositTokens(1500, 500).then(resolve)), // Lambo eligability
  () => new Promise((resolve) => claimRewards(1000).then(resolve)),
  () => new Promise((resolve) => releaseTokens(500).then(resolve)),
  // () => new Promise((resolve) => withdrawTokens().then(resolve)),
  // () => new Promise((resolve) => claimRewards(3500).then(resolve)),
  // () => new Promise((resolve) => depositTokens(123_458, 10000).then(resolve)), // Super chad member
  // () => new Promise((resolve) => depositTokens(2_007_824).then(resolve)), // Mega chad member
  // () => new Promise((resolve) => depositTokens(3_720_345).then(resolve)), // Giga chad member
];

await transactionQueue.reduce((current, next) => {
  return current.then(() => next());
}, Promise.resolve());

const subscriptionState = await program.account.subscriptionAccount.fetch(
  subscriptionAccount
);
const vaultAccount = await spl.getAccount(
  program.provider.connection,
  vaultTokenAccount,
  "confirmed",
  tokenProgram
);
// const rewardsAccount = await spl.getAccount(
//   program.provider.connection,
//   rewardTokenAccount,
//   "confirmed",
//   tokenProgram
// );
const sourceAccount = await spl.getAccount(
  program.provider.connection,
  sourceTokenAccount,
  "confirmed",
  tokenProgram
);

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
// console.log("     > Entries    :", rewardsAccount.amount);
console.log("     > Slots      :", subscriptionState.slots.length);
console.log("  | Vault account :", vaultTokenAccount.toBase58());
console.log("     > Amount     :", vaultAccount.amount);
console.log("-----------------------------------");

// console.log("DISPLAY TRANSACTION HISTORY");
// console.log("DISPLAY LOCKED DURATION");
