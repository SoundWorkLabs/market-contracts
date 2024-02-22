use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::TransferCpiBuilder;

use crate::{
    helpers::{MplBubblegum, Noop, SplAccountCompression},
    state::listing::{AssetManagerV1, ListingDataCompressed},
};

#[derive(Accounts)]
#[instruction(params: ListCompressedParams)]
pub struct ListCompressed<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: not really an account but the asset id
    pub asset_id: UncheckedAccount<'info>, //,

    /// CHECK: okay
    pub tree_config: AccountInfo<'info>,

    /// CHECK: okay
    pub merkle_tree: AccountInfo<'info>,

    // expecting this to already be initialized
    #[account(mut)]
    pub asset_manager: Account<'info, AssetManagerV1>,

    #[account(
        init,
        payer = authority,
        space = ListingDataCompressed::LEN,
        seeds = [asset_id.key().as_ref(), "ryo".as_bytes()],
        bump
    )]
    pub listing_data: Account<'info, ListingDataCompressed>,

    pub bubblegum_program: Program<'info, MplBubblegum>,
    pub compression_program: Program<'info, SplAccountCompression>,
    pub log_wrapper: Program<'info, Noop>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ListCompressedParams {
    lamports: u64,
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
}

impl ListCompressed<'_> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// creates a listing
    #[access_control(ctx.accounts.validate())]
    pub fn list_compressed(
        ctx: Context<ListCompressed>,
        params: ListCompressedParams,
    ) -> Result<()> {
        // insert data into listing data account
        let listing_data_acc = &mut ctx.accounts.listing_data;

        let ListCompressedParams {
            root,
            data_hash,
            creator_hash,
            nonce,
            index,
            ..
        } = params;

        **listing_data_acc = ListingDataCompressed::new(
            params.lamports,
            ctx.accounts.authority.key(),
            ctx.accounts.asset_id.key(),
        );

        // transfer the compressed NFT to our asset manager
        TransferCpiBuilder::new(&ctx.accounts.bubblegum_program.to_account_info())
            .tree_config(&ctx.accounts.tree_config)
            .leaf_owner(&ctx.accounts.authority, true)
            // .leaf_delegate(leaf_delegate, as_signer) // skip me
            .new_leaf_owner(&ctx.accounts.asset_manager.to_account_info())
            .merkle_tree(&ctx.accounts.merkle_tree)
            .log_wrapper(&ctx.accounts.log_wrapper)
            .compression_program(&ctx.accounts.compression_program)
            .system_program(&ctx.accounts.system_program)
            .root(root)
            .data_hash(data_hash)
            .creator_hash(creator_hash)
            .nonce(nonce)
            .index(index)
            .invoke()?;

        Ok(())
    }
}
