use anchor_lang::prelude::*;
use soundwork_list::{
    self,
    cpi::accounts::WithdrawSol,
    error::CustomError,
    program::SoundworkList,
    state::{listing::ListingDataV1, sol_escrow::SolEscrowWallet},
};

use crate::state::bid::BiddingDataV1;

#[derive(Accounts)]
pub struct RejectBid<'info> {
    // he will pay for the tx when he accepts a bid
    #[account(
        mut,
        address = listing_data_acc.owner.key() @ CustomError::UnrecognizedSigner
    )]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub listing_data_acc: Account<'info, ListingDataV1>,

    /// CHECK: checked when passing from PDA
    #[account(mut)]
    pub buyer: AccountInfo<'info>,

    #[account(mut)]
    pub buyer_sol_escrow: Account<'info, SolEscrowWallet>,

    #[account(
        mut,
        close = buyer
    )]
    pub bidding_data_acc: Account<'info, BiddingDataV1>,

    pub soundwork_list: Program<'info, SoundworkList>,
    pub system_program: Program<'info, System>,
}

pub fn reject_bid_handler(ctx: Context<RejectBid>) -> Result<()> {
    let bidding_data_acc = &ctx.accounts.bidding_data_acc;

    soundwork_list::cpi::withdraw_sol(
        ctx.accounts.withdraw_escrow_sol_ctx(),
        bidding_data_acc.lamports.into(),
    )?;

    Ok(())
}

impl<'info> RejectBid<'info> {
    pub fn withdraw_escrow_sol_ctx(&self) -> CpiContext<'_, '_, '_, 'info, WithdrawSol<'info>> {
        let cpi_program = self.soundwork_list.to_account_info();

        let cpi_accounts = WithdrawSol {
            payer: self.seller.to_account_info(),
            authority: self.buyer.to_account_info(),
            sol_escrow_wallet: self.buyer_sol_escrow.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}
