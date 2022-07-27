pub mod constant;
pub mod context;
pub mod error;
pub mod event;
pub mod state;
pub mod external;

use anchor_lang::prelude::*;
use solana_program::{
  program_pack::{
    Pack,
  },
  system_program,
};
use crate::constant::{
  CUSD_PRECISION,
  ROOT_KEYS,
  ROOT_SIGNER_SEED_1,
  ROOT_SIGNER_SEED_2,
  SYSTEM_FEE_CAP,
};
use crate::context::*;
use crate::error::{
  ErrorCode,
};
use crate::event::*;
use crate::external::anchor_spl_token::{
  burn_token,
  mint_token,
  transfer_authority,
  transfer_token,
};
use crate::external::chainlink_solana::{
  get_price_feed,
};
use crate::external::spl_token::{
  TokenAccount,
};

#[cfg(feature = "localhost")]
declare_id!("CFvHYH4afBtK97rAwKkZtpnEQGqx8AmS6SWmYZd6JdmE");

#[cfg(feature = "devnet")]
declare_id!("CF1Xn3Sx1M6KMvtD8zcQBmtDXSFaW7nttRLkYck5f6bR");

#[cfg(all(not(feature = "localhost"), not(feature = "devnet")))]
declare_id!("CDMBw8drd8Ypct1YDTyJiUHN3KqP9tqtJJxjym5p3muQ");

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

    emit!(CreateMinterEvent {
      is_active: minter.is_active,
    });

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn set_minter(
    ctx: Context<SetMinterContext>,
    is_active: bool,
    input_tokens: Vec<Pubkey>,
    input_decimals: Vec<u16>,
    input_percentages: Vec<u16>,
    input_price_feeds: Vec<Pubkey>,
    fee_percent: u16,
    total_minted_limit: u64,
    per_period_minted_limit: u64,
    min_amount: u64,
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
    if fee_percent > SYSTEM_FEE_CAP {
      return Err(ErrorCode::InvalidInput.into());
    }

    let minter = &mut ctx.accounts.minter;
    minter.is_active = is_active;
    minter.input_tokens = input_tokens.clone();
    minter.input_decimals = input_decimals.clone();
    minter.input_percentages = input_percentages.clone();
    minter.input_price_feeds = input_price_feeds.clone();
    minter.fee_percent = fee_percent;
    minter.total_minted_limit = total_minted_limit;
    minter.per_period_minted_limit = per_period_minted_limit;
    minter.min_amount = min_amount;

    emit!(SetMinterEvent {
      is_active,
      input_tokens,
      input_decimals,
      input_percentages,
      input_price_feeds,
      fee_percent,
      total_minted_limit,
      per_period_minted_limit,
      min_amount,
    });

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

    emit!(CreateBurnerEvent {
      is_active: burner.is_active,
    });

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn set_burner(
    ctx: Context<SetBurnerContext>,
    is_active: bool,
    output_token: Pubkey,
    output_decimals: u16,
    output_price_feed: Pubkey,
    fee_percent: u16,
    total_burned_limit: u64,
    per_period_burned_limit: u64,
    min_amount: u64,
  ) -> Result<()> {

    if fee_percent > SYSTEM_FEE_CAP {
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
    burner.min_amount = min_amount;

    emit!(SetBurnerEvent {
      is_active,
      output_token,
      output_decimals,
      output_price_feed,
      fee_percent,
      total_burned_limit,
      per_period_burned_limit,
      min_amount,
    });

    Ok(())
  }

pub fn mint<'a>(
    ctx: Context<'_, '_, '_, 'a, MintContext<'a>>,
    amount: u64, // amount of CUSD user want to mint
    extra_instructions: Vec<u8>,
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let app_data = &ctx.accounts.app_data;
    let root_signer = &ctx.accounts.root_signer;
    let minter = &ctx.accounts.minter;

    if amount < minter.min_amount {
      return Err(ErrorCode::InvalidInput.into());
    }
    if !minter.is_active {
      return Err(ErrorCode::Unavailable.into());
    }

    let current_timestamp = Clock::get().unwrap().unix_timestamp;
    let timestamp_per_period = i64::from(app_data.limit) * 3600;
    let is_in_period = minter.last_period_timestamp + timestamp_per_period > current_timestamp;
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

    for (i, input_token) in minter.input_tokens.iter().enumerate() {
      let input_price_feed = &minter.input_price_feeds[i];
      let (mut price, mut precision) = (1u64, 1u64);
      let use_price_feed = *input_price_feed != system_program::ID;
      if use_price_feed {
        let price_feed = &accounts[3*i];
        if price_feed.key() != *input_price_feed {
          return Err(ErrorCode::InvalidAccount.into());
        }
        (price, precision) = get_price_feed(
            &*chainlink_program,
            &*price_feed,
          );
      }
      else {
        let clock = Clock::get().unwrap();
        msg!("Price fetched: {}/{} at {} in block {}", price, precision, clock.unix_timestamp, clock.slot);
      }

      let value_contrib = minter.input_percentages[i];

      let input_value = amount.checked_mul(u64::from(value_contrib)).unwrap().checked_div(10000).unwrap();
      let input_amount = multiply_fraction(input_value, precision, price);
      let input_precision = u64::pow(10, u32::from(minter.input_decimals[i]));
      let input_amount = multiply_fraction(input_amount, input_precision, CUSD_PRECISION);

      let from_account_index = account_indices[3*i + 1];
      let to_account_index = account_indices[3*i + 2];
      let from_account = &accounts[from_account_index];
      let from_account = TokenAccount::unpack_from_slice(&from_account.try_borrow_data().unwrap()).unwrap();
      let to_account = &accounts[to_account_index];
      let to_account = TokenAccount::unpack_from_slice(&to_account.try_borrow_data().unwrap()).unwrap();
      if from_account.mint != *input_token {
        return Err(ErrorCode::InvalidAccount.into());
      }
      if to_account.mint != *input_token || to_account.owner != root_signer.key() {
        return Err(ErrorCode::InvalidAccount.into());
      }

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
    minter.per_period_minted_amount = current_period_minted_amount + amount;
    if !is_in_period {
      minter.last_period_timestamp = current_timestamp;
    }

    let protocol_fee = multiply_fraction(amount, u64::from(minter.fee_percent), 10000);
    let amount_to_transfer = amount.checked_sub(protocol_fee).unwrap();
    minter.accumulated_fee = minter.accumulated_fee.checked_add(protocol_fee).unwrap();

    let cusd_mint = &ctx.accounts.cusd_mint;
    let recipient = &ctx.accounts.recipient;

    let seeds: &[&[u8]] = &[
      ROOT_SIGNER_SEED_1,
      ROOT_SIGNER_SEED_2,
      &[app_data.signer_nonce],
    ];

    mint_token(
        &*root_signer,
        &*cusd_mint,
        &*recipient,
        amount_to_transfer,
        &[&seeds],
      )
      .expect("CUSD Factory: CPI failed.");

    Ok(())
  }

  pub fn burn<'a>(
    ctx: Context<'_, '_, '_, 'a, BurnContext<'a>>,
    amount: u64, // amount of output_token user want to burn
  ) -> Result<()> {

    let user = &ctx.accounts.user;
    let app_data = &ctx.accounts.app_data;
    let burner = &ctx.accounts.burner;
    let chainlink_program = &ctx.accounts.chainlink_program;
    let accounts = &ctx.remaining_accounts;
    let price_feed = &accounts[0];

    if amount < burner.min_amount {
      return Err(ErrorCode::InvalidInput.into());
    }
    if !burner.is_active {
      return Err(ErrorCode::Unavailable.into());
    }

    let current_timestamp = Clock::get().unwrap().unix_timestamp;
    let timestamp_per_period = i64::from(app_data.limit) * 3600;
    let is_in_period = burner.last_period_timestamp + timestamp_per_period > current_timestamp;
    let current_period_burned_amount = if is_in_period { burner.per_period_burned_amount } else { 0u64 };

    let (mut price, mut precision) = (1u64, 1u64);
    let use_price_feed = burner.output_price_feed != system_program::ID;
    if use_price_feed {
      if price_feed.key() != burner.output_price_feed {
        return Err(ErrorCode::InvalidAccount.into());
      }
      (price, precision) = get_price_feed(
        &*chainlink_program,
        &*price_feed,
      );
    }
    else {
      let clock = Clock::get().unwrap();
      msg!("Price fetched: {}/{} at {} in block {}", price, precision, clock.unix_timestamp, clock.slot);
    }

    let cusd_amount = multiply_fraction(amount, price, precision);
    let output_precision = u64::pow(10, u32::from(burner.output_decimals));
    let cusd_amount = multiply_fraction(cusd_amount, CUSD_PRECISION, output_precision);

    if current_period_burned_amount + cusd_amount > burner.per_period_burned_limit {
      return Err(ErrorCode::LimitReached.into());
    }
    if burner.total_burned_amount + cusd_amount > burner.total_burned_limit {
      return Err(ErrorCode::LimitReached.into());
    }

    let pool_cusd = &ctx.accounts.pool_cusd;
    let user_cusd = &ctx.accounts.user_cusd;
    transfer_token(
        &*user,
        &user_cusd.to_account_info(),
        &pool_cusd.to_account_info(),
        cusd_amount,
        &[],
      )
      .expect("CUSD Factory: CPI failed.");

    let root_signer = &ctx.accounts.root_signer;
    let cusd_mint = &ctx.accounts.cusd_mint;
    let seeds: &[&[u8]] = &[
      ROOT_SIGNER_SEED_1,
      ROOT_SIGNER_SEED_2,
      &[app_data.signer_nonce],
    ];
    burn_token(
        &*root_signer,
        &*cusd_mint,
        &pool_cusd.to_account_info(),
        cusd_amount,
        &[&seeds],
      )
      .expect("CUSD Factory: CPI failed.");

    let burner = &mut ctx.accounts.burner;
    burner.total_burned_amount = burner.total_burned_amount + cusd_amount;
    burner.per_period_burned_amount = current_period_burned_amount + cusd_amount;
    if !is_in_period {
      burner.last_period_timestamp = current_timestamp;
    }
    let protocol_fee = multiply_fraction(amount, u64::from(burner.fee_percent), 10000);
    let amount_to_transfer = amount.checked_sub(protocol_fee).unwrap();
    burner.accumulated_fee = burner.accumulated_fee.checked_add(protocol_fee).unwrap();

    let pool_token = &accounts[1];
    let pool_token = TokenAccount::unpack_from_slice(&pool_token.try_borrow_data().unwrap()).unwrap();
    if pool_token.owner != root_signer.key() || pool_token.mint != burner.output_token {
      return Err(ErrorCode::InvalidAccount.into());
    }
    let user_token = &accounts[2];
    let user_token = TokenAccount::unpack_from_slice(&user_token.try_borrow_data().unwrap()).unwrap();
    if user_token.mint != burner.output_token {
      return Err(ErrorCode::InvalidAccount.into());
    }
    transfer_token(
        &*root_signer,
        &accounts[1],
        &accounts[2],
        amount_to_transfer,
        &[&seeds],
      )
      .expect("CUSD Factory: CPI failed.");

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn withdraw_token(
    ctx: Context<WithdrawTokenContext>,
    amount: u64,
  ) -> Result<()> {

    let app_data = &ctx.accounts.app_data;
    let root_signer = &ctx.accounts.root_signer;
    let pool_token = &ctx.accounts.pool_token;
    let recipient_token = &ctx.accounts.recipient_token;
    let seeds: &[&[u8]] = &[
      ROOT_SIGNER_SEED_1,
      ROOT_SIGNER_SEED_2,
      &[app_data.signer_nonce],
    ];

    transfer_token(
        &*root_signer,
        &pool_token.to_account_info(),
        &recipient_token.to_account_info(),
        amount,
        &[&seeds],
      )
      .expect("CUSD Factory: CPI failed.");

    emit!(WithdrawTokenEvent {
      recipient_token_account: *ctx.accounts.recipient_token.to_account_info().key,
      amount,
    });

    Ok(())
  }

  #[access_control(is_root(*ctx.accounts.root.key))]
  pub fn unlock_token_mint(
    ctx: Context<UnlockTokenMintContext>,
  ) -> Result<()> {

    let root = &ctx.accounts.root;
    let app_data = &ctx.accounts.app_data;
    let root_signer = &ctx.accounts.root_signer;
    let token_mint = &ctx.accounts.token_mint;
    let seeds: &[&[u8]] = &[
      ROOT_SIGNER_SEED_1,
      ROOT_SIGNER_SEED_2,
      &[app_data.signer_nonce],
    ];
    transfer_authority(
        &*root_signer,
        &token_mint.to_account_info(),
        0,
        &*root,
        &[&seeds],
      )
      .expect("CUSD Factory: CPI failed.");

    emit!(UnlockTokenMintEvent {
      token_mint: *token_mint.to_account_info().key,
      new_authority: *root_signer.key,
    });

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
        ROOT_SIGNER_SEED_1,
        ROOT_SIGNER_SEED_2,
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
    limit: u32,
  ) -> Result<()> {

    let app_data = &mut ctx.accounts.app_data;
    app_data.limit = limit;

    emit!(SetAppDataEvent {
      limit,
    });

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

fn multiply_fraction(number: u64, numerator: u64, denominator: u64) -> u64 {
  let number_128 = u128::from(number)
    .checked_mul(u128::from(numerator)).unwrap()
    .checked_div(u128::from(denominator)).unwrap();
  u64::try_from(number_128).unwrap()
}
