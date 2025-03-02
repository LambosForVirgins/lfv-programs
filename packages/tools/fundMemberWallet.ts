import {
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { MINT_ADDRESS, MINT_DECIMALS, program } from "../../client/constants";
import BN from "bn.js";
import { Logger } from "./Logger";

export const fundMemberWallet = async (
  initialMintAmount: BN,
  receiver: PublicKey,
  signer: Keypair
) => {
  const decimalFactor = new BN(Math.pow(10, 9));
  const solAmount = new BN(1).mul(decimalFactor);

  const destinationTokenAccount = getAssociatedTokenAddressSync(
    MINT_ADDRESS,
    receiver
  );

  try {
    const fundingTransaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: signer.publicKey,
        toPubkey: receiver,
        lamports: solAmount.toNumber(),
      }),
      createMintToInstruction(
        MINT_ADDRESS,
        destinationTokenAccount,
        signer.publicKey,
        initialMintAmount.toNumber(),
        [],
        TOKEN_PROGRAM_ID
      )
    );

    const signature = await program.provider.connection.sendTransaction(
      fundingTransaction,
      [signer]
    );

    await program.provider.connection.confirmTransaction(
      signature,
      "confirmed"
    );

    Logger.success(`Funded member`, receiver.toBase58());
  } catch (error) {
    console.error("Funding", error);
  }
};
