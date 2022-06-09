use anchor_lang::prelude::*;
use crate::error::{
  ErrorCode,
};
use crate::state::{
  AppData,
  Burner,
  Minter,
};
use crate::external::anchor_token::{
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
      &[8, 201, 24, 140, 93, 100, 30, 148][..],
      &[15, 81, 173, 106, 105, 203, 253, 99][..],
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,

  /// CHECK: PDA as root authority of the program
  #[account(
    seeds = [
      &[2, 151, 229, 53, 244, 77, 229, 7][..],
      &[68, 203, 0, 94, 226, 230, 93, 156][..],
    ],
    bump,
  )]
  pub root_signer: AccountInfo<'info>,

  /// CHECK: CUSD Token Mint
  #[account(
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
      &[8, 201, 24, 140, 93, 100, 30, 148][..],
      &[15, 81, 173, 106, 105, 203, 253, 99][..],
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,

  /// CHECK: PDA as root authority of the program
  #[account(
    seeds = [
      &[2, 151, 229, 53, 244, 77, 229, 7][..],
      &[68, 203, 0, 94, 226, 230, 93, 156][..],
    ],
    bump,
  )]
  pub root_signer: AccountInfo<'info>,

  /// CHECK: CUSD Token Mint
  #[account(
    constraint = is_cusd_token_mint(&cusd_mint) @ErrorCode::InvalidAccount,
  )]
  pub cusd_mint: AccountInfo<'info>,

  #[account(mut)]
  pub burner: Account<'info, Burner>,

  /// CHECK: Pool CUSD token account
  #[account(
    constraint = pool_cusd.owner == root_signer.key() @ErrorCode::InvalidAccount,
    constraint = pool_cusd.mint == cusd_mint.key() @ErrorCode::InvalidAccount,
  )]
  #[account(mut)]
  pub pool_cusd: Account<'info, TokenAccount>,

  /// CHECK: Account to send token
  #[account(
    constraint = pool_token.owner == root_signer.key() @ErrorCode::InvalidAccount,
    constraint = pool_token.mint == burner.output_token @ErrorCode::InvalidAccount,
  )]
  #[account(mut)]
  pub pool_token: Account<'info, TokenAccount>,

  /// CHECK: User CUSD token account
  #[account(
    constraint = user_cusd.mint == cusd_mint.key() @ErrorCode::InvalidAccount,
  )]
  #[account(mut)]
  pub user_cusd: Account<'info, TokenAccount>,

  /// CHECK: Account to receive token
  #[account(
    mut,
    constraint = user_token.mint == burner.output_token @ErrorCode::InvalidAccount,
  )]
  pub user_token: Account<'info, TokenAccount>,

  /// CHECK: Chainlink program
  #[account(
    constraint = is_chainlink_program(&chainlink_program) @ErrorCode::InvalidAccount,
  )]
  pub chainlink_program: AccountInfo<'info>,

  /// CHECK: Price feed for output token
  #[account(
    constraint = price_feed.key() == burner.output_price_feed @ErrorCode::InvalidAccount,
  )]
  pub price_feed: AccountInfo<'info>,

  /// CHECK: Solana native Token Program
  #[account(
    constraint = is_token_program(&token_program) @ErrorCode::InvalidAccount,
  )]
  pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct WithdrawTokenContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer, mut)]
  pub root: AccountInfo<'info>,

  #[account(
    seeds = [
      &[8, 201, 24, 140, 93, 100, 30, 148][..],
      &[15, 81, 173, 106, 105, 203, 253, 99][..],
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,

  /// CHECK: PDA as root authority of the program
  #[account(
    seeds = [
      &[2, 151, 229, 53, 244, 77, 229, 7][..],
      &[68, 203, 0, 94, 226, 230, 93, 156][..],
    ],
    bump,
  )]
  pub root_signer: AccountInfo<'info>,

  #[account(
    constraint = pool_token.owner == root_signer.key() @ErrorCode::InvalidAccount,
  )]
  pub pool_token: Account<'info, TokenAccount>,

  pub recipient_token: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct UnlockTokenMintContext<'info> {

  /// CHECK: program owner, verified using #access_control
  #[account(signer, mut)]
  pub root: AccountInfo<'info>,

  #[account(
    seeds = [
      &[8, 201, 24, 140, 93, 100, 30, 148][..],
      &[15, 81, 173, 106, 105, 203, 253, 99][..],
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,

  /// CHECK: PDA as root authority of the program
  #[account(
    seeds = [
      &[2, 151, 229, 53, 244, 77, 229, 7][..],
      &[68, 203, 0, 94, 226, 230, 93, 156][..],
    ],
    bump,
  )]
  pub root_signer: AccountInfo<'info>,

  /// CHECK: TokenMint under root_signer authority
  #[account(
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
      &[8, 201, 24, 140, 93, 100, 30, 148][..],
      &[15, 81, 173, 106, 105, 203, 253, 99][..],
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
      &[8, 201, 24, 140, 93, 100, 30, 148][..],
      &[15, 81, 173, 106, 105, 203, 253, 99][..],
    ],
    bump = app_data.nonce,
  )]
  pub app_data: Account<'info, AppData>,
}
