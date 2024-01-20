use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    helpers::transfer_nft,
    state::listing::{AssetManagerV1, ListingDataV1},
};

#[derive(Accounts)]
pub struct DeleteListing<'info> {
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

    #[account(mut, close = authority)]
    pub listing_data: Account<'info, ListingDataV1>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn delete_listing_handler(ctx: Context<DeleteListing>) -> Result<()> {
    // signer seeds
    let (_, bump) = Pubkey::find_program_address(&[b"soundwork".as_ref()], ctx.program_id);
    let asset_manager_seeds = &[b"soundwork".as_ref(), &[bump]];
    let asset_manager_signer = &[&asset_manager_seeds[..]];

    transfer_nft(
        ctx.accounts.vault_token_account.to_account_info(),
        ctx.accounts.authority_token_account.to_account_info(),
        ctx.accounts.mint.clone(),
        ctx.accounts.asset_manager.to_account_info(),
        ctx.accounts.token_program.clone(),
        asset_manager_signer,
    )?;

    // todo (Jimi) remove delegation ?

    Ok(())
}
