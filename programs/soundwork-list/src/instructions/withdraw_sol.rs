use anchor_lang::prelude::*;

use crate::{error::CustomError, state::sol_escrow::SolEscrowWallet};

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    /// CHECK: the payer might not be a signer
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: checked in the ix
    #[account(
        mut,
        address = sol_escrow_wallet.owner @ CustomError::InvalidAuthority
    )]
    pub authority: AccountInfo<'info>,

    #[account(mut)]
    pub sol_escrow_wallet: Account<'info, SolEscrowWallet>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_sol_handler(ctx: Context<WithdrawSol>, lamports: Option<u64>) -> Result<()> {
    let sol_escrow_wallet = &ctx.accounts.sol_escrow_wallet;

    if let Some(lamports) = lamports {
        if lamports > sol_escrow_wallet.get_lamports() {
            return Err(error!(CustomError::InsufficientFunds));
        }

        // transfer requested amount
        ctx.accounts.sol_escrow_wallet.sub_lamports(lamports)?;
        ctx.accounts.authority.add_lamports(lamports)?;

        return Ok(());
    }

    // else close the account and transfer everything to user
    sol_escrow_wallet.close(ctx.accounts.authority.to_account_info())?;

    Ok(())
}
