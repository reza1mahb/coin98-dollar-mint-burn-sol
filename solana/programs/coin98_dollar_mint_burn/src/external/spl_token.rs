use solana_program::account_info::{
  AccountInfo,
};

solana_program::declare_id!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub fn is_token_program<'a>(account: &AccountInfo<'a>) -> bool {
  *account.key == ID
}
