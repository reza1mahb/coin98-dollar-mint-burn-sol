use anchor_lang::prelude::*;
use solana_program::{
  instruction::{
    Instruction,
  },
  program::{
    invoke,
    invoke_signed,
  },
};
use crate::external::spl_token::{
  ID as TOKEN_PROGRAM_ID,
};

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
