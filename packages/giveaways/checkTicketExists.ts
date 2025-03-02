import { Connection, PublicKey } from "@solana/web3.js";

export const checkTicketExists = async (
  connection: Connection,
  ticket: PublicKey
): Promise<boolean> => {
  try {
    const ticketAccount = await connection.getAccountInfo(ticket);
    return ticketAccount !== null;
  } catch (err) {
    return false;
  }
};
