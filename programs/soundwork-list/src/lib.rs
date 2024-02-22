use anchor_lang::prelude::*;

pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("EUmBNHvFqhkA6Uaqv6pDup5ESHKCqoAweQ4kzAMjNZhX");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Soundwork List Program",
    project_url: "https://soundwork.io",
    contacts: "email:info@soundwork.io, twitter:@soundworkio",
    policy: "https://github.com/SoundWorkLabs/market-contracts/blob/master/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/SoundWorkLabs/market-contracts"
}

#[program]
pub mod soundwork_list {

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
    pub fn delete_listing(ctx: Context<DeleteListing>) -> Result<()> {
        instructions::delete_listing_handler(ctx)
    }

    /// buy an NFT from soundwork
    /// transfer NFT to user if he has funds to purchase the NFT
    pub fn buy_listing(ctx: Context<BuyListing>, bid_amt: Option<u64>) -> Result<()> {
        instructions::buy_listing_handler(ctx, bid_amt)
    }

    /// transfer lamports to the escrow wallet
    pub fn deposit_sol(ctx: Context<DepositSol>, lamports: u64) -> Result<()> {
        instructions::deposit_sol_handler(ctx, lamports)
    }

    /// withdraw sol from the user escrow
    pub fn withdraw_sol(ctx: Context<WithdrawSol>, lamports: Option<u64>) -> Result<()> {
        instructions::withdraw_sol_handler(ctx, lamports)
    }
}
