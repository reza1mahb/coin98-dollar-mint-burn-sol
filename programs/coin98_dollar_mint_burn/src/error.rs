use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {

  #[msg("CUSD Factory: Invalid account.")]
  InvalidAccount,

  #[msg("CUSD Factory: Invalid input.")]
  InvalidInput,

  #[msg("CUSD Factory: Limit reached")]
  LimitReached,

  #[msg("CUSD Factory: Unauthorized")]
  Unauthorized,

  #[msg(CUSD Factory: Factory unavailable)]
  Unavailable,
}
