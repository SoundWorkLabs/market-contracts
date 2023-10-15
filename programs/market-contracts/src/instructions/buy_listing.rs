// buy a listed NFT

use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

use crate::state::listing::{AssetManager, ListingDataV1};

#[derive(Accounts)]
pub struct BuyListing<'info> {
    // user buying the NFT
    #[account(mut)]
    pub authority: Signer<'info>,

    ////////////////////////////////////////////////////////////////////////////
    // Auto derived below.
    ////////////////////////////////////////////////////////////////////////////
    pub assetManager: Account<'info, AssetManager>,

    pub vaultTokenAccount: Account<'info, TokenAccount>, // asset manager acc that will hold all nfts

    pub listingData: Account<'info, ListingDataV1>,

    pub system_program: Program<'info, System>,
}

pub fn buy_listing_handler(ctx: Context<BuyListing>) -> Result<()> {
    // user submits tx to buy the NFT

    // we check if he has enough lamports to purchase it
    
    // we transfer the NFT to the user 

    // we transfer some lamports to owner, take protocol fees and also check for 
    // royalty enforcement

    // we transfer the NFT to buyer

    Ok(())
}
