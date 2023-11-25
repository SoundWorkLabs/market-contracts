use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use soundwork_list::{
    self,
    cpi::accounts::BuyListing,
    program::SoundworkList,
    state::{
        listing::{AssetManagerV1, ListingDataV1},
        sol_escrow::SolEscrowWallet,
    },
};

use crate::state::bid::BiddingDataV1;

#[derive(Accounts)]
pub struct AcceptBid<'info> {
    // he will pay for the tx when he accepts a bid
    #[account(
        mut,
        address = listing_data_acc.owner.key()
    )]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub listing_data_acc: Account<'info, ListingDataV1>,

    #[account(
        mut,
        close = buyer
    )]
    pub bidding_data_acc: Account<'info, BiddingDataV1>,

    /// CHECK: checked when passing from PDA
    #[account(mut)]
    pub buyer: AccountInfo<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub buyer_sol_escrow: Account<'info, SolEscrowWallet>,

    #[account(
        init_if_needed,
        payer = seller, 
        associated_token::authority = buyer,
        associated_token::mint = mint
    )]
    pub buyer_token_acc: Account<'info, TokenAccount>,

    #[account(mut)]
    pub asset_manager: Account<'info, AssetManagerV1>,

    #[account(mut)]
    pub vault_token_acc: Account<'info, TokenAccount>,

    pub soundwork_list: Program<'info, SoundworkList>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn accept_bid_handler(ctx: Context<AcceptBid>) -> Result<()> {
    let listing_data = &ctx.accounts.listing_data_acc;
    let bidding_data = &ctx.accounts.bidding_data_acc;

    // todo(Jimii): Expiry
    // - time it expires

    // transfer nft bidder and send fees to buyer
    soundwork_list::cpi::buy_listing(ctx.accounts.buy_listing_ctx())?;

    Ok(())
}

/// cpi call to list program
impl<'info> AcceptBid<'info> {
    pub fn buy_listing_ctx(&self) -> CpiContext<'_, '_, '_, 'info, BuyListing<'info>> {
        let cpi_program = self.soundwork_list.to_account_info();
        let cpi_accounts = BuyListing {
            buyer: self.seller.to_account_info(), // ! because he is paying for the tx
            escrow_wallet_as_buyer: self.buyer_sol_escrow.to_account_info(),
            og_owner: self.seller.to_account_info(),
            asset_manager: self.asset_manager.to_account_info(),
            vault_token_account: self.vault_token_acc.to_account_info(),
            buyer_token_account: self.buyer_token_acc.to_account_info(),
            mint: self.mint.to_account_info(),
            listing_data: self.listing_data_acc.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}
