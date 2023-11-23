use anchor_lang::prelude::*;

use crate::{helpers::transfer_lamports, state::sol_escrow::SolEscrowWallet};

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        space = SolEscrowWallet::LEN,
        seeds = [owner.key().as_ref(), "Hitori".as_bytes()],
        bump
    )]
    pub sol_escrow_wallet: Account<'info, SolEscrowWallet>,

    pub system_program: Program<'info, System>,
}

pub fn deposit_sol_handler(ctx: Context<DepositSol>, lamports: u64) -> Result<()> {
    let sol_escrow_wallet = &mut ctx.accounts.sol_escrow_wallet;

    **sol_escrow_wallet = SolEscrowWallet::new(ctx.accounts.owner.key());

    transfer_lamports(
        &ctx.accounts.owner,
        &ctx.accounts.sol_escrow_wallet.to_account_info(),
        &ctx.accounts.system_program,
        lamports,
    )?;

    Ok(())
}
