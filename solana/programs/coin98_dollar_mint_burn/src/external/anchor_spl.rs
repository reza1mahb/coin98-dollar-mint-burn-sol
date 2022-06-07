use anchor_lang::prelude::*;
use solana_program::{
  instruction::{
    Instruction,
  },
  program::{
    invoke,
    invoke_signed,
  },
  program_pack::{
    Pack,
  },
};
use std::ops::{
  Deref,
};
use crate::external::spl_token;
use crate::external::spl_token::{
  ID as TOKEN_PROGRAM_ID,
};

#[derive(Clone)]
pub struct TokenAccount(spl_token::TokenAccount);

impl TokenAccount {
  pub const LEN: usize = spl_token::TokenAccount::LEN;
}

impl anchor_lang::AccountDeserialize for TokenAccount {
  fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
    spl_token::TokenAccount::unpack(buf)
      .map(TokenAccount)
      .map_err(Into::into)
  }
}

impl anchor_lang::AccountSerialize for TokenAccount {}

impl anchor_lang::Owner for TokenAccount {
  fn owner() -> Pubkey {
    spl_token::ID
  }
}

impl Deref for TokenAccount {
  type Target = spl_token::TokenAccount;

  fn deref(&self) -> &Self::Target {
      &self.0
  }
}

pub fn mint_token<'i>(
  authority: &AccountInfo<'i>,
  token_mint: &AccountInfo<'i>,
  token_account: &AccountInfo<'i>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let data = MintTokenParams {
    instruction: 7,
    amount,
  };
  let instruction = Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      AccountMeta::new(*token_mint.key, false),
      AccountMeta::new(*token_account.key, false),
      AccountMeta::new_readonly(*authority.key, true),
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    invoke(&instruction, &[token_mint.clone(), token_account.clone(), authority.clone()])
  }
  else {
    invoke_signed(&instruction, &[token_mint.clone(), token_account.clone(), authority.clone()], &signer_seeds)
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct MintTokenParams {
  pub instruction: u8,
  pub amount: u64,
}

pub fn transfer_token<'i>(
  owner: &AccountInfo<'i>,
  from_pubkey: &AccountInfo<'i>,
  to_pubkey: &AccountInfo<'i>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let data = TransferTokenParams {
    instruction: 3,
    amount,
  };
  let instruction = Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      AccountMeta::new(*from_pubkey.key, false),
      AccountMeta::new(*to_pubkey.key, false),
      AccountMeta::new_readonly(*owner.key, true),
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    invoke(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()])
  }
  else {
    invoke_signed(&instruction, &[from_pubkey.clone(), to_pubkey.clone(), owner.clone()], &signer_seeds)
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct TransferTokenParams {
  pub instruction: u8,
  pub amount: u64,
}
