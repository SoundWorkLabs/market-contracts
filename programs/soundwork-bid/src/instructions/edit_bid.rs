use anchor_lang::prelude::*;

use crate::state::bid::BiddingDataV1;
use soundwork_list::error::CustomError;

#[derive(Accounts)]
pub struct EditBid<'info> {
    #[account(
        mut,
        address = bidding_data_acc.owner @ CustomError::UnrecognizedSigner
    )]
    pub bidder: Signer<'info>,

    #[account(mut)]
    pub bidding_data_acc: Account<'info, BiddingDataV1>,

    pub system_program: Program<'info, System>,
}

pub fn edit_bid_handler(
    ctx: Context<EditBid>,
    new_expiry: Option<i64>,
    new_lamports: Option<u64>,
) -> Result<()> {
    let bidding_data_acc = &mut ctx.accounts.bidding_data_acc;

    if let Some(expire_ts) = new_expiry {
        bidding_data_acc.expires_ts = expire_ts;
    }

    if let Some(lamports) = new_lamports {
        bidding_data_acc.lamports = lamports;
    }

    Ok(())
}
