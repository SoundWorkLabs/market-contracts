use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    // helpers::transfer_nft,
    state::listing::{AssetManagerV1, ListingDataV1},
};

#[derive(Accounts)]
pub struct EditListing<'info> {
    #[account(mut, address = listing_data.owner)]
    pub authority: Signer<'info>,

    #[account(
        mut,  
        token::mint = mint,
        token::authority = authority
    )]
    pub authority_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub asset_manager: Account<'info, AssetManagerV1>,

    #[account(
        mut, 
        token::mint = mint,
        token::authority = asset_manager
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub listing_data: Account<'info, ListingDataV1>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn edit_listing_handler(ctx: Context<EditListing>, lamports: u64) -> Result<()> {
    // signer seeds
    let listing_data_acc = &mut ctx.accounts.listing_data;
    listing_data_acc.lamports = lamports;
   
    Ok(())
}