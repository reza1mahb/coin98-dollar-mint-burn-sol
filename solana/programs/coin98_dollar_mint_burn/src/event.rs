use anchor_lang::prelude::{
  AnchorDeserialize,
  AnchorSerialize,
  borsh,
  event,
  Pubkey,
};

#[event]
pub struct CreateMinterEvent {
  pub is_active: bool,
}

#[event]
pub struct SetMinterEvent {
  pub is_active: bool,
  pub input_tokens: Vec<Pubkey>,
  pub input_decimals: Vec<u16>,
  pub input_percentages: Vec<u16>,
  pub input_price_feeds: Vec<Pubkey>,
  pub fee_percent: u16,
  pub total_minted_limit: u64,
  pub per_period_minted_limit: u64,
  pub min_amount: u64,
}

#[event]
pub struct CreateBurnerEvent {
  pub is_active: bool,
}

#[event]
pub struct SetBurnerEvent {
  pub is_active: bool,
  pub output_token: Pubkey,
  pub output_decimals: u16,
  pub output_price_feed: Pubkey,
  pub fee_percent: u16,
  pub total_burned_limit: u64,
  pub per_period_burned_limit: u64,
  pub min_amount: u64,
}

#[event]
pub struct SetAppDataEvent {
  pub limit: u32,
}

#[event]
pub struct WithdrawTokenEvent {
  pub recipient_token_account: Pubkey,
  pub amount: u64,
}

#[event]
pub struct UnlockTokenMintEvent {
  pub token_mint: Pubkey,
  pub new_authority: Pubkey,
}
