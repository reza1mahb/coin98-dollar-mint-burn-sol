use anchor_lang::prelude::*;
use crate::state::{
  AppData,
};

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
