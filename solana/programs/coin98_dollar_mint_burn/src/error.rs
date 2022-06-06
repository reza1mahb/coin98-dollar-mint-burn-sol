use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {

  #[msg("CUSD Factory: Invalid input.")]
  InvalidInput,

  #[msg("CUSD Factory: Unauthorized")]
  Unauthorized,
}
