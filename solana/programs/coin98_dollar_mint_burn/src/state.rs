use anchor_lang::prelude::*;

#[account]
pub struct AppData {
  pub nonce: u8,
  pub limit: u64,
}

impl AppData {
  pub const LEN: usize = 1 + 8;
}

#[account]
pub struct Minter {
  pub nonce: u8,
  pub is_active: bool,
  pub input_tokens: Vec<Pubkey>,
  pub input_decimals: Vec<u8>,
  pub input_percentages: Vec<u16>,
  pub input_price_feeds: Vec<Pubkey>,
  pub fee_percent: u16,
  pub accumulated_fee: u64,
  pub total_minted_amount: u64,
  pub total_minted_limit: u64,
  pub per_period_minted_amount: u64,
  pub per_period_minted_limit: u64,
  pub last_period_timestamp: i64,
}

impl Minter {
  pub fn size(token_count: u8) -> usize {
    let token_count = usize::from(token_count);
    1 + 1 + (4 + 32 * token_count) + (4 + token_count) + (4 + 2 * token_count) + (4 + 32 * token_count) + 2 + 8 + 8 + 8 + 8 + 8 + 8
  }
}

#[account]
pub struct Burner {
  pub nonce: u8,
  pub is_active: bool,
  pub output_token: Pubkey,
  pub output_decimals: u8,
  pub output_price_feed: Pubkey,
  pub fee_percent: u16,
  pub accumulated_fee: u64,
  pub total_burned_amount: u64,
  pub total_burned_limit: u64,
  pub per_period_burned_amount: u64,
  pub per_period_burned_limit: u64,
  pub last_period_timestamp: i64,
}

impl Burner {
  pub const LEN: usize = 1 + 1 + 32 + 1 + 32 + 2 + 8 + 8 + 8 + 8 + 8 + 8;
}
