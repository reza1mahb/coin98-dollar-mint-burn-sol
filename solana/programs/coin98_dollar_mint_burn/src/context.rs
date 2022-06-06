use anchor_lang::prelude::*;
use crate::state::{
  AppData,
  Burner,
  Minter,
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
