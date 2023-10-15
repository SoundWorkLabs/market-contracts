use anchor_lang::prelude::*;


#[account]
pub struct AssetManager {
    //
}

impl AssetManager {
    pub const LEN: usize = 8 + (8);
}


#[account]
pub struct ListingDataV1 {
    pub price: f64,
    created_ts: i64,
}

impl ListingDataV1 {
    pub const LEN: usize = 8 + (8 + 8);

    pub fn new(price: f64) -> Self {
        let created_ts = Clock::get().unwrap().unix_timestamp;

        Self { price, created_ts }
    }
}
