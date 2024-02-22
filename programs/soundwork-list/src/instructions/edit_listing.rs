use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    error::CustomError,
    state::listing::{AssetManagerV1, ListingDataV1},
};

#[derive(Accounts)]
pub struct EditListing<'info> {
    #[account(mut, address = listing_data.owner @ CustomError::UnrecognizedSigner)]
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

impl EditListing<'_> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Edits a listing
    #[access_control(ctx.accounts.validate())]
    pub fn edit_listing(ctx: Context<EditListing>, lamports: u64) -> Result<()> {
        let listing_data_acc = &mut ctx.accounts.listing_data;
        listing_data_acc.lamports = lamports;

        Ok(())
    }
}
