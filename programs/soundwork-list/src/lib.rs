use anchor_lang::prelude::*;
#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

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

    /// list an SPL NFT on soundwork by moving NFT to our asset manager
    /// create an `listingData` account to hold price,
    pub fn list_nft(ctx: Context<CreateListing>, lamports: u64) -> Result<()> {
        CreateListing::create_listing(ctx, lamports)
    }

    /// edit listing, by updating the `listingData` account information
    pub fn edit_listing(ctx: Context<EditListing>, lamports: u64) -> Result<()> {
        EditListing::edit_listing(ctx, lamports)
    }

    /// remove listing by closing the `listingData` account
    /// and transfer NFT from soundwork to user
    pub fn delete_listing(ctx: Context<DeleteListing>) -> Result<()> {
        DeleteListing::delete_listing(ctx)
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

    /// list a compressed NFT
    pub fn list_compressed(
        ctx: Context<ListCompressed>,
        params: ListCompressedParams,
    ) -> Result<()> {
        ListCompressed::list_compressed(ctx, params)
    }

    /// Delete compressed NFT listing
    pub fn delist_compressed(
        ctx: Context<DelistCompressed>,
        params: DelistCompressedParams,
    ) -> Result<()> {
        DelistCompressed::delist_compressed(ctx, params)
    }
}
