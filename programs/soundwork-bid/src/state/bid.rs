use anchor_lang::prelude::*;

#[account]
pub struct BiddingDataV1 {
    pub expires_ts: i64,
    pub lamports: u64,
    pub owner: Pubkey,
}

impl BiddingDataV1 {
    pub const LEN: usize = 8 + (8 + 8 + 32);

    pub fn new(lamports: u64, expires_ts: i64, owner: Pubkey) -> Self {
        Self {
            lamports,
            expires_ts,
            owner
        }
    }
}
