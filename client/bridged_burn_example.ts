import {
  Connection,
  PublicKey,
  Keypair,
  Transaction,
  SystemProgram,
  SYSVAR_CLOCK_PUBKEY,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createAssociatedTokenAccountInstruction,
  mintTo,
  createMint,
} from "@solana/spl-token";
import { Program, AnchorProvider, Wallet, BN } from "@coral-xyz/anchor";
// import { BridgedBurn } from "../target/types/bridged_burn"; // Uncomment when IDL is generated
type BridgedBurn = any; // Temporary type

// Configuration
const PROGRAM_ID = new PublicKey(
  "4rGdLkQDuZcJhCM85wwcpcyM7t5GxtpjAapV2LR6buiK"
);
const RPC_URL = process.env.SOLANA_RPC_URL || "https://api.devnet.solana.com";

/**
 * Example Sui address (32 bytes)
 * In practice, this would come from the user's Sui wallet
 */
const EXAMPLE_SUI_ADDRESS = new Uint8Array([
  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
  0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
  0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
]);

interface BurnAndCloseParams {
  connection: Connection;
  program: Program<BridgedBurn>;
  owner: Keypair;
  tokenAccount: PublicKey;
  mint: PublicKey;
  vault: PublicKey;
  suiAddress: Uint8Array;
}

/**
 * Burns tokens and sends confirmation to Sui via Wormhole
 */
export async function burnAndClose(
  params: BurnAndCloseParams
): Promise<string> {
  const { connection, program, owner, tokenAccount, mint, vault, suiAddress } =
    params;

  // Validate inputs
  if (suiAddress.length !== 32) {
    throw new Error("Sui address must be exactly 32 bytes");
  }

  // Get token account info to check balance
  const tokenAccountInfo =
    await connection.getTokenAccountBalance(tokenAccount);
  const balance = tokenAccountInfo.value.uiAmount || 0;

  if (balance === 0) {
    throw new Error("Token account has no tokens to burn");
  }

  console.log(`🔥 Burning ${balance} tokens...`);
  console.log(`📧 Sui receiver: 0x${Buffer.from(suiAddress).toString("hex")}`);
  console.log(`💰 Token mint: ${mint.toString()}`);
  console.log(`🏦 Vault: ${vault.toString()}`);

  try {
    // Execute burn and close instruction
    const signature = await program.methods
      .burnAndClose(Array.from(suiAddress))
      .accounts({
        owner: owner.publicKey,
        tokenAccount,
        mint,
        vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    console.log(`✅ Transaction confirmed: ${signature}`);
    console.log(
      `🔗 View on explorer: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );

    return signature;
  } catch (error) {
    console.error("❌ Burn and close failed:", error);
    throw error;
  }
}

/**
 * Sets up a test environment with a token account containing tokens
 */
export async function setupTestEnvironment(
  connection: Connection,
  payer: Keypair
): Promise<{
  mint: PublicKey;
  tokenAccount: PublicKey;
  vault: PublicKey;
}> {
  console.log("🏗️  Setting up test environment...");

  // Create a new mint
  const mint = await createMint(
    connection,
    payer,
    payer.publicKey, // mint authority
    null, // freeze authority
    9 // decimals
  );

  console.log(`🏭 Created mint: ${mint.toString()}`);

  // Create associated token account
  const tokenAccount = await getAssociatedTokenAddress(mint, payer.publicKey);

  const createTokenAccountIx = createAssociatedTokenAccountInstruction(
    payer.publicKey, // payer
    tokenAccount, // associated token account
    payer.publicKey, // owner
    mint // mint
  );

  // Create vault account (simple keypair for this example)
  const vault = Keypair.generate().publicKey;

  // Send transaction to create token account
  const transaction = new Transaction().add(createTokenAccountIx);
  const signature = await connection.sendTransaction(transaction, [payer]);
  await connection.confirmTransaction(signature);

  console.log(`💳 Created token account: ${tokenAccount.toString()}`);

  // Mint some tokens to the account
  const mintAmount = 1000 * 10 ** 9; // 1000 tokens
  await mintTo(connection, payer, mint, tokenAccount, payer, mintAmount);

  console.log(`💰 Minted ${mintAmount / 10 ** 9} tokens to account`);

  return { mint, tokenAccount, vault };
}

/**
 * Parses burn confirmation payload from Wormhole event
 */
export interface BurnConfirmation {
  messageType: number;
  suiReceiver: Uint8Array;
  solanaSender: Uint8Array;
  mint: Uint8Array;
  amount: bigint;
  timestamp: bigint;
}

export function parseBurnConfirmationPayload(
  payload: Uint8Array
): BurnConfirmation {
  if (payload.length !== 113) {
    throw new Error(
      `Invalid payload size: expected 113 bytes, got ${payload.length}`
    );
  }

  let offset = 0;

  // Parse message type (1 byte)
  const messageType = payload[offset];
  offset += 1;

  // Parse Sui receiver (32 bytes)
  const suiReceiver = payload.slice(offset, offset + 32);
  offset += 32;

  // Parse Solana sender (32 bytes)
  const solanaSender = payload.slice(offset, offset + 32);
  offset += 32;

  // Parse mint (32 bytes)
  const mint = payload.slice(offset, offset + 32);
  offset += 32;

  // Parse amount (8 bytes, little-endian)
  const amountBytes = payload.slice(offset, offset + 8);
  const amount = new DataView(amountBytes.buffer).getBigUint64(0, true);
  offset += 8;

  // Parse timestamp (8 bytes, little-endian)
  const timestampBytes = payload.slice(offset, offset + 8);
  const timestamp = new DataView(timestampBytes.buffer).getBigUint64(0, true);

  return {
    messageType,
    suiReceiver,
    solanaSender,
    mint,
    amount,
    timestamp,
  };
}

/**
 * Monitors events from the bridged burn program
 */
export async function monitorBurnEvents(
  connection: Connection,
  program: Program<BridgedBurn>,
  callback: (event: any) => void
): Promise<void> {
  console.log("👀 Monitoring burn events...");

  // Listen for BridgeBurnEvent
  program.addEventListener("BridgeBurnEvent", (event, slot) => {
    console.log("🔥 Burn Event:", {
      slot,
      suiReceiver: `0x${Buffer.from(event.suiReceiver).toString("hex")}`,
      solSender: event.solSender.toString(),
      mint: event.mint.toString(),
      amount: event.amount.toString(),
    });
    callback({ type: "burn", event, slot });
  });

  // Listen for WormholeMessageEvent
  program.addEventListener("WormholeMessageEvent", (event, slot) => {
    console.log("🌉 Wormhole Message Event:", {
      slot,
      targetChain: event.targetChain,
      payloadSize: event.payload.length,
      consistencyLevel: event.consistencyLevel,
    });

    // Parse the payload
    try {
      const payload = new Uint8Array(event.payload);
      const burnConfirmation = parseBurnConfirmationPayload(payload);
      console.log("📦 Parsed Burn Confirmation:", {
        messageType: burnConfirmation.messageType,
        suiReceiver: `0x${Buffer.from(burnConfirmation.suiReceiver).toString("hex")}`,
        solanaSender: `0x${Buffer.from(burnConfirmation.solanaSender).toString("hex")}`,
        mint: `0x${Buffer.from(burnConfirmation.mint).toString("hex")}`,
        amount: burnConfirmation.amount.toString(),
        timestamp: new Date(
          Number(burnConfirmation.timestamp) * 1000
        ).toISOString(),
      });
    } catch (error) {
      console.error("❌ Failed to parse payload:", error);
    }

    callback({ type: "wormhole", event, slot });
  });
}

/**
 * Example usage
 */
export async function main() {
  // Setup connection and wallet
  const connection = new Connection(RPC_URL, "confirmed");
  const payer = Keypair.generate(); // In practice, load from env or user input

  // Airdrop SOL for testing (devnet only)
  if (RPC_URL.includes("devnet")) {
    console.log("💰 Requesting SOL airdrop...");
    const airdropSignature = await connection.requestAirdrop(
      payer.publicKey,
      2_000_000_000
    ); // 2 SOL
    await connection.confirmTransaction(airdropSignature);
  }

  // Setup program
  const wallet = new Wallet(payer);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  const program = new Program<BridgedBurn>(
    require("../target/idl/bridged_burn.json"),
    PROGRAM_ID,
    provider
  );

  // Setup test environment
  const { mint, tokenAccount, vault } = await setupTestEnvironment(
    connection,
    payer
  );

  // Start monitoring events
  await monitorBurnEvents(connection, program, (event) => {
    console.log("📢 Event received:", event.type);
  });

  // Execute burn and close
  const signature = await burnAndClose({
    connection,
    program,
    owner: payer,
    tokenAccount,
    mint,
    vault,
    suiAddress: EXAMPLE_SUI_ADDRESS,
  });

  console.log("🎉 Burn and close completed successfully!");
  console.log(`📋 Transaction: ${signature}`);
}

// Run if this file is executed directly
if (require.main === module) {
  main().catch(console.error);
}
