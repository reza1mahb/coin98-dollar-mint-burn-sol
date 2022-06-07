pub mod constant;
pub mod context;
pub mod error;
pub mod state;
pub mod external;

use anchor_lang::prelude::*;
use crate::constant::{
  ROOT_KEYS,
};
use crate::context::*;
use crate::error::{
  ErrorCode,
};
use crate::external::anchor_spl::{
  mint_token,
  transfer_token,
};
use crate::external::chainlink_solana::{
  get_price_feed,
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

  pub fn mint<'i>(
    ctx: Context<'_, '_, '_, 'i, MintContext<'i>>,
    amount: u64,
    extra_instructions: Vec<u8>,
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let app_data = &ctx.accounts.app_data;
    let minter = &ctx.accounts.minter;

    if !minter.is_active {
      return Err(ErrorCode::Unavailable.into());
    }

    let current_timestamp = Clock::get().unwrap().unix_timestamp;
    let timestamp_per_period = app_data.limit * 3600;
    let is_in_period = minter.last_period_timestamp + timestamp_per_period < current_timestamp;
    let current_period_minted_amount = if is_in_period { minter.per_period_minted_amount } else { 0u64 };

    if current_period_minted_amount + amount > minter.per_period_minted_limit {
      return Err(ErrorCode::LimitReached.into());
    }
    if minter.total_minted_amount + amount > minter.total_minted_limit {
      return Err(ErrorCode::LimitReached.into());
    }

    let chainlink_program = &ctx.accounts.chainlink_program;
    let accounts = &ctx.remaining_accounts;

    let account_indices: Vec<usize> = extra_instructions.iter()
      .map(|extra| {
        usize::from(*extra)
      })
      .collect();

    for (i, _address) in minter.input_tokens.iter().enumerate() {
      let price_feed = &accounts[i];
      let (price, precision) = get_price_feed(
        &*chainlink_program,
        &*price_feed,
      );
      let value_contrib = minter.input_percentages[i];

      let input_amount = amount.checked_mul(u64::from(value_contrib)).unwrap().checked_div(10000).unwrap()
        .checked_mul(precision).unwrap().checked_div(price).unwrap();

      let from_account_index = account_indices[2*i + 1];
      let to_account_index = account_indices[2*i + 2];
      transfer_token(
          &*user,
          &accounts[from_account_index],
          &accounts[to_account_index],
          input_amount,
          &[],
        )
        .expect("CUSD Factory: CPI failed.");
    }

    let minter = &mut ctx.accounts.minter;
    minter.total_minted_amount = minter.total_minted_amount + amount;
    minter.per_period_minted_limit = minter.total_minted_amount + current_period_minted_amount + amount;
    if !is_in_period {
      minter.last_period_timestamp = current_timestamp;
    }

    let cusd_mint = &ctx.accounts.cusd_mint;
    let recipient = &ctx.accounts.recipient;
    let root_signer = &ctx.accounts.root_signer;

    let seeds: &[&[u8]] = &[
      &[2, 151, 229, 53, 244, 77, 229, 7][..],
      &[68, 203, 0, 94, 226, 230, 93, 156][..],
      &[app_data.signer_nonce],
    ];

    mint_token(
        &*cusd_mint,
        &*recipient,
        &*root_signer,
        amount,
        &[&seeds],
      )
      .expect("CUSD Factory: CPI failed.");

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn create_app_data(
    ctx: Context<CreateAppDataContext>,
  ) -> Result<()> {

    let app_data = &mut ctx.accounts.app_data;
    app_data.nonce = *ctx.bumps.get("app_data").unwrap();
    let (_, signer_nonce) = Pubkey::find_program_address(
      &[
        &[2, 151, 229, 53, 244, 77, 229, 7][..],
        &[68, 203, 0, 94, 226, 230, 93, 156][..],
      ],
      ctx.program_id,
    );
    app_data.signer_nonce = signer_nonce;
    app_data.limit = 24;

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn set_app_data(
    ctx: Context<SetAppDataContext>,
    limit: i64,
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
