use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    error::CustomError,
    helpers::{transfer_lamports, transfer_nft},
    state::listing::{AssetManagerV1, ListingDataV1},
};

#[derive(Accounts)]
pub struct BuyListing<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: user buying the NFT
    #[account(mut)]
    pub buyer: AccountInfo<'info>,

    /// CHECK: program account
    #[account(mut)]
    pub escrow_wallet_as_buyer: Option<UncheckedAccount<'info>>,

    /// CHECK: address of NFT lister initialized in listing data account
    #[account(mut, address = listing_data.owner.key())]
    pub og_owner: SystemAccount<'info>,

    #[account(mut)]
    pub asset_manager: Account<'info, AssetManagerV1>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>, // asset manager ATA that will hold all nfts

    #[account(
        init_if_needed,
        payer = payer,
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

    /// CHECK: only used when accept bid is called
    #[account(mut)]
    pub bid_data_acc: Option<UncheckedAccount<'info>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn buy_listing_handler(ctx: Context<BuyListing>) -> Result<()> {
    let bid_data_acc = ctx.accounts.bid_data_acc.as_ref();
    let escrow_wallet = ctx.accounts.escrow_wallet_as_buyer.as_ref();

    // ! derive asset manager seeds
    let (_, bump) = Pubkey::find_program_address(&[b"soundwork".as_ref()], ctx.program_id);
    let asset_manager_seeds = &[b"soundwork".as_ref(), &[bump]];
    let asset_manager_signer = &[&asset_manager_seeds[..]];

    // when escrow account if provided, buyer wants to pay using his escrow,
    // else buyer has to use his wallet
    // NOTE: we have lamports checks here instead of using constraints because of mutual exclusivity of the
    // escrow_wallet and buyer accounts.
    if escrow_wallet.is_some() {
        if escrow_wallet.unwrap().lamports() < ctx.accounts.listing_data.lamports {
            return Err(error!(CustomError::InsufficientFunds));
        }

        // buying at bid price
        escrow_wallet
            .unwrap()
            .sub_lamports(bid_data_acc.unwrap().lamports())?; //okay to unwrap here. 
        ctx.accounts
            .og_owner
            .add_lamports(bid_data_acc.unwrap().lamports())?; // okay to unwrap here
    } else {
        if ctx.accounts.buyer.lamports() < ctx.accounts.listing_data.lamports {
            return Err(error!(CustomError::InsufficientFunds));
        }

        // buying at full price
        transfer_lamports(
            &ctx.accounts.buyer,
            &ctx.accounts.og_owner,
            &ctx.accounts.system_program,
            ctx.accounts.listing_data.lamports,
        )?;
    }

    // we transfer the NFT to buyer
    transfer_nft(
        ctx.accounts.vault_token_account.to_account_info(),
        ctx.accounts.buyer_token_account.to_account_info(),
        ctx.accounts.mint.clone(),
        ctx.accounts.asset_manager.to_account_info(),
        ctx.accounts.token_program.clone(),
        asset_manager_signer,
    )?;

    // todo (Jimii) protocol fees
    // todo (Jimii) royalty enforcement

    Ok(())
}
