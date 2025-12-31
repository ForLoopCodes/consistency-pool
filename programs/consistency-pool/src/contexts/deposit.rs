use anchor_lang::prelude::*;
use anchor_spl::{
    token::TransferChecked,
    token_interface::{ Mint, TokenAccount, TokenInterface },
    associated_token::AssociatedToken,
};
use crate::state::PoolAccount;

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(
      mut, 
      associated_token::mint = usdc_mint, 
      associated_token::authority = user, 
      associated_token::token_program = token_program
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub pool_account: Account<'info, PoolAccount>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(amount >= self.pool_account.min_deposit, crate::ConsistencyError::DepositTooSmall);
        anchor_spl::token::transfer_checked(
            CpiContext::new(self.token_program.to_account_info(), TransferChecked {
                from: self.user_token_account.to_account_info(),
                to: self.vault.to_account_info(),
                authority: self.user.to_account_info(),
                mint: self.usdc_mint.to_account_info(),
            }),
            amount,
            self.usdc_mint.decimals
        )?;

        self.pool_account.total_deposited = self.pool_account.total_deposited
            .checked_add(amount)
            .ok_or(crate::ConsistencyError::Overflow)?;
        Ok(())
    }
}
