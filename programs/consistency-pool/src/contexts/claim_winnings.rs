use anchor_lang::prelude::*;
use anchor_spl::{
    token::TransferChecked,
    token_interface::{ Mint, TokenAccount, TokenInterface },
    associated_token::AssociatedToken,
};
use crate::state::PoolAccount;

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(mut, associated_token::mint = usdc_mint, associated_token::authority = user, associated_token::token_program = token_program)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub pool_account: Account<'info, PoolAccount>,
}

impl<'info> ClaimWinnings<'info> {
    pub fn claim_winnings(&mut self) -> Result<()> {
        require!(
            self.pool_account.successful_count > 0,
            crate::ConsistencyError::NoSuccessfulUsers
        );
        anchor_spl::token::transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.vault.to_account_info(),
                    to: self.user_token_account.to_account_info(),
                    authority: self.pool_account.to_account_info(),
                    mint: self.usdc_mint.to_account_info(),
                },
                &[&[b"pool", self.pool_account.owner.as_ref(), &[self.pool_account.bump]]]
            ),
            self.pool_account.total_deposited
                .checked_div(self.pool_account.successful_count as u64)
                .ok_or(crate::ConsistencyError::InvalidCalculation)?,
            self.usdc_mint.decimals
        )?;
        Ok(())
    }
}
