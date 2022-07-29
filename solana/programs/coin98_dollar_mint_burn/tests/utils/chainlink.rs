use anchor_lang::*;
use anchor_lang::solana_program::system_program;
use solana_program::instruction::{Instruction, AccountMeta};
use solana_sdk::pubkey::Pubkey;

const FEED_SEEDS: &[u8] = &[57, 108, 60, 177, 143, 129, 26, 24];

pub fn create_feed_instruction(
    authority: &Pubkey,
    path: Vec<u8>,
    live_length: u8,
    history_length: u8,
    description: String,
    decimals: u8,
    granularity: u8,
) -> Instruction {
    let (feed, _): (Pubkey, u8) = find_feed_address(&path);

    let accounts = chainlink_dfeed::accounts::CreateFeedContext {
        authority: *authority,
        feed,
        system_program: system_program::id()
    }.to_account_metas(None);

    let data = chainlink_dfeed::instruction::CreateFeed {
        _derivation_path: path,
        live_length,
        _history_length: history_length,
        description,
        decimals,
        granularity
    }
    .data();

    let instruction = Instruction {
        program_id: chainlink_dfeed::id(),
        data,
        accounts
    };

    instruction
}

pub fn submit_feed_instruction(
    authority: &Pubkey,
    feed: &Pubkey,
    timestamp: i64,
    answer: i128
) -> Instruction {
    let accounts = chainlink_dfeed::accounts::SubmitFeedContext {
        authority: *authority,
        feed: *feed,
    }.to_account_metas(None);

    let data = chainlink_dfeed::instruction::SubmitFeed {
        timestamp,
        answer
    }
    .data();

    let instruction = Instruction {
        program_id: chainlink_dfeed::id(),
        data,
        accounts
    };

    instruction
}

pub fn find_feed_address(path: &Vec<u8>) -> (Pubkey, u8) {
    let seeds = &[FEED_SEEDS, path];
    Pubkey::find_program_address(seeds, &chainlink_dfeed::id())
}
