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

#[derive(Clone)]
pub struct TokenMint(spl_token::TokenMint);

impl TokenMint {
  pub const LEN: usize = spl_token::TokenMint::LEN;
}

impl anchor_lang::AccountDeserialize for TokenMint {
  fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
    spl_token::TokenMint::unpack(buf)
      .map(TokenMint)
      .map_err(Into::into)
  }
}

impl anchor_lang::AccountSerialize for TokenMint {}

impl anchor_lang::Owner for TokenMint {
  fn owner() -> Pubkey {
    spl_token::ID
  }
}

impl Deref for TokenMint {
  type Target = spl_token::TokenMint;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct BurnTokenParams {
  pub instruction: u8,
  pub amount: u64,
}

pub fn burn_token<'a>(
  owner: &AccountInfo<'a>,
  token_mint: &AccountInfo<'a>,
  token_account: &AccountInfo<'a>,
  amount: u64,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let data = BurnTokenParams {
    instruction: 8,
    amount,
  };
  let instruction = Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      AccountMeta::new(*token_account.key, false),
      AccountMeta::new(*token_mint.key, false),
      AccountMeta::new_readonly(*owner.key, true),
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    invoke(&instruction, &[token_account.clone(), token_mint.clone(), owner.clone()])
  }
  else {
    invoke_signed(&instruction, &[token_account.clone(), token_mint.clone(), owner.clone()], &signer_seeds)
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct MintTokenParams {
  pub instruction: u8,
  pub amount: u64,
}

pub fn mint_token<'a>(
  authority: &AccountInfo<'a>,
  token_mint: &AccountInfo<'a>,
  token_account: &AccountInfo<'a>,
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
pub struct TransferAuthorityParams {
  pub instruction: u8,
  pub authority_type: u8,
  pub has_authority: bool,
  pub new_authority: Pubkey,
}

pub fn transfer_authority<'a>(
  authority: &AccountInfo<'a>,
  token_mint: &AccountInfo<'a>,
  authority_type: u8,
  new_authority: &AccountInfo<'a>,
  signer_seeds: &[&[&[u8]]],
) -> std::result::Result<(), ProgramError> {
  let data = TransferAuthorityParams {
    instruction: 6,
    authority_type,
    has_authority: true,
    new_authority: new_authority.key(),
  };
  let instruction = Instruction {
    program_id: TOKEN_PROGRAM_ID,
    accounts: vec![
      AccountMeta::new(*token_mint.key, false),
      AccountMeta::new_readonly(*authority.key, true),
    ],
    data: data.try_to_vec().unwrap(),
  };
  if signer_seeds.len() == 0 {
    invoke(&instruction, &[token_mint.clone(), authority.clone()])
  }
  else {
    invoke_signed(&instruction, &[token_mint.clone(), authority.clone()], &signer_seeds)
  }
}

#[derive(AnchorSerialize, AnchorDeserialize, Default)]
pub struct TransferTokenParams {
  pub instruction: u8,
  pub amount: u64,
}

pub fn transfer_token<'a>(
  owner: &AccountInfo<'a>,
  from_pubkey: &AccountInfo<'a>,
  to_pubkey: &AccountInfo<'a>,
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
