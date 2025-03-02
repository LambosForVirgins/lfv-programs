import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { getInitializeRewardTokenAccountInstruction } from "./getInitializeRewardTokenAccountInstruction";
import { getClaimRewardsTransaction } from "./getClaimRewardsInstruction";
import { Logger } from "@/tools/Logger";

export const claimRewards = async (
  connection: Connection,
  signer: Keypair,
  rewardTokenAccount: PublicKey
) => {
  try {
    const transaction = new Transaction();

    if (!rewardTokenAccount) {
      // TODO: This doesn't generate the correct program derived address for the seed
      transaction.add(
        getInitializeRewardTokenAccountInstruction(signer.publicKey)
      );
    }

    transaction.add(
      await getClaimRewardsTransaction(signer.publicKey, rewardTokenAccount)
    );

    const txHash = await connection.sendTransaction(transaction, [signer], {
      skipPreflight: false,
    });
    // Confirm transaction
    await connection.confirmTransaction(txHash, "finalized");
    Logger.success("Claimed rewards", txHash);
  } catch (err) {
    console.error("Claim", err);
  }
};
