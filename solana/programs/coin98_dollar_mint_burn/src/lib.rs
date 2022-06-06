pub mod constant;
pub mod context;
pub mod state;

use anchor_lang::prelude::*;
use crate::context::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod coin98_dollar_mint_burn {
  use super::*;

  pub fn create_app_data(
    ctx: Context<CreateAppDataContext>,
  ) -> Result<()> {

    let app_data = &mut ctx.accounts.app_data;
    app_data.nonce = *ctx.bumps.get("app_data").unwrap();
    app_data.limit = 24;

    Ok(())
  }

  pub fn set_app_data(
    ctx: Context<SetAppDataContext>,
    limit: u64,
  ) -> Result<()> {

    let app_data = &mut ctx.accounts.app_data;
    app_data.limit = limit;

    Ok(())
  }
}
