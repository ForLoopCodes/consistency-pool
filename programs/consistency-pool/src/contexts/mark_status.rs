use anchor_lang::prelude::*;

use crate::state::PoolAccount;

#[derive(Accounts)]
pub struct MarkStatus<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub pool_account: Account<'info, PoolAccount>,
}

impl<'info> MarkStatus<'info> {
    pub fn mark_status(&mut self, succeeded: bool) -> Result<()> {
        if succeeded {
            self.pool_account.successful_count = self.pool_account.successful_count
                .checked_add(1)
                .ok_or(crate::ConsistencyError::Overflow)?;
            msg!("User {} marked as successful", self.user.key());
        } else {
            self.pool_account.failed_count = self.pool_account.failed_count
                .checked_add(1)
                .ok_or(crate::ConsistencyError::Overflow)?;
            msg!("User {} marked as failed", self.user.key());
        }
        Ok(())
    }
}
