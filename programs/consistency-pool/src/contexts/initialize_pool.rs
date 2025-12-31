use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ Mint, TokenAccount, TokenInterface },
};
use crate::state::PoolAccount;

#[derive(Accounts)]
#[instruction(consistency_period: u64, min_deposit: u64)]
pub struct InitializePool<'info> {
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub usdc_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = owner,
        associated_token::mint = usdc_mint,
        associated_token::authority = pool_account,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = owner,
        space = 8 + PoolAccount::INIT_SPACE,
        seeds = [b"pool", owner.key().as_ref()],
        bump
    )]
    pub pool_account: Account<'info, PoolAccount>,
}

impl<'info> InitializePool<'info> {
    pub fn initialize(
        &mut self,
        consistency_period: u64,
        min_deposit: u64,
        bumps: &InitializePoolBumps
    ) -> Result<()> {
        let pool = &mut self.pool_account;

        pool.owner = self.owner.key();
        pool.usdc_mint = self.usdc_mint.key();
        pool.vault = self.vault.key();
        pool.consistency_period = consistency_period;
        pool.min_deposit = min_deposit;
        pool.total_deposited = 0;
        pool.successful_count = 0;
        pool.failed_count = 0;
        pool.start_time = Clock::get()?.unix_timestamp as u64;
        pool.bump = bumps.pool_account;

        msg!("Pool initialized");
        Ok(())
    }
}
