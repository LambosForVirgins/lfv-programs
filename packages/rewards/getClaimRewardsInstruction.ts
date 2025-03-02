import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { program } from "../../client/constants";
import {
  findRewardMintAddress,
  findRewardTokenAccountAddress,
  findSubscriptionAccountAddress,
} from "@/pda";
import { Logger } from "@/tools/Logger";

export const getClaimRewardsTransaction = async (
  publicKey: PublicKey,
  destinationTokenAccount: PublicKey
): Promise<TransactionInstruction> => {
  const subscription = findSubscriptionAccountAddress(publicKey),
    mint = findRewardMintAddress();

  return program.methods
    .claim()
    .accounts({
      subscription,
      mint,
      destinationTokenAccount,
      signer: publicKey,
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .instruction();
};
