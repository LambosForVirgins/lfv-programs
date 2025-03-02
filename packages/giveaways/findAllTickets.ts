import { web3 } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { program } from "../../client/constants";

export const findAllTickets = async (owner: PublicKey) => {
  const ticketOwnerFilter: web3.GetProgramAccountsFilter = {
    memcmp: {
      offset: 9,
      bytes: owner.toBase58(),
    },
  };

  const ticketAccounts = await program.provider.connection.getProgramAccounts(
    program.programId,
    { filters: [ticketOwnerFilter] }
  );
  // TODO: Remap ticket accounts to a more readable format
  return ticketAccounts;
};
