import { PublicKey, Connection } from "@solana/web3.js";

export const isMintInitialized = async (
  connection: Connection,
  mint: PublicKey
) => {
  try {
    const rewardsInfo = await connection.getAccountInfo(mint);
    return !!rewardsInfo;
  } catch (err) {
    return false;
  }
};
