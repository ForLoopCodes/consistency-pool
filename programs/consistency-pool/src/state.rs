use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PoolAccount {
    pub owner: Pubkey,
    pub usdc_mint: Pubkey,
    pub vault: Pubkey,
    pub consistency_period: u64,
    pub min_deposit: u64,
    pub total_deposited: u64,
    pub successful_count: u64,
    pub failed_count: u64,
    pub start_time: u64,
    pub bump: u8,
}

#[error_code]
pub enum ConsistencyError {
    #[msg("Deposit amount is below minimum")]
    DepositTooSmall,

    #[msg("Calculation resulted in overflow")]
    Overflow,

    #[msg("No successful users in pool")]
    NoSuccessfulUsers,

    #[msg("Invalid calculation")]
    InvalidCalculation,

    #[msg("Consistency period not yet ended")]
    PeriodNotEnded,

    #[msg("User already claimed winnings")]
    AlreadyClaimed,
}
