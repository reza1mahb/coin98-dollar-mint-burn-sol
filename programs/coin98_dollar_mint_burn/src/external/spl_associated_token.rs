use solana_program::pubkey::{
  Pubkey,
};
use crate::external::spl_token::{
  ID as TOKEN_PROGRAM_ID,
};

solana_program::declare_id!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

pub fn get_associated_token_address(
  wallet: &Pubkey,
  mint: &Pubkey,
) -> Pubkey {
  Pubkey::find_program_address(
    &[
      &wallet.to_bytes(),
      &TOKEN_PROGRAM_ID.to_bytes(),
      &mint.to_bytes(),
    ],
    &ID,
  ).0
}
