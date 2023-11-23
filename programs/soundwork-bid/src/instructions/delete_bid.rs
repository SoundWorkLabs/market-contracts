use anchor_lang::prelude::*;

use crate::state::bid::BiddingDataV1;
use soundwork_list::{
    self, cpi::accounts::WithdrawSol, error::CustomError, program::SoundworkList,
};

#[derive(Accounts)]
pub struct DeleteBid<'info> {
    #[account(
        mut,
        address = bidding_data_acc.owner @ CustomError::UnrecognizedSigner
    )]
    pub bidder: Signer<'info>,

    #[account(
        mut,
        close = bidder
    )]
    pub bidding_data_acc: Account<'info, BiddingDataV1>,

    /// CHECK: initialized by list program through the CPI
    #[account(
        mut,
        owner = soundwork_list.key()
    )]
    pub sol_escrow_wallet: AccountInfo<'info>,

    pub soundwork_list: Program<'info, SoundworkList>,
    pub system_program: Program<'info, System>,
}

pub fn delete_bid_handler(ctx: Context<DeleteBid>) -> Result<()> {
    let bidding_data_acc = &ctx.accounts.bidding_data_acc;

    soundwork_list::cpi::withdraw_sol(
        ctx.accounts.withdraw_escrow_sol_ctx(),
        bidding_data_acc.lamports.into(),
    )?;

    Ok(())
}

impl<'info> DeleteBid<'info> {
    pub fn withdraw_escrow_sol_ctx(&self) -> CpiContext<'_, '_, '_, 'info, WithdrawSol<'info>> {
        let cpi_program = self.soundwork_list.to_account_info();

        let cpi_accounts = WithdrawSol {
            payer: self.bidder.to_account_info(),
            authority: self.bidder.to_account_info(),
            sol_escrow_wallet: self.sol_escrow_wallet.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}
