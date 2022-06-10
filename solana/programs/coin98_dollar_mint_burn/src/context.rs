use anchor_lang::prelude::*;
use crate::constant::{
  APP_DATA_SEED_1,
  APP_DATA_SEED_2,
  ROOT_SIGNER_SEED_1,
  ROOT_SIGNER_SEED_2,
};
use crate::error::{
  ErrorCode,
};
use crate::state::{
  AppData,
  Burner,
  Minter,
};
use crate::external::anchor_spl_token::{
  TokenAccount,
  TokenMint,
};
use crate::external::chainlink_solana::{
  is_chainlink_program,
};
use crate::external::cusd_token_mint::{
  is_cusd_token_mint,
};
use crate::external::spl_token::{
  is_token_program,
};

#[derive(Accounts)]
#[instruction(derivation_path: Vec<u8>)]
pub struct CreateMinterContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer, mut)]
  pub root: AccountInfo<'info>,

  #[account(
    init,
    seeds = [
      &[121, 44, 123, 235, 166, 175, 64, 142],
      &*derivation_path,
    ],
    bump,
    payer = root,
    space = 16 + Minter::size(8),
  )]
  pub minter: Account<'info, Minter>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetMinterContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(mut)]
  pub minter: Account<'info, Minter>,
}

#[derive(Accounts)]
#[instruction(derivation_path: Vec<u8>)]
pub struct CreateBurnerContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer, mut)]
  pub root: AccountInfo<'info>,

  #[account(
    init,
    seeds = [
      &[240, 112, 187, 250, 94, 126, 188, 74],
      &*derivation_path,
    ],
    bump,
    payer = root,
    space = 16 + Burner::LEN,
  )]
  pub burner: Account<'info, Burner>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetBurnerContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(mut)]
  pub burner: Account<'info, Burner>,
}

#[derive(Accounts)]
pub struct MintContext<'info> {

  /// CHECK: user account
  #[account(signer)]
  pub user: AccountInfo<'info>,

  #[account(
    seeds = [
      APP_DATA_SEED_1,
      APP_DATA_SEED_2,
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,

  /// CHECK: PDA as root authority of the program
  #[account(
    seeds = [
      ROOT_SIGNER_SEED_1,
      ROOT_SIGNER_SEED_2,
    ],
    bump = app_data.signer_nonce,
  )]
  pub root_signer: AccountInfo<'info>,

  /// CHECK: CUSD Token Mint
  #[account(
    mut,
    constraint = is_cusd_token_mint(&cusd_mint) @ErrorCode::InvalidAccount,
  )]
  pub cusd_mint: AccountInfo<'info>,

  #[account(mut)]
  pub minter: Account<'info, Minter>,

  /// CHECK: Account to receive CUSD
  #[account(mut)]
  pub recipient: AccountInfo<'info>,

  /// CHECK: Chainlink program
  #[account(
    constraint = is_chainlink_program(&chainlink_program) @ErrorCode::InvalidAccount,
  )]
  pub chainlink_program: AccountInfo<'info>,

  /// CHECK: Solana native Token Program
  #[account(
    constraint = is_token_program(&token_program) @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct BurnContext<'info> {

  /// CHECK: user account
  #[account(signer)]
  pub user: AccountInfo<'info>,

  #[account(
    seeds = [
      APP_DATA_SEED_1,
      APP_DATA_SEED_2,
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,

  /// CHECK: PDA as root authority of the program
  #[account(
    seeds = [
      ROOT_SIGNER_SEED_1,
      ROOT_SIGNER_SEED_2,
    ],
    bump = app_data.signer_nonce,
  )]
  pub root_signer: AccountInfo<'info>,

  /// CHECK: CUSD Token Mint
  #[account(
    mut,
    constraint = is_cusd_token_mint(&cusd_mint) @ErrorCode::InvalidAccount,
  )]
  pub cusd_mint: AccountInfo<'info>,

  #[account(mut)]
  pub burner: Account<'info, Burner>,

  /// CHECK: Pool CUSD token account
  #[account(
    mut,
    constraint = pool_cusd.owner == root_signer.key() @ErrorCode::InvalidAccount,
    constraint = pool_cusd.mint == cusd_mint.key() @ErrorCode::InvalidAccount,
  )]
  pub pool_cusd: Account<'info, TokenAccount>,

  /// CHECK: User CUSD token account
  #[account(
    mut,
    constraint = user_cusd.mint == cusd_mint.key() @ErrorCode::InvalidAccount,
  )]
  pub user_cusd: Account<'info, TokenAccount>,

  /// CHECK: Chainlink program
  #[account(
    constraint = is_chainlink_program(&chainlink_program) @ErrorCode::InvalidAccount,
  )]
  pub chainlink_program: AccountInfo<'info>,

  /// CHECK: Solana native Token Program
  #[account(
    constraint = is_token_program(&token_program) @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawTokenContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(
    seeds = [
      APP_DATA_SEED_1,
      APP_DATA_SEED_2,
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,

  /// CHECK: PDA as root authority of the program
  #[account(
    seeds = [
      ROOT_SIGNER_SEED_1,
      ROOT_SIGNER_SEED_2,
    ],
    bump = app_data.signer_nonce,
  )]
  pub root_signer: AccountInfo<'info>,

  #[account(
    mut,
    constraint = pool_token.owner == root_signer.key() @ErrorCode::InvalidAccount,
  )]
  pub pool_token: Account<'info, TokenAccount>,

  #[account(mut)]
  pub recipient_token: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct UnlockTokenMintContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(
    seeds = [
      APP_DATA_SEED_1,
      APP_DATA_SEED_2,
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,

  /// CHECK: PDA as root authority of the program
  #[account(
    seeds = [
      ROOT_SIGNER_SEED_1,
      ROOT_SIGNER_SEED_2,
    ],
    bump = app_data.signer_nonce,
  )]
  pub root_signer: AccountInfo<'info>,

  /// CHECK: TokenMint under root_signer authority
  #[account(
    mut,
    constraint = token_mint.mint_authority.contains(&root_signer.key()) @ErrorCode::InvalidAccount,
  )]
  pub token_mint: Account<'info, TokenMint>,
}

#[derive(Accounts)]
pub struct CreateAppDataContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer, mut)]
  pub root: AccountInfo<'info>,

  #[account(
    init,
    seeds = [
      APP_DATA_SEED_1,
      APP_DATA_SEED_2,
    ],
    bump,
    payer = root,
    space = 16 + AppData::LEN,
  )]
  pub app_data: Account<'info, AppData>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAppDataContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer)]
  pub root: AccountInfo<'info>,

  #[account(
    mut,
    seeds = [
      APP_DATA_SEED_1,
      APP_DATA_SEED_2,
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,
}
