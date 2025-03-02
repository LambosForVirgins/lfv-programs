import { RewardProgram } from "../target/types/reward_program";
import IDL from "../target/idl/reward_program.json";
import { PublicKey, Connection } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import { loadKeypairFileSync } from "@/testing/utils";

export const appWallet = loadKeypairFileSync(
  "./keys/APP1udKK1p1G7eE3PeTHp5qo8FDTAzmxC5buW7Luss3B.json"
);

const wallet = new anchor.Wallet(appWallet);

const connection = new Connection("https://api.devnet.solana.com", "confirmed");

const provider = new anchor.AnchorProvider(connection, wallet, {
  preflightCommitment: "confirmed",
});

// Configure the client to use the local cluster
anchor.setProvider(provider);

export const MINT_DECIMALS = 9;

export const REWARD_PROGRAM_ID = new PublicKey(
  "9QZ5nMuz1cH4Nb7mWwSDrXy5zMWg1DT6TSjdgga933wU"
);

export const METAPLEX_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

export const MINT_ADDRESS = new PublicKey(
  "LFVqPrRGnwYdCwFcDzShBxN2GMFmD4AoCMrjxjq4xdz"
);

export const program = new anchor.Program<RewardProgram>(
  IDL as RewardProgram,
  REWARD_PROGRAM_ID
);

// Account seeds
export enum Seed {
  SubscriptionAccount = "subscription",
  MemberAccount = "member",
  VaultAccount = "vault",
  RewardMint = "reward",
  DrawAccount = "draw",
  TicketAccount = "ticket",
  Metadata = "metadata",
}
