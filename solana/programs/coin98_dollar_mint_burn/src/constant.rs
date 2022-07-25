#[cfg(feature = "localhost")]
pub const ROOT_KEYS: &[&str] = &[
  "8ST8fTBGKaVPx4f1KG1zMMw4EJmSJBW2UgX1JR2pPoVa",
];

#[cfg(feature = "devnet")]
pub const ROOT_KEYS: &[&str] = &[
  "EZuvvbVWibGSQpU4urZixQho2hDWtarC9bhT5NVKFpw8",
  "5UrM9csUEDBeBqMZTuuZyHRNhbRW4vQ1MgKJDrKU1U2v",
  "GnzQDYm2gvwZ8wRVmuwVAeHx5T44ovC735vDgSNhumzQ",
];

#[cfg(all(not(feature = "localhost"), not(feature = "devnet")))]
pub const ROOT_KEYS: &[&str] = &[
  ""
];

pub const APP_DATA_SEED_1: &[u8] = &[144, 146, 13, 147, 226, 199, 230, 50];
pub const APP_DATA_SEED_2: &[u8] = &[15, 81, 173, 106, 105, 203, 253, 99];
pub const CUSD_PRECISION: u64 = 1000000; // decimals = 6
pub const ROOT_SIGNER_SEED_1: &[u8] = &[2, 151, 229, 53, 244, 77, 229, 7];
pub const ROOT_SIGNER_SEED_2: &[u8] = &[68, 203, 0, 94, 226, 230, 93, 156];

pub const SYSTEM_FEE_CAP: u16 = 2000;
