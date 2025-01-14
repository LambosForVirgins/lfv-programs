import * as anchor from "@coral-xyz/anchor";
import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import spl from "@solana/spl-token";
import type { Constants } from "../target/types/constants";
const DECIMALS = 9;
const AMOUNT = 100;

const assertErrorAsync = async (
  fn: Promise<any>,
  message: string
): Promise<void> => {
  try {
    await fn;
    assert.fail();
  } catch (error) {
    assert(error.message === message);
  }
};

const getTokenBalance = async (tokenAccount: web3.PublicKey): Promise<BN> => {
  return await spl
    .getAccount(program.provider.connection, tokenAccount, "confirmed", spl.TOKEN_PROGRAM_ID)
    .then((info) => new BN(Number(info.amount)));
};

const findMemberAccount = (signer: web3.Keypair) => {
  const [pda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("member_account"), signer.publicKey.toBuffer()],
    program.programId
  );

  return pda;
};

const findVaultTokenAccountAddress = (
  mint: web3.PublicKey,
  signer: web3.Keypair
) => {
  const [pda] = web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault_token_account"),
      mint.toBuffer(),
      signer.publicKey.toBuffer(),
    ],
    program.programId
  );

  return pda;
};

describe("Member Locker", async () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Constants as anchor.Program<Constants>;
  
  const memberWallet = new web3.Keypair(),
    adminWallet = pg.wallets.app.keypair,
    tokenMint = pg.wallets.mint.keypair,
    entryMint = pg.wallets.gift.keypair;

  const amount = new BN(AMOUNT * Math.pow(10, DECIMALS));

  describe("Precheck test data requirements", () => {
    let tokenAccount: web3.PublicKey;

    before(async () => {
      tokenAccount = await spl.createAssociatedTokenAccount(
        program.provider.connection,
        adminWallet,
        tokenMint.publicKey,
        memberWallet.publicKey,
        { commitment: "finalized" },
        spl.TOKEN_PROGRAM_ID,
        spl.ASSOCIATED_TOKEN_PROGRAM_ID
      );

      const transaction = new web3.Transaction().add(
        web3.SystemProgram.transfer({
          fromPubkey: adminWallet.publicKey,
          toPubkey: memberWallet.publicKey,
          lamports: 1 * web3.LAMPORTS_PER_SOL,
        }),
        spl.createMintToInstruction(
          tokenMint.publicKey,
          tokenAccount,
          adminWallet.publicKey,
          amount.mul(new BN(10)).toNumber(),
          [],
          spl.TOKEN_PROGRAM_ID
        )
      );

      const signature = await program.provider.connection.sendTransaction(transaction, [
        adminWallet,
      ]);

      await program.provider.connection.confirmTransaction(signature, "confirmed");

      tokenAccount = spl.getAssociatedTokenAddressSync(
        tokenMint.publicKey,
        memberWallet.publicKey,
        false,
        spl.TOKEN_PROGRAM_ID,
        spl.ASSOCIATED_TOKEN_PROGRAM_ID
      );
    });

    it("initialized associated token account", async () => {
      // TODO: Make this check initialization instead of throw
      await spl.getAccount(
        program.provider.connection,
        tokenAccount,
        "confirmed",
        spl.TOKEN_PROGRAM_ID
      );
    });

    it("member has adequete token mint balance", async () => {
      const balance = await getTokenBalance(tokenAccount);
      assert(
        balance.sub(amount).toNumber() > 0,
        `Insufficient member token balance`
      );
    });
  });

  describe("Initialize", () => {
    let memberAccount: web3.PublicKey,
      vaultTokenAccount: web3.PublicKey,
      associatedTokenAccount: spl.Account;

    before(async () => {
      // Generate member program account keys
      memberAccount = findMemberAccount(memberWallet);
      vaultTokenAccount = findVaultTokenAccountAddress(
        tokenMint.publicKey,
        memberWallet
      );
      // Send transaction
      const txHash = await program.methods
        .initialize()
        .accounts({
          memberAccount,
          vaultTokenAccount,
          mint: tokenMint.publicKey,
          signer: memberWallet.publicKey,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
          systemProgram: web3.SystemProgram.programId,
        })
        .signers([memberWallet])
        .rpc();
      // Confirm transaction
      await program.provider.connection.confirmTransaction(txHash);
      // Fetch the associated token account
      associatedTokenAccount = await spl.getAccount(
        program.provider.connection,
        vaultTokenAccount
      );
    });

    describe("member token vault account", () => {
      it("initialize member account", () => {
        assert(associatedTokenAccount.isInitialized);
        // assert(newAccount.memberCount.toNumber() === 0);
        // assert(newAccount.stakedAmount.toNumber() === 0);
        // assert(newAccount.baseRewardRate.toNumber() === 10);
        // assert(newAccount.rewardMultiplier.toNumber() === 10);
      });

      it("initialized associated token vault account", () => {
        assert(
          associatedTokenAccount.mint.equals(tokenMint.publicKey),
          `Invalid token vault mint ${associatedTokenAccount.mint.toBase58()}`
        );
      });

      it("owned by the program member account", () => {
        assert(
          associatedTokenAccount.owner.equals(memberAccount),
          `Illegal vault owner ${associatedTokenAccount.owner.toBase58()}`
        );
        // assert(associatedTokenAccount.closeAuthority.equals(memberAccount));
      });

      it("unassigned approval delegation", () => {
        assert(
          associatedTokenAccount.delegate === null,
          "Delegate is assigned"
        );
        assert(
          associatedTokenAccount.delegatedAmount === BigInt(0),
          "Delegated amount greater than zero"
        );
      });

      it.skip("has empty token vault balance", () => {
        assert(
          associatedTokenAccount.amount === BigInt(0),
          "Vault balance is greater than zero"
        );
      });

      it("fail direct token transfer authorization", async () => {
        await assertErrorAsync(
          spl.transferChecked(
            program.provider.connection,
            memberWallet,
            vaultTokenAccount,
            tokenMint.publicKey,
            memberWallet.publicKey,
            memberAccount,
            BigInt(0),
            DECIMALS
          ),
          "Signature verification failed"
        );
      });
    });

    describe.skip("member account state", () => {
      it.skip("created membership account", async () => {
        const membership = await program.account.memberAccount.fetch(
          memberAccount
        );
        assert(
          membership.totalAmount.eq(new BN(0)),
          "Account balance is greater than zero"
        );
      });

      it.skip("update global member count");
    });
  });

  describe("Deposit token mint into locker", () => {
    let memberAccount: web3.PublicKey,
      vaultTokenAccount: web3.PublicKey,
      sourceTokenAccount: web3.PublicKey,
      vaultStartingBalance: BN,
      amount = new BN(AMOUNT * Math.pow(10, DECIMALS));

    before(async () => {
      memberAccount = findMemberAccount(memberWallet);
      vaultTokenAccount = findVaultTokenAccountAddress(
        tokenMint.publicKey,
        memberWallet
      );

      sourceTokenAccount = spl.getAssociatedTokenAddressSync(
        tokenMint.publicKey,
        memberWallet.publicKey,
        false,
        spl.TOKEN_PROGRAM_ID,
        spl.ASSOCIATED_TOKEN_PROGRAM_ID
      );

      vaultStartingBalance = await getTokenBalance(vaultTokenAccount);
    });

    it("deposit tokens into vault", async () => {
      // Capture starting balances for comparison
      // const { stakedAmount: globalPoolvaultStartingBalance } =
      //   await program.account.stakeSystem.fetch(stakeSystem);
      // const { totalAmount: memberPoolStartingAmount } =
      //   await program.account.memberAccount.fetch(memberPool);
      // Send transaction
      const txHash = await program.methods
        .deposit(amount)
        .accounts({
          memberAccount,
          vaultTokenAccount,
          sourceTokenAccount,
          mint: tokenMint.publicKey,
          signer: memberWallet.publicKey,
          systemProgram: web3.SystemProgram.programId,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
        })
        .signers([memberWallet])
        .rpc();
      // Confirm transaction
      await program.provider.connection.confirmTransaction(txHash);

      assert(txHash);

      // Fetch the global pool and check updates
      // const { stakedAmount } = await program.account.stakeSystem.fetch(
      //   stakeSystem
      // );
      // assert(stakedAmount.eq(globalPoolvaultStartingBalance.add(amount)));
      // // Check the member pool updates
      // const { totalAmount } = await program.account.memberPool.fetch(
      //   memberPool
      // );
      // assert(totalAmount.eq(memberPoolStartingAmount.add(amount)));
    });

    it("final balance should increase by amount", async () => {
      const balance = await getTokenBalance(vaultTokenAccount);
      assert(
        balance.sub(vaultStartingBalance).eq(amount),
        "Invalid balance change"
      );
    });

    it.skip("recorded deposit timestamp and amount");

    it.skip("updated total locked token amount");

    it.skip("fails when amount is less than minimum");

    it.skip("fails when insufficient token balance");

    it("fail to directly withdraw tokens", async () => {
      await assertErrorAsync(
        spl.transferChecked(
          program.provider.connection,
          memberWallet,
          vaultTokenAccount,
          tokenMint.publicKey,
          memberWallet.publicKey,
          memberAccount,
          amount.toNumber(),
          DECIMALS
        ),
        "Signature verification failed"
      );
    });
  });

  describe.skip("Unlock tokens", () => {
    it.skip("does not reward unlocked tokens");
  });

  describe.skip("Withdraw tokens", () => {
    let memberAccount: web3.PublicKey,
      vaultTokenAccount: web3.PublicKey,
      sourceTokenAccount: web3.PublicKey,
      vaultStartingBalance: BN;

    const amount = new BN(AMOUNT * Math.pow(10, DECIMALS));

    before(async () => {
      // Derive the global and member pool address
      memberAccount = findMemberAccount(memberWallet);
      vaultTokenAccount = findVaultTokenAccountAddress(
        tokenMint.publicKey,
        memberWallet
      );

      sourceTokenAccount = await spl.getAssociatedTokenAddress(
        tokenMint.publicKey,
        memberWallet.publicKey,
        false,
        spl.TOKEN_PROGRAM_ID,
        spl.ASSOCIATED_TOKEN_PROGRAM_ID
      );

      vaultStartingBalance = await getTokenBalance(vaultTokenAccount);
    });

    it("withdraw tokens from vault", async () => {
      // Capture starting balances for comparison
      // const { stakedAmount: globalPoolvaultStartingBalance } =
      //   await program.account.stakeSystem.fetch(stakeSystem);
      // const { totalAmount: memberPoolStartingAmount } =
      //   await program.account.memberAccount.fetch(memberPool);
      // Send transaction
      const txHash = await program.methods
        .withdraw(amount)
        .accounts({
          memberAccount,
          vaultTokenAccount,
          sourceTokenAccount,
          mint: tokenMint.publicKey,
          signer: memberWallet.publicKey,
          systemProgram: web3.SystemProgram.programId,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
        })
        .signers([memberWallet])
        .rpc();
      // Confirm transaction
      await program.provider.connection.confirmTransaction(txHash);

      console.log(txHash, " ", memberAccount.toBase58());

      assert(txHash);
    });

    it("final balance should be less withdrawal amount", async () => {
      const balance = await getTokenBalance(vaultTokenAccount);
      assert(vaultStartingBalance.sub(balance).eq(amount));
    });

    it.skip("updates the locked slot balance");

    it.skip("releases the locked slot");

    it.skip("recorded deposit timestamp and amount");

    it.skip("updated total locked token amount");

    it.skip("fails when amount is greater than vault balance");

    it.skip("fails when vault is empty");

    it.skip("fails when slot is enabled");

    it.skip("fails when time within locked time period");
  });

  describe.skip("Disabling locked slot", () => {
    it.skip("updates the enabled state to false");
  });

  describe.skip("Enabling locked slot", () => {
    it("fails when ");
  });

  describe.skip("Claiming rewards", () => {
    // Should reward tokens for all the epochs the member might have missed since last claiming
    it.skip("rewards tokens accross multiple epochs");
    it.skip("update time rewarded after reward");
    it.skip("does not reward unlocked tokens");
  });

  describe("Self exclusion", () => {
    let memberAccount: web3.PublicKey;

    before(async () => {
      memberAccount = findMemberAccount(memberWallet);
    });

    it("fails to exclude member when not signer", async () => {
      console.log(memberAccount.toBase58());
      await assertErrorAsync(
        program.methods
          .exclude()
          .accounts({
            memberAccount,
            signer: adminWallet.publicKey,
          })
          .signers([adminWallet])
          .rpc(),
        "Forbidden exclusion signer"
      );
    });

    it("updates the member status when signer", async () => {
      const txHash = await program.methods
        .exclude()
        .accounts({
          memberAccount,
          signer: memberWallet.publicKey,
        })
        .signers([memberWallet])
        .rpc();
      // Confirm transaction
      await program.provider.connection.confirmTransaction(txHash);

      assert(txHash);
    });

    it.skip("fails to update status when excluded");
  });

  after(() => {
    /**
     * Allowing a disabled slot to be re-enabled opens up a
     * re-entry attack where the member recieves rewards and
     * is able to withdraw their tokens.
     */
    it.skip("disabled slots should never be enabled");
  });
});
