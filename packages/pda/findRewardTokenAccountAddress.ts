import { PublicKey } from "@solana/web3.js";
import { program } from "../../client/constants";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { findRewardMintAddress } from "./findRewardMintAddress";

export const findRewardTokenAccountAddress = (owner: PublicKey) => {
  const rewardMint = findRewardMintAddress();
  const pda = getAssociatedTokenAddressSync(
    rewardMint,
    owner,
    false,
    program.programId
  );

  return pda;
};
