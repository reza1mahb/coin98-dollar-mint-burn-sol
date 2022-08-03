use solana_program::{
  account_info::{
    AccountInfo,
  },
  declare_id,
};

#[cfg(feature = "localhost")]
declare_id!("CXDihKKJfusBgYx9EqfLLYRZJGDRBUVjFmQTGutn9WAZ");

#[cfg(feature = "devnet")]
declare_id!("CXDihKKJfusBgYx9EqfLLYRZJGDRBUVjFmQTGutn9WAZ");

#[cfg(feature = "devnet")]
declare_id!("CXDihKKJfusBgYx9EqfLLYRZJGDRBUVjFmQTGutn9WAZ");

#[cfg(feature = "unit-test")]
declare_id!("CXDihKKJfusBgYx9EqfLLYRZJGDRBUVjFmQTGutn9WAZ");

#[cfg(all(not(feature = "localhost"), not(feature = "devnet"), not(feature = "unit-test")))]
declare_id!("CUSDvqAQLbt7fRofcmV2EXfPA2t36kzj7FjzdmqDiNQL");

pub fn is_cusd_token_mint<'a>(account: &AccountInfo<'a>) -> bool {
  *account.key == ID
}
