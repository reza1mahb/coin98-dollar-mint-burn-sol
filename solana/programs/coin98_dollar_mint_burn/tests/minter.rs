#![cfg(feature = "unit-test")]
pub mod utils;

use anchor_lang::*;
use std::{assert_eq, result::Result};
pub use solana_sdk::{
    pubkey::Pubkey,
    instruction::InstructionError,
    signature::{Keypair, Signer},
    transaction::{Transaction, TransactionError},
    transport::TransportError,
};
use solana_program_test::{*, ProgramTestContext};
use std::time::*;
use utils::helper::*;
use utils::wallet::*;
use utils::instructions::*;
use utils::chainlink::*;

#[tokio::test]
async fn create_minter() {
    let mut context = coin98_dollar_mint_burn_program_test().start_with_context().await;

    let payer_wallet = get_default_wallet().unwrap();
    airdrop(&mut context, &payer_wallet.pubkey(), 10_000_000_000).await.unwrap();
    
    let feed_path: Vec<u8> = (0..10).map(|_| { rand::random::<u8>() }).collect(); 

    let (root_signer, _): (Pubkey, u8) = find_root_signer_address();

    let c98_mint = Keypair::new();
    create_mint(&mut context, &c98_mint, &payer_wallet.pubkey(), None).await.unwrap();
    let cusd_mint = Keypair::from_bytes(&[202,192,162,73,184,144,236,61,88,204,128,42,118,116,110,72,153,114,57,183,67,59,239,160,46,130,112,92,219,145,116,21,171,46,92,155,111,121,107,137,187,201,219,116,208,23,156,137,19,146,184,45,122,164,241,252,184,1,174,7,13,160,189,174]).unwrap();
    create_mint(&mut context, &cusd_mint, &root_signer, Some(&root_signer)).await.unwrap();
    
    let payer_c98_token_account = create_associated_token_account(&mut context, &payer_wallet.pubkey(), &c98_mint.pubkey()).await.unwrap();
    let pool_c98_token_account = create_associated_token_account(&mut context, &root_signer, &c98_mint.pubkey()).await.unwrap();
    let pool_cusd_token_account = create_associated_token_account(&mut context, &root_signer, &cusd_mint.pubkey()).await.unwrap();
    let payer_cusd_token_account = create_associated_token_account(&mut context, &payer_wallet.pubkey(), &cusd_mint.pubkey()).await.unwrap();

    let create_app_data = create_app_data_instruction(&payer_wallet.pubkey());
    let set_app_data = set_app_data_instruction(&payer_wallet.pubkey(), 1_000_000_000);
    process_transaction(&mut context, &Vec::from([create_app_data, set_app_data]), &Vec::from([&payer_wallet])).await.unwrap();

    mint_tokens(&mut context, &c98_mint.pubkey(), &payer_c98_token_account, 1_000_000_000_000, &payer_wallet.pubkey(), Some(&payer_wallet)).await.unwrap();

    let (c98_feed, _): (Pubkey, u8) = find_feed_address(&feed_path);
    let create_feed = create_feed_instruction(&payer_wallet.pubkey(), feed_path, 25, 75, "C98-USD".to_string(), 6, 10);
    let submit_feed = submit_feed_instruction(&payer_wallet.pubkey(), &c98_feed, Instant::now().elapsed().as_secs().try_into().unwrap(), 1000000);
    process_transaction(&mut context, &Vec::from([create_feed, submit_feed]), &Vec::from([&payer_wallet])).await.unwrap();

    let minter_path = (0..10).map(|_| { rand::random::<u8>() }).collect();
    let (minter, _) = find_minter_address(&minter_path);
    let create_minter = create_minter_instruction(&payer_wallet.pubkey(), minter_path);
    let set_minter = set_minter_instruction(&payer_wallet.pubkey(), &minter, true, Vec::from([c98_mint.pubkey()]), Vec::from([0]), Vec::from([10000]), Vec::from([c98_feed.clone()]), 0, 1_000_000_000_000u64, 1_000_000_000_000u64, 0);
    process_transaction(&mut context, &Vec::from([create_minter, set_minter]), &Vec::from([&payer_wallet])).await.unwrap();

    let extra_instructions: Vec<u8> = Vec::from([0, 1, 2]);
    let mint = mint_instruction(&payer_wallet.pubkey(), &cusd_mint.pubkey(), &minter, &payer_cusd_token_account, 1_000_000_000_000, extra_instructions, Vec::from([c98_feed, payer_c98_token_account, pool_c98_token_account]));
    process_transaction(&mut context, &Vec::from([mint]), &Vec::from([&payer_wallet])).await.unwrap();

    // burn
    mint_tokens(&mut context, &c98_mint.pubkey(), &pool_c98_token_account, 1_000_000_000_000, &payer_wallet.pubkey(), Some(&payer_wallet)).await.unwrap();

    let burner_path = (0..10).map(|_| { rand::random::<u8>() }).collect();
    let (burner, _) = find_burner_address(&burner_path);
    let create_burner  = create_burner_instruction(&payer_wallet.pubkey(), burner_path);
    let set_burner = set_burner_instruction(&payer_wallet.pubkey(), &burner, true, c98_mint.pubkey(), 0, c98_feed.clone(), 0, 1_000_000_000_000u64, 1_000_000_000_000u64, 0);
    process_transaction(&mut context, &Vec::from([create_burner, set_burner]), &Vec::from([&payer_wallet])).await.unwrap();

    let burn = burn_instruction(&payer_wallet.pubkey(), &cusd_mint.pubkey(), &burner, &pool_cusd_token_account, &payer_cusd_token_account, Vec::from([c98_feed, pool_c98_token_account, payer_c98_token_account]), 1_000);
    process_transaction(&mut context, &Vec::from([burn]), &Vec::from([&payer_wallet])).await.unwrap();
}
