import { PublicKey } from "@solana/web3.js";
import { METAPLEX_PROGRAM_ID, Seed } from "../../client/constants";

export const findMetaplexAddress = (mintAddress: PublicKey) => {
  const [metadataAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from(Seed.Metadata),
      METAPLEX_PROGRAM_ID.toBuffer(),
      mintAddress.toBuffer(),
    ],
    METAPLEX_PROGRAM_ID
  );

  return metadataAddress;
};
