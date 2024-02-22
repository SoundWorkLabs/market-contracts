use anchor_lang::prelude::*;
use mpl_bubblegum::instructions::TransferCpiBuilder;

use crate::{
    helpers::{MplBubblegum, Noop, SplAccountCompression},
    state::listing::{AssetManagerV1, ListingDataCompressed},
};

#[derive(Accounts)]
#[instruction(params: DelistCompressedParams)]
pub struct DelistCompressed<'info> {
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
pub struct DelistCompressedParams {
    root: [u8; 32],
    data_hash: [u8; 32],
    creator_hash: [u8; 32],
    nonce: u64,
    index: u32,
}

impl DelistCompressed<'_> {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// creates a listing
    #[access_control(ctx.accounts.validate())]
    pub fn delist_compressed(
        ctx: Context<DelistCompressed>,
        params: DelistCompressedParams,
    ) -> Result<()> {
        // signer seeds
        let (_, bump) = Pubkey::find_program_address(&[b"soundwork".as_ref()], ctx.program_id);
        let asset_manager_seeds = &[b"soundwork".as_ref(), &[bump]];
        let asset_manager_signer = &[&asset_manager_seeds[..]];

        let DelistCompressedParams {
            root,
            data_hash,
            creator_hash,
            nonce,
            index,
        } = params;

        // transfer the compressed NFT back to owner
        TransferCpiBuilder::new(&ctx.accounts.bubblegum_program.to_account_info())
            .tree_config(&ctx.accounts.tree_config)
            .leaf_owner(&ctx.accounts.asset_manager.to_account_info(), true)
            // .leaf_delegate(leaf_delegate, as_signer) // skip me
            .new_leaf_owner(&ctx.accounts.authority)
            .merkle_tree(&ctx.accounts.merkle_tree)
            .log_wrapper(&ctx.accounts.log_wrapper)
            .compression_program(&ctx.accounts.compression_program)
            .system_program(&ctx.accounts.system_program)
            .root(root)
            .data_hash(data_hash)
            .creator_hash(creator_hash)
            .nonce(nonce)
            .index(index)
            .invoke_signed(asset_manager_signer)?;

        Ok(())
    }
}
