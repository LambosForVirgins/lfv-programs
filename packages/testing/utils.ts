import { BN, web3 } from "@coral-xyz/anchor";
import { MINT_ADDRESS, program } from "../../client/constants";
import spl from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";
import { readFileSync } from "fs";

const tokenMint = MINT_ADDRESS;

export const sleep = (ms: number): Promise<void> => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

export const loadKeypairFileSync = (path: string): Keypair => {
  try {
    return Keypair.fromSecretKey(
      new Uint8Array(JSON.parse(readFileSync(path).toString()))
    );
  } catch (error) {
    throw new Error("Failed to load keypair");
  }
};

export const assertErrorAsync = async (
  fn: Promise<any>,
  message: string
): Promise<void> => {
  try {
    await fn;
    assert.fail();
  } catch (error: any) {
    assert(error.message === message);
  }
};

export const mintTokenToAccount = async (
  amount: BN,
  member: PublicKey,
  signer: Keypair
) => {
  const tokenAccount = await spl.createAssociatedTokenAccount(
    program.provider.connection,
    signer,
    tokenMint,
    member,
    { commitment: "finalized" },
    spl.TOKEN_PROGRAM_ID,
    spl.ASSOCIATED_TOKEN_PROGRAM_ID
  );

  const transaction = new web3.Transaction().add(
    web3.SystemProgram.transfer({
      fromPubkey: signer.publicKey,
      toPubkey: member,
      lamports: 1 * web3.LAMPORTS_PER_SOL,
    }),
    spl.createMintToInstruction(
      tokenMint,
      tokenAccount,
      signer.publicKey,
      amount.mul(new BN(10)).toNumber(),
      [],
      spl.TOKEN_PROGRAM_ID
    )
  );

  const signature = await program.provider.connection.sendTransaction(
    transaction,
    [signer]
  );

  await program.provider.connection.confirmTransaction(signature, "confirmed");
};
