use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use solana_program::program::invoke;
use spl_stake_pool::instruction::deposit_sol;

#[cfg(feature = "mainnet")]
declare_id!("SPPdCjFYYwH3ca2kCT9baLcgbXz81P5bd5QutHynuRz");
#[cfg(not(feature = "mainnet"))]
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stake_pool_proxy {
    use super::*;

    pub fn deposit_all_sol(ctx: Context<DepositAllSol>) -> Result<()> {
        let deposit_amount = ctx.accounts.lamports_from.lamports();

        let instruction = deposit_sol(
            ctx.accounts.stake_pool_program.key,
            &ctx.accounts.stake_pool.key,
            ctx.accounts.stake_pool_withdraw_authority.key,
            ctx.accounts.reserve_stake_account.key,
            ctx.accounts.lamports_from.key,
            ctx.accounts.pool_tokens_to.key,
            ctx.accounts.manager_fee_account.key,
            ctx.accounts.referrer_pool_tokens_account.key,
            &ctx.accounts.pool_mint.key(),
            ctx.accounts.token_program.key,
            deposit_amount,
        );

        let cpi_accounts = &[
            ctx.accounts.stake_pool.clone(),
            ctx.accounts.stake_pool_withdraw_authority.clone(),
            ctx.accounts.reserve_stake_account.clone(),
            ctx.accounts.lamports_from.clone(),
            ctx.accounts.pool_tokens_to.clone(),
            ctx.accounts.manager_fee_account.clone(),
            ctx.accounts.referrer_pool_tokens_account.clone(),
            ctx.accounts.pool_mint.clone().to_account_info(),
            ctx.accounts.stake_pool_program.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.rent.clone().to_account_info(),
        ];

        invoke(&instruction, cpi_accounts)?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct DepositAllSol<'info> {
    /// CHECK: Checked in stake pool program
    #[account(mut)]
    pub stake_pool: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    pub stake_pool_withdraw_authority: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    #[account(mut)]
    pub reserve_stake_account: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    #[account(mut, signer)]
    pub lamports_from: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    #[account(mut)]
    pub pool_tokens_to: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    #[account(mut)]
    pub manager_fee_account: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    #[account(mut)]
    pub referrer_pool_tokens_account: AccountInfo<'info>,

    #[account(mut)]
    pub pool_mint: Account<'info, Mint>,

    /// CHECK: Checked in constraints
    #[account(address = spl_stake_pool::ID)]
    pub stake_pool_program: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    pub token_program: AccountInfo<'info>,

    /// CHECK: Checked in stake pool program
    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}
