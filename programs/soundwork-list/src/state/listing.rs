use anchor_lang::prelude::*;

#[account]
pub struct AssetManagerV1 {
    //
}

impl AssetManagerV1 {
    pub const LEN: usize = 8 + (8);
}

#[account]
pub struct ListingDataV1 {
    pub lamports: u64,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub created_ts: i64,
}

impl ListingDataV1 {
    pub const LEN: usize = 8 + (8 + 32 + 32 + 8);

    pub fn new(lamports: u64, owner: Pubkey, mint: Pubkey) -> Self {
        let created_ts = Clock::get().unwrap().unix_timestamp;

        Self {
            lamports,
            owner,
            created_ts,
            mint,
        }
    }
}

#[account]
pub struct ListingDataCompressed {
    /// Amount in lamports the asset is being listed for
    pub lamports: u64,
    /// The owner of the asset
    pub owner: Pubkey,
    /// compressed asset ID
    pub asset_id: Pubkey,
    /// creation timestamp for the listing
    pub created_ts: i64,
    /// Unused reserved byte space for future additive changes.
    pub _reserved1: [u8; 32],
    /// Unused reserved byte space for future additive changes.
    pub _reserved2: [u8; 32],
}

impl ListingDataCompressed {
    pub const LEN: usize = 8 + (8 + 32 + 32 + 8) + 32 + 32;

    pub fn new(lamports: u64, owner: Pubkey, asset_id: Pubkey) -> Self {
        let created_ts = Clock::get().unwrap().unix_timestamp;

        Self {
            lamports,
            owner,
            created_ts,
            asset_id,

            _reserved1: [0; 32],
            _reserved2: [0; 32],
        }
    }
}
