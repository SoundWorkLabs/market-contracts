use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, TransferChecked},
};

use crate::state::listing::{AssetManagerV1, ListingDataV1};

#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = authority_token_account.amount == 1,
        constraint = authority_token_account.owner == authority.key(),
        token::mint = mint,
        token::authority = authority,
    )]
    pub authority_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    // init if needed. this will almost always be initialized
    #[account(
        init_if_needed,
        payer = authority,
        space = AssetManagerV1::LEN,
        seeds = [b"soundwork"],
        bump
    )]
    pub asset_manager: Account<'info, AssetManagerV1>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::authority = asset_manager,
        associated_token::mint = mint
    )]
    pub vault_token_account: Account<'info, TokenAccount>, // asset manager token acc that will hold all nfts

    ////////////////////////////////////////////////////////////////////////////
    // Auto derived for single listing data.
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
    pub associated_token_program: Program<'info, AssociatedToken>, // ! new
    pub system_program: Program<'info, System>,
}

impl<'info> CreateListing<'info> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// creates a listing
    #[access_control(ctx.accounts.validate())]
    pub fn create_listing(ctx: Context<CreateListing>, lamports: u64) -> Result<()> {
        // insert data into listing data account
        let listing_data_acc = &mut ctx.accounts.listing_data;
        **listing_data_acc = ListingDataV1::new(
            lamports,
            ctx.accounts.authority.key(),
            ctx.accounts.mint.key(),
        );

        // signer seeds
        let (_, bump) = Pubkey::find_program_address(&[b"soundwork".as_ref()], ctx.program_id);
        let asset_manager_seeds = &[b"soundwork".as_ref(), &[bump]];
        let asset_manager_signer = &[&asset_manager_seeds[..]];

        // // delegate authority to asset manager
        // delegate_nft(
        //     ctx.accounts.authority.to_account_info(),
        //     ctx.accounts.asset_manager.to_account_info(),
        //     ctx.accounts.authority_token_account.to_account_info(),
        //     ctx.accounts.mint.clone(),
        //     ctx.accounts.token_program.clone(),
        // )?;

        // transfer token to asset manager vault
        let cpi_accounts = TransferChecked {
            from: ctx.accounts.authority_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> =
            CpiContext::new(cpi_program, cpi_accounts);

        token::transfer_checked(cpi_context, 1, 0)?;

        Ok(())
    }
}

//
