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
    created_ts: i64,
}

impl ListingDataV1 {
    pub const LEN: usize = 8 + (8 + 8 + 32);

    pub fn new(lamports: u64, owner: Pubkey) -> Self {
        let created_ts = Clock::get().unwrap().unix_timestamp;

        Self { lamports, owner, created_ts }
    }
}
