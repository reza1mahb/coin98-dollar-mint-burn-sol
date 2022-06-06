pub mod constant;
pub mod context;
pub mod error;
pub mod state;

use anchor_lang::prelude::*;
use crate::constant::{
  ROOT_KEYS,
};
use crate::context::*;
use crate::error::{
  ErrorCode,
};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod coin98_dollar_mint_burn {
  use super::*;

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn create_minter(
    ctx: Context<CreateMinterContext>,
    _derivation_path: Vec<u8>,
  ) -> Result<()> {

    let minter = &mut ctx.accounts.minter;
    minter.nonce = *ctx.bumps.get("minter").unwrap();
    minter.is_active = false;
    minter.input_tokens = Vec::new();
    minter.input_decimals = Vec::new();
    minter.input_percentages = Vec::new();
    minter.input_price_feeds = Vec::new();

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn set_minter(
    ctx: Context<SetMinterContext>,
    is_active: bool,
    input_tokens: Vec<Pubkey>,
    input_decimals: Vec<u8>,
    input_percentages: Vec<u16>,
    input_price_feeds: Vec<Pubkey>,
    total_minted_limit: u64,
    per_period_minted_limit: u64,
    fee_percent: u16,
  ) -> Result<()> {

    if input_tokens.len() != input_decimals.len() {
      return Err(ErrorCode::InvalidInput.into());
    }
    if input_tokens.len() != input_percentages.len() {
      return Err(ErrorCode::InvalidInput.into());
    }
    if input_tokens.len() != input_price_feeds.len() {
      return Err(ErrorCode::InvalidInput.into());
    }
    let percentage: u16 = input_percentages.iter().sum();
    if percentage != 10000 {
      return Err(ErrorCode::InvalidInput.into());
    }
    if fee_percent > 10000 {
      return Err(ErrorCode::InvalidInput.into());
    }

    let minter = &mut ctx.accounts.minter;
    minter.is_active = is_active;
    minter.input_tokens = input_tokens;
    minter.input_decimals = input_decimals;
    minter.input_percentages = input_percentages;
    minter.input_price_feeds = input_price_feeds;
    minter.fee_percent = fee_percent;
    minter.total_minted_limit = total_minted_limit;
    minter.per_period_minted_limit = per_period_minted_limit;

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn create_burner(
    ctx: Context<CreateBurnerContext>,
    _derivation_path: Vec<u8>,
  ) -> Result<()> {

    let burner = &mut ctx.accounts.burner;
    burner.nonce = *ctx.bumps.get("burner").unwrap();
    burner.is_active = false;

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn set_burner(
    ctx: Context<SetBurnerContext>,
    is_active: bool,
    output_token: Pubkey,
    output_decimals: u8,
    output_price_feed: Pubkey,
    fee_percent: u16,
    total_burned_limit: u64,
    per_period_burned_limit: u64,
  ) -> Result<()> {

    if fee_percent > 10000 {
      return Err(ErrorCode::InvalidInput.into());
    }

    let burner = &mut ctx.accounts.burner;
    burner.is_active = is_active;
    burner.output_token = output_token;
    burner.output_decimals = output_decimals;
    burner.output_price_feed = output_price_feed;
    burner.fee_percent = fee_percent;
    burner.total_burned_limit = total_burned_limit;
    burner.per_period_burned_limit = per_period_burned_limit;

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn create_app_data(
    ctx: Context<CreateAppDataContext>,
  ) -> Result<()> {

    let app_data = &mut ctx.accounts.app_data;
    app_data.nonce = *ctx.bumps.get("app_data").unwrap();
    app_data.limit = 24;

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn set_app_data(
    ctx: Context<SetAppDataContext>,
    limit: u64,
  ) -> Result<()> {

    let app_data = &mut ctx.accounts.app_data;
    app_data.limit = limit;

    Ok(())
  }
}

pub fn is_root(user: Pubkey) -> Result<()> {
  let user_key = user.to_string();
  let result = ROOT_KEYS.iter().position(|&key| key == &user_key[..]);
  if result == None {
    return Err(ErrorCode::Unauthorized.into());
  }

  Ok(())
}
