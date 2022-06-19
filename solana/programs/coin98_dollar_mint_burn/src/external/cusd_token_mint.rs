use solana_program::{
  account_info::{
    AccountInfo,
  },
  declare_id,
};

#[cfg(feature = "localhost")]
declare_id!("CXDihKKJfusBgYx9EqfLLYRZJGDRBUVjFmQTGutn9WAZ");

#[cfg(not(feature = "localhost"))]
declare_id!("CUSDsY78qAQbDEivJuxzpcpkXYMyW2sg2Mpk4iwFckR4");

pub fn is_cusd_token_mint<'a>(account: &AccountInfo<'a>) -> bool {
  *account.key == ID
}
