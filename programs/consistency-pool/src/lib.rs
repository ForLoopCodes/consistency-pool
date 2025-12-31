use anchor_lang::prelude::*;

pub mod contexts;
pub mod state;

use contexts::*;
use state::*;

declare_id!("6BLPdL9narQPFQsqS7AXuRBRS4VoyKmHHzdwkgnLaApt");

#[program]
pub mod consistency_pool {
    use super::*;

    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        consistency_period: u64,
        min_deposit: u64
    ) -> Result<()> {
        ctx.accounts.initialize(consistency_period, min_deposit, &ctx.bumps)?;
        Ok(())
    }
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }
    pub fn mark_status(ctx: Context<MarkStatus>, succeeded: bool) -> Result<()> {
        ctx.accounts.mark_status(succeeded)?;
        Ok(())
    }
    pub fn claim_winnings(ctx: Context<ClaimWinnings>) -> Result<()> {
        ctx.accounts.claim_winnings()?;
        Ok(())
    }
}
