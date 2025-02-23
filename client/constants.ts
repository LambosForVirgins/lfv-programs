import { RewardProgram } from "../target/types/reward_program";
import IDL from "../target/idl/reward_program.json";
import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

export const REWARD_PROGRAM_ID = new PublicKey("");

export const METAPLEX_PROGRAM_ID = new PublicKey(
  "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
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
  RewardTokenMint = "reward",
  DrawAccount = "draw",
  TicketAccount = "ticket",
  Metadata = "metadata",
}
