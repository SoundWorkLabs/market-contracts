// buy a listed NFT

use anchor_lang::prelude::*;
use anchor_spl::token::{ TokenAccount, Mint, Token };

use crate::{
    state::listing::{ AssetManagerV1, ListingDataV1 },
    helpers::{ transfer_nft, transfer_lamports },
    error::CustomError,
};

#[derive(Accounts)]
pub struct BuyListing<'info> {
    // user buying the NFT
    #[account(
        mut, 
        constraint = buyer.lamports() > listing_data.lamports @ CustomError::InsufficientFunds
     )]
    pub buyer: Signer<'info>,

    /// CHECK: address of NFT lister initialized in listing data account
    #[account(mut, address = listing_data.owner)]
    pub og_owner: AccountInfo<'info>,

    #[account(mut)]
    pub asset_manager: Account<'info, AssetManagerV1>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>, // asset manager acc that will hold all nfts

    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>, // asset manager acc that will hold all nfts

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut, close = og_owner)]
    pub listing_data: Account<'info, ListingDataV1>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn buy_listing_handler(ctx: Context<BuyListing>) -> Result<()> {
    // signer seeds
    let (_, bump) = Pubkey::find_program_address(&[b"soundwork".as_ref()], ctx.program_id);
    let asset_manager_seeds = &[b"soundwork".as_ref(), &[bump]];
    let asset_manager_signer = &[&asset_manager_seeds[..]];

    // we check if he has enough lamports to purchase it
    msg!("balance: ref ->  {}", ctx.accounts.buyer.lamports());
    msg!("balance: modifiable -> {:?}", ctx.accounts.buyer.lamports);

    // ! remove me
    msg!("this is the price set {:?}", ctx.accounts.listing_data.lamports);
    msg!("this is the original owner {:?}", ctx.accounts.listing_data.owner);

    // we transfer some lamports to owner, take protocol fees and also check for
    transfer_lamports(
        &ctx.accounts.buyer,
        &ctx.accounts.og_owner,
        &ctx.accounts.system_program,
        ctx.accounts.listing_data.lamports
    )?;

    // todo (Jimii) royalty enforcement

    // we transfer the NFT to buyer
    transfer_nft(
        ctx.accounts.vault_token_account.to_account_info(),
        ctx.accounts.buyer_token_account.to_account_info(),
        ctx.accounts.mint.clone(),
        ctx.accounts.asset_manager.to_account_info(),
        ctx.accounts.token_program.clone(),
        asset_manager_signer
    )?;

    // todo (Jimii) protocol fees

    Ok(())
}
