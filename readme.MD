# Consistency Pool

This example demonstrates how to create a smart contract for tracking user consistency in challenges or habits with financial incentives. Users deposit funds into a pool, mark their daily/periodic status as successful or failed, and successful participants can claim proportional winnings at the end of the consistency period.

---

## Let's walk through the architecture:

For this program, we will have one state account, the pool account:

```rust
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
```

The pool account will hold the following data:

- `owner`: The account that created the pool.
- `usdc_mint`: The mint of the USDC token used for deposits.
- `vault`: The associated token account that holds all deposited funds.
- `consistency_period`: The duration of the consistency challenge in seconds.
- `min_deposit`: The minimum amount each user must deposit to participate.
- `total_deposited`: The total amount of USDC deposited by all users.
- `successful_count`: The number of users who marked themselves as successful.
- `failed_count`: The number of users who marked themselves as failed.
- `start_time`: The timestamp when the pool was initialized.
- `bump`: The bump seed for the pool account PDA.

---

## Initialize Pool

The pool owner initializes the consistency pool with the following context:

```rust
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
```

The accounts passed in this context:

- `owner`: The account creating and owning the pool.
- `usdc_mint`: The USDC token mint.
- `vault`: The vault ATA that will hold all deposited funds.
- `pool_account`: The pool state account.

The `initialize` function sets up the pool with the specified consistency period and minimum deposit amount.

---

## Deposit

Users can deposit USDC into the pool to participate:

```rust
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
```

The accounts passed in this context:

- `user`: The account depositing funds.
- `usdc_mint`: The USDC token mint.
- `user_token_account`: The user's USDC token account.
- `vault`: The pool's vault ATA.
- `pool_account`: The pool state account.

The `deposit` function transfers USDC from the user to the vault and updates the total deposited amount. The deposit amount must be at least the minimum deposit specified during initialization.

---

## Mark Status

Users mark their status as successful or failed:

```rust
#[derive(Accounts)]
pub struct MarkStatus<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub pool_account: Account<'info, PoolAccount>,
}
```

The accounts passed in this context:

- `user`: The account marking their status.
- `pool_account`: The pool state account.

The `mark_status` function increments either the successful_count or failed_count based on the boolean parameter.

---

## Claim Winnings

After the consistency period ends, successful users can claim their proportional share of the winnings:

```rust
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
```

The accounts passed in this context:

- `user`: The account claiming winnings.
- `vault`: The pool's vault ATA.
- `usdc_mint`: The USDC token mint.
- `user_token_account`: The user's USDC token account.
- `pool_account`: The pool state account.

The `claim_winnings` function transfers the user's proportional share (total_deposited / successful_count) from the vault to their token account. This can only be called if there are successful users and uses PDA signing for the vault authority.
