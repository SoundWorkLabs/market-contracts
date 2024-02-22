use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

use crate::{
    error::CustomError,
    state::listing::{AssetManagerV1, ListingDataV1},
};

#[derive(Accounts)]
pub struct DeleteListing<'info> {
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

    #[account(mut, close = authority)]
    pub listing_data: Account<'info, ListingDataV1>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl DeleteListing<'_> {
    pub fn validate(&self) -> Result<()> {
        return Ok(());
    }

    /// Deletes a listing
    #[access_control(ctx.accounts.validate())]
    pub fn delete_listing(ctx: Context<DeleteListing>) -> Result<()> {
        // signer seeds
        let (_, bump) = Pubkey::find_program_address(&[b"soundwork".as_ref()], ctx.program_id);
        let asset_manager_seeds = &[b"soundwork".as_ref(), &[bump]];
        let asset_manager_signer = &[&asset_manager_seeds[..]];

        let cpi_accounts = TransferChecked {
            from: ctx.accounts.vault_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.authority_token_account.to_account_info(),
            authority: ctx.accounts.asset_manager.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> =
            CpiContext::new(cpi_program, cpi_accounts).with_signer(asset_manager_signer);

        token::transfer_checked(cpi_context, 1, 0)?;

        Ok(())
    }
}
