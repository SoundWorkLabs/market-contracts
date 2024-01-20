use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("CVuoapcC1RG8y1m86eChkXmynPi4FaykDC8jM8soMZ4j");

#[program]
pub mod soundwork_bid {
    use super::*;

    /// make a bid for an nft
    pub fn make_bid(ctx: Context<CreateBid>, lamports: u64, expires_ts: i64) -> Result<()> {
        instructions::create_bid_handler(ctx, lamports, expires_ts)
    }

    pub fn edit_bid(
        ctx: Context<EditBid>,
        new_lamports: Option<u64>,
        new_expires: Option<i64>,
    ) -> Result<()> {
        instructions::edit_bid_handler(ctx, new_expires, new_lamports)
    }

    /// deletes a bid for an nft
    pub fn delete_bid(ctx: Context<DeleteBid>) -> Result<()> {
        instructions::delete_bid_handler(ctx)
    }

    /// accepts bid from user
    pub fn accept_bid(ctx: Context<AcceptBid>) -> Result<()> {
        instructions::accept_bid_handler(ctx)
    }

    /// reject a user's bid
    pub fn reject_bid(ctx: Context<RejectBid>) -> Result<()> {
        instructions::reject_bid_handler(ctx)
    }
}
