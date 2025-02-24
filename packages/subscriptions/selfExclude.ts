import { Keypair, Signer } from "@solana/web3.js";
import { program } from "../../client/constants";
import { findSubscriptionAccountAddress } from "../pda";

const selfExclude = async (signer: Keypair) => {
  if (!("exclude" in program.methods))
    throw new Error("Method 'exclude' not found");

  const subscriptionAccount = findSubscriptionAccountAddress(signer);
  // Self exclude member account
  const excludeTransaction = await program.methods
    .exclude()
    .accounts({
      subscription: subscriptionAccount,
      signer: signer.publicKey,
    })
    .signers([signer])
    .rpc();
  // Confirm transaction
  await program.provider.connection.confirmTransaction(
    excludeTransaction,
    "confirmed"
  );
  console.log(`Excluded member: ${subscriptionAccount.toBase58()}`);
};
