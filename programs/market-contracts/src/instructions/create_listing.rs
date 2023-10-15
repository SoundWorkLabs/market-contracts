use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{
    helpers::{delegate_nft, transfer_nft},
    state::listing::{AssetManager, ListingDataV1},
};

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = authority_token_account.amount == 1,
        constraint = authority_token_account.owner == authority.key()
    )]
    pub authority_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    // init if needed. this will almost always be initialized
    #[account(
        init_if_needed,
        payer = authority,
        space = AssetManager::LEN,
        seeds = [b"soundwork"],
        bump
    )]
    pub asset_manager: Account<'info, AssetManager>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [asset_manager.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = asset_manager
    )]
    pub vault_token_account: Account<'info, TokenAccount>, // asset manager token acc that will hold all nfts

    ////////////////////////////////////////////////////////////////////////////
    // Auto derived for single listing.
    ////////////////////////////////////////////////////////////////////////////
    #[account(
        init_if_needed,
        payer = authority,
        space = ListingDataV1::LEN,
        seeds = [mint.key().as_ref(), "ryo".as_bytes()],
        bump
    )]
    pub listing_data: Account<'info, ListingDataV1>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn create_listing_handler(ctx: Context<CreateListing>, price: f64) -> Result<()> {
    // insert data into listing data account
    let listing_data_acc = &mut ctx.accounts.listing_data.clone();
    **listing_data_acc = ListingDataV1::new(price);

    // signer seeds
    let (_, bump) = Pubkey::find_program_address(&[b"soundwork".as_ref()], ctx.program_id);
    let asset_manager_seeds = &[b"soundwork".as_ref(), &[bump]];
    let asset_manager_signer = &[&asset_manager_seeds[..]];

    // delegate authority to asset manager
    delegate_nft(
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.asset_manager.to_account_info(),
        ctx.accounts.authority_token_account.to_account_info(),
        ctx.accounts.mint.clone(),
        ctx.accounts.token_program.clone(),
    )?;

    msg!("transfer nft to asset manager vault");

    // we transfer the NFT to asset manager
    transfer_nft(
        ctx.accounts.authority_token_account.to_account_info(),
        ctx.accounts.vault_token_account.to_account_info(),
        ctx.accounts.mint.clone(),
        ctx.accounts.authority.to_account_info(),
        ctx.accounts.token_program.clone(),
        asset_manager_signer,
    )?;

    // ! remove below
    msg!("transfer the nft back to user");
    transfer_nft(
        ctx.accounts.vault_token_account.to_account_info(),
        ctx.accounts.authority_token_account.to_account_info(),
        ctx.accounts.mint.clone(),
        ctx.accounts.asset_manager.to_account_info(),
        ctx.accounts.token_program.clone(),
        asset_manager_signer,
    )?;

    Ok(())
}
