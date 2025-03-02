import { type PublicKey, type TransactionInstruction } from "@solana/web3.js";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { findRewardMintAddress, findRewardTokenAccountAddress } from "@/pda";
import { Logger } from "@/tools/Logger";

export const getInitializeRewardTokenAccountInstruction = (
  owner: PublicKey
): TransactionInstruction => {
  const mint = findRewardMintAddress(),
    associatedTokenAddress = findRewardTokenAccountAddress(owner);

  console.log("reward mint", mint.toBase58());

  Logger.info(
    "Initializing reward associated token account",
    associatedTokenAddress.toBase58()
  );

  return createAssociatedTokenAccountInstruction(
    owner,
    associatedTokenAddress,
    owner,
    mint,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );
};
