use anchor_lang::prelude::*;

pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("EUmBNHvFqhkA6Uaqv6pDup5ESHKCqoAweQ4kzAMjNZhX");

#[program]
pub mod market_contracts {

    use super::*;

    /// list an NFT on soundwork by moving NFT to our asset manager
    /// create an `listingData` account to hold price,
    pub fn list_nft(ctx: Context<CreateListing>, lamports: u64) -> Result<()> {
        instructions::create_listing_handler(ctx, lamports)
    }

    /// edit listing, by updating the `listingData` account information
    pub fn edit_listing(ctx: Context<EditListing>, lamports: u64) -> Result<()> {
        instructions::edit_listing_handler(ctx, lamports)
    }

    /// remove listing by closing the `listingData` account
    /// and transfer NFT from soundwork to user
    pub fn remove_listing(ctx: Context<DeleteListing>) -> Result<()> {
        instructions::delete_listing_handler(ctx)
    }

    /// buy an NFT from soundwork
    /// transfer NFT to user if he has funds to purchase the NFT
    pub fn buy_listing(ctx: Context<BuyListing>) -> Result<()> {
        instructions::buy_listing_handler(ctx)
    }
}
