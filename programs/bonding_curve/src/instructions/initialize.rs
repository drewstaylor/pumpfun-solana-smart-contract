use crate::{errors::CustomError, state::*};
use anchor_lang::prelude::*;

pub fn initialize(
    ctx: Context<InitializeCurveConfiguration>,
    fees: f64,
) -> Result<()> {
    let dex_config = &mut ctx.accounts.dex_configuration_account;

    if !(0_f64..=100_f64).contains(&fees) {
        return err!(CustomError::InvalidFee);
    }

    dex_config.set_inner(CurveConfiguration::new(fees));

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCurveConfiguration<'info> {
    #[account(
        init,
        space = CurveConfiguration::ACCOUNT_SIZE,
        payer = admin,
        seeds = [CurveConfiguration::SEED.as_bytes()],
        bump,
    )]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
