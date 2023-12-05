// buy a listed NFT

use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, token::{ TokenAccount, Mint, Token } };

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
        // constraint = buyer.lamports() > listing_data.lamports @ CustomError::InsufficientFunds
    )]
    pub buyer: Signer<'info>,

    /// CHECK: program account
    #[account(
        mut, 
        // constraint = escrow_wallet_as_buyer.lamports() > listing_data.lamports @ CustomError::InsufficientFunds
    )]
    pub escrow_wallet_as_buyer: AccountInfo<'info>,

    /// CHECK: address of NFT lister initialized in listing data account
    #[account(mut, address = listing_data.owner.key())]
    pub og_owner: AccountInfo<'info>,

    #[account(mut)]
    pub asset_manager: Account<'info, AssetManagerV1>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>, // asset manager ATA that will hold all nfts

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::authority = buyer,
        associated_token::mint = mint
    )]
    pub buyer_token_account: Account<'info, TokenAccount>, // buyer ATA

    #[account(
        mut,
        address = listing_data.mint
    )]
    pub mint: Account<'info, Mint>,

    #[account(mut, close = og_owner)]
    pub listing_data: Account<'info, ListingDataV1>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy_listing_handler(ctx: Context<BuyListing>) -> Result<()> {
    // ! derive asset manager seeds
    let (_, bump) = Pubkey::find_program_address(&[b"soundwork".as_ref()], ctx.program_id);
    let asset_manager_seeds = &[b"soundwork".as_ref(), &[bump]];
    let asset_manager_signer = &[&asset_manager_seeds[..]];

    // todo(Jimii): protocol fees
    // ! check if we want to use the user's escrow wallet as signer
    if ctx.accounts.escrow_wallet_as_buyer.owner == ctx.program_id {
        ctx.accounts.escrow_wallet_as_buyer.sub_lamports(ctx.accounts.listing_data.lamports)?;
        ctx.accounts.og_owner.add_lamports(ctx.accounts.listing_data.lamports)?;
    } else {
        transfer_lamports(
            &ctx.accounts.buyer,
            &ctx.accounts.og_owner,
            &ctx.accounts.system_program,
            ctx.accounts.listing_data.lamports
        )?;
    }

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
