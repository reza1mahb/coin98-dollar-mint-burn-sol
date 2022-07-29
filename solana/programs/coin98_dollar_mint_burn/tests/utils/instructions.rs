use anchor_lang::*;
use anchor_lang::solana_program::system_program;
use solana_program::instruction::{Instruction, AccountMeta};
use solana_sdk::pubkey::Pubkey;
use coin98_dollar_mint_burn::constant::{
    APP_DATA_SEED_1,
    APP_DATA_SEED_2,
    ROOT_SIGNER_SEED_1,
    ROOT_SIGNER_SEED_2
};

const MINTER_SEEDS: &[u8] = &[121, 44, 123, 235, 166, 175, 64, 142];
const BURNER_SEEDS: &[u8] = &[240, 112, 187, 250, 94, 126, 188, 74];

pub const TOKEN_PROGRAM_ID: Pubkey = Pubkey::new_from_array([6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169]);

pub fn create_minter_instruction(root: &Pubkey, path: Vec<u8>) -> Instruction {
    let (minter, _): (Pubkey, u8) = find_minter_address(&path);

    let accounts = coin98_dollar_mint_burn::accounts::CreateMinterContext {
        root: *root,
        minter,
        system_program: system_program::id()
    }.to_account_metas(None);

    let data = coin98_dollar_mint_burn::instruction::CreateMinter {
        _derivation_path: path
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}

pub fn set_minter_instruction(
    root: &Pubkey, 
    minter: &Pubkey, 
    is_active: bool, 
    input_tokens: Vec<Pubkey>,
    input_decimals: Vec<u16>,
    input_percentages: Vec<u16>,
    input_price_feeds: Vec<Pubkey>,
    fee_percent: u16,
    total_minted_limit: u64,
    per_period_minted_limit: u64,
    min_amount: u64
) -> Instruction {
    let accounts = coin98_dollar_mint_burn::accounts::SetMinterContext {
        root: *root,
        minter: *minter,
    }.to_account_metas(None);

    let data = coin98_dollar_mint_burn::instruction::SetMinter {
        is_active,
        input_tokens,
        input_decimals,
        input_percentages,
        input_price_feeds,
        fee_percent,
        total_minted_limit,
        per_period_minted_limit,
        min_amount
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}

pub fn mint_instruction(
    user: &Pubkey, 
    cusd_mint: &Pubkey,
    minter: &Pubkey,
    recipient: &Pubkey,
    amount: u64,
    extra_instructions: Vec<u8>,
    extra_accounts: Vec<Pubkey>
) -> Instruction {
    let (app_data, _): (Pubkey, u8) = find_app_data_address();
    let (root_signer, _): (Pubkey, u8) = find_root_signer_address();

    let mut accounts = coin98_dollar_mint_burn::accounts::MintContext {
        user: *user,
        app_data,
        root_signer,
        cusd_mint: *cusd_mint,
        minter: *minter,
        recipient: *recipient,
        chainlink_program: chainlink_dfeed::id(),
        token_program: TOKEN_PROGRAM_ID
    }.to_account_metas(None);

    for account in extra_accounts.iter() {
        accounts.push(AccountMeta::new(*account, false));
    }

    let data = coin98_dollar_mint_burn::instruction::Mint {
        amount,
        extra_instructions
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}

pub fn create_burner_instruction(root: &Pubkey, path: Vec<u8>) -> Instruction {
    let (burner, _): (Pubkey, u8) = find_burner_address(&path);

    let accounts = coin98_dollar_mint_burn::accounts::CreateBurnerContext {
        root: *root,
        burner,
        system_program: system_program::id()
    }.to_account_metas(None);

    let data = coin98_dollar_mint_burn::instruction::CreateBurner {
        _derivation_path: path
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}

pub fn set_burner_instruction(
    root: &Pubkey,
    burner: &Pubkey,
    is_active: bool,
    output_token: Pubkey,
    output_decimals: u16,
    output_price_feed: Pubkey,
    fee_percent: u16,
    total_burned_limit: u64,
    per_period_burned_limit: u64,
    min_amount: u64
) -> Instruction {
    let accounts = coin98_dollar_mint_burn::accounts::SetBurnerContext {
        root: *root,
        burner: *burner,
    }.to_account_metas(None);

    let data = coin98_dollar_mint_burn::instruction::SetBurner {
        is_active,
        output_token,
        output_decimals,
        output_price_feed,
        fee_percent,
        total_burned_limit,
        per_period_burned_limit,
        min_amount
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}

pub fn burn_instruction(
    user: &Pubkey, 
    cusd_mint: &Pubkey,
    burner: &Pubkey,
    pool_cusd: &Pubkey,
    user_cusd: &Pubkey,
    extra_accounts: Vec<Pubkey>,
    amount: u64
) -> Instruction {
    let (app_data, _): (Pubkey, u8) = find_app_data_address();
    let (root_signer, _): (Pubkey, u8) = find_root_signer_address();

    let mut accounts = coin98_dollar_mint_burn::accounts::BurnContext {
        user: *user,
        app_data,
        root_signer,
        cusd_mint: *cusd_mint,
        burner: *burner,
        pool_cusd: *pool_cusd,
        user_cusd: *user_cusd,
        chainlink_program: chainlink_dfeed::id(),
        token_program: TOKEN_PROGRAM_ID
    }.to_account_metas(None);

    for account in extra_accounts.iter() {
        accounts.push(AccountMeta::new(*account, false));
    }

    let data = coin98_dollar_mint_burn::instruction::Burn {
        amount
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}


pub fn create_app_data_instruction(
    root: &Pubkey
) -> Instruction {
    let (app_data, _): (Pubkey, u8) = find_app_data_address();

    let accounts = coin98_dollar_mint_burn::accounts::CreateAppDataContext {
        root: *root,
        app_data,
        system_program: system_program::id()
    }.to_account_metas(None);

    let data = coin98_dollar_mint_burn::instruction::CreateAppData {
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}

pub fn set_app_data_instruction(
    root: &Pubkey,
    limit: u32
) -> Instruction {
    let (app_data, _): (Pubkey, u8) = find_app_data_address();

    let accounts = coin98_dollar_mint_burn::accounts::SetAppDataContext {
        root: *root,
        app_data,
    }.to_account_metas(None);

    let data = coin98_dollar_mint_burn::instruction::SetAppData {
        limit
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}

pub fn withdraw_token_instruction(
    root: &Pubkey,
    pool_token: &Pubkey,
    recipient_token: &Pubkey,
    amount: u64
) -> Instruction {
    let (app_data, _): (Pubkey, u8) = find_app_data_address();
    let (root_signer, _): (Pubkey, u8) = find_root_signer_address();

    let accounts = coin98_dollar_mint_burn::accounts::WithdrawTokenContext {
        root: *root,
        app_data,
        root_signer,
        pool_token: *pool_token,
        recipient_token: *recipient_token,
        token_program: TOKEN_PROGRAM_ID
    }.to_account_metas(None);

    let data = coin98_dollar_mint_burn::instruction::WithdrawToken {
        amount,
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}

pub fn unlock_token_mint_instruction(
    root: &Pubkey,
    token_mint: &Pubkey
) -> Instruction {
    let (app_data, _): (Pubkey, u8) = find_app_data_address();
    let (root_signer, _): (Pubkey, u8) = find_root_signer_address();

    let accounts = coin98_dollar_mint_burn::accounts::UnlockTokenMintContext {
        root: *root,
        app_data,
        root_signer,
        token_mint: *token_mint,
        token_program: TOKEN_PROGRAM_ID
    }.to_account_metas(None);

    let data = coin98_dollar_mint_burn::instruction::UnlockTokenMint {
    }
    .data();

    let instruction = Instruction {
        program_id: coin98_dollar_mint_burn::id(),
        data,
        accounts
    };

    instruction
}

pub fn find_minter_address(path: &Vec<u8>) -> (Pubkey, u8) {
    let seeds = &[MINTER_SEEDS, path];
    Pubkey::find_program_address(seeds, &coin98_dollar_mint_burn::id())
}

pub fn find_burner_address(path: &Vec<u8>) -> (Pubkey, u8) {
    let seeds = &[BURNER_SEEDS, path];
    Pubkey::find_program_address(seeds, &coin98_dollar_mint_burn::id())
}

pub fn find_app_data_address() -> (Pubkey, u8) {
    let seeds = &[APP_DATA_SEED_1, APP_DATA_SEED_2];
    Pubkey::find_program_address(seeds, &coin98_dollar_mint_burn::id())
}

pub fn find_root_signer_address() -> (Pubkey, u8) {
    let seeds = &[ROOT_SIGNER_SEED_1, ROOT_SIGNER_SEED_2];
    Pubkey::find_program_address(seeds, &coin98_dollar_mint_burn::id())
}
