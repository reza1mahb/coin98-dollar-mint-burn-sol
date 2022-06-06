use anchor_lang::prelude::*;

#[account]
pub struct AppData {
  pub nonce: u8,
  pub limit: u64,
}

impl AppData {
  pub const LEN: usize = 1 + 8;
}
