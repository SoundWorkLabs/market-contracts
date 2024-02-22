use anchor_lang::prelude::*;
#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("CVuoapcC1RG8y1m86eChkXmynPi4FaykDC8jM8soMZ4j");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Soundwork Bid Program",
    project_url: "https://soundwork.io",
    contacts: "email:info@soundwork.io, twitter:@soundworkio",
    policy: "https://github.com/SoundWorkLabs/market-contracts/blob/master/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/SoundWorkLabs/market-contracts"
}

#[program]
pub mod soundwork_bid {
    use super::*;

    /// make a bid for an nft
    pub fn make_bid(ctx: Context<CreateBid>, lamports: u64, expires_ts: i64) -> Result<()> {
        instructions::create_bid_handler(ctx, lamports, expires_ts)
    }

    pub fn edit_bid(
        ctx: Context<EditBid>,
        new_expires: Option<i64>,
        new_lamports: Option<u64>,
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
