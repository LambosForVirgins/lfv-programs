#[cfg(test)]
mod refund_tests {
    use super::*;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::{
        system_program, system_instruction, rent::Rent, program_pack::Pack,
    };
    use anchor_spl::token::spl_token;
    use spl_token::instruction as token_instruction;
    use anchor_lang::solana_program_test::*;
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
        instruction::{AccountMeta, Instruction},
    };

    #[tokio::test]
    async fn test_burn_and_close_rent_refund_to_vault() {
        // Create a program test with just the SPL Token program
        // This tests the same CloseAccount behavior that burn_and_close uses
        let program_test = ProgramTest::default();
        
        // Start test environment
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
        
        // Create test accounts
        let owner = Keypair::new();
        let mint_authority = Keypair::new();
        let vault = Keypair::new();
        let mint = Keypair::new();
        let token_account = Keypair::new();
        
        let rent = Rent::default();
        let token_account_rent = rent.minimum_balance(spl_token::state::Account::LEN);
        let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);
        
        // Fund accounts
        let mut transaction = Transaction::new_with_payer(
            &[
                system_instruction::create_account(
                    &payer.pubkey(),
                    &owner.pubkey(),
                    1_000_000_000, // 1 SOL
                    0,
                    &system_program::id(),
                ),
                system_instruction::create_account(
                    &payer.pubkey(),
                    &vault.pubkey(),
                    1_000_000, // 0.001 SOL initial balance
                    0,
                    &system_program::id(),
                ),
                system_instruction::create_account(
                    &payer.pubkey(),
                    &mint.pubkey(),
                    mint_rent,
                    spl_token::state::Mint::LEN as u64,
                    &spl_token::id(),
                ),
                system_instruction::create_account(
                    &payer.pubkey(),
                    &token_account.pubkey(),
                    token_account_rent,
                    spl_token::state::Account::LEN as u64,
                    &spl_token::id(),
                ),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &owner, &vault, &mint, &token_account], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
        
        // Initialize mint
        let mut transaction = Transaction::new_with_payer(
            &[token_instruction::initialize_mint(
                &spl_token::id(),
                &mint.pubkey(),
                &mint_authority.pubkey(),
                None,
                9, // 9 decimals
            ).unwrap()],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
        
        // Initialize token account
        let mut transaction = Transaction::new_with_payer(
            &[token_instruction::initialize_account(
                &spl_token::id(),
                &token_account.pubkey(),
                &mint.pubkey(),
                &owner.pubkey(),
            ).unwrap()],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
        
        // Mint tokens to the token account
        let token_amount = 1_000_000_000u64; // 1 token with 9 decimals
        let mut transaction = Transaction::new_with_payer(
            &[token_instruction::mint_to(
                &spl_token::id(),
                &mint.pubkey(),
                &token_account.pubkey(),
                &mint_authority.pubkey(),
                &[],
                token_amount,
            ).unwrap()],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &mint_authority], recent_blockhash);
        banks_client.process_transaction(transaction).await.unwrap();
        
        // Record initial balances
        let vault_account_before = banks_client.get_account(vault.pubkey()).await.unwrap().unwrap();
        let owner_account_before = banks_client.get_account(owner.pubkey()).await.unwrap().unwrap();
        let initial_vault_balance = vault_account_before.lamports;
        let initial_owner_balance = owner_account_before.lamports;
        
        // Call burn_and_close
        let sui_receiver = [0x42u8; 32];
        let burn_and_close_instruction = Instruction {
            program_id: crate::id(),
            accounts: vec![
                AccountMeta::new(owner.pubkey(), true),
                AccountMeta::new(token_account.pubkey(), false),
                AccountMeta::new(mint.pubkey(), false),
                AccountMeta::new(vault.pubkey(), false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
            data: {
                let mut data = vec![0u8; 1 + 32]; // instruction discriminator + sui_address
                data[0] = 0; // Assuming burn_and_close is the first instruction
                data[1..33].copy_from_slice(&sui_receiver);
                data
            },
        };
        
        let mut transaction = Transaction::new_with_payer(
            &[burn_and_close_instruction],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer, &owner], recent_blockhash);
        
        // Execute burn_and_close
        banks_client.process_transaction(transaction).await.unwrap();
        
        // Verify results
        let vault_account_after = banks_client.get_account(vault.pubkey()).await.unwrap().unwrap();
        let owner_account_after = banks_client.get_account(owner.pubkey()).await.unwrap().unwrap();
        let final_vault_balance = vault_account_after.lamports;
        let final_owner_balance = owner_account_after.lamports;
        
        // Calculate balance changes
        let vault_balance_change = final_vault_balance - initial_vault_balance;
        let owner_balance_change = final_owner_balance - initial_owner_balance;
        
        // Verify rent refund went to vault
        assert_eq!(vault_balance_change, token_account_rent,
                  "Vault should receive exactly the token account rent refund");
        
        // Verify owner did not receive rent refund
        assert_eq!(owner_balance_change, 0,
                  "Owner should not receive any rent refund");
        
        // Verify token account was closed
        let token_account_after = banks_client.get_account(token_account.pubkey()).await.unwrap();
        assert!(token_account_after.is_none(),
                "Token account should be closed after burn_and_close");
        
        // Additional verifications
        assert!(vault_balance_change > 0, "Vault balance should increase");
        assert_eq!(owner_balance_change, 0, "Owner balance should not change");
        assert!(vault_balance_change > 2_000_000, "Rent refund should be substantial");
        
        println!("✓ Successfully called burn_and_close function");
        println!("✓ Verified rent refund goes to vault: {} lamports", vault_balance_change);
        println!("✓ Verified owner receives no rent refund: {} lamports", owner_balance_change);
        println!("✓ Verified token account was closed");
        println!("✓ Initial vault balance: {} lamports", initial_vault_balance);
        println!("✓ Final vault balance: {} lamports", final_vault_balance);
    }
} 