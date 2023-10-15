use anchor_lang::prelude::*;

pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

// use instructions::create_listing::CreateListing;
use instructions::*;
use state::*;

declare_id!("HQZV1Ukjs5bJGaZjZoijVyK1HBBqzwsZX9eAwmkvTG4u");

#[program]
pub mod market_contracts {

    use super::*;

    /// list an NFT on soundwork by moving NFT to our asset manager
    /// create an `listingData` account to hold price,
    pub fn list_nft(ctx: Context<CreateListing>, price: f64) -> Result<()> {
        instructions::create_listing_handler(ctx, price)
    }

    // /// edit listing, by updating the `listingData` account information
    // pub fn edit_listing() {
    //     //
    // }

    // /// remove listing by closing the `listingData` account
    // /// and transfer NFT from soundwork to user
    // pub fn remove_listing() {
    //     //
    // }

    // /// buy an NFT from soundwork
    // /// transfer NFT to user if he has funds to purchase the NFT
    // pub fn buy_listing() {
    //     //
    // }
}
