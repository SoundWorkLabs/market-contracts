use anchor_lang::prelude::*;

#[account]
pub struct SolEscrowWallet {
    pub owner: Pubkey,
}

impl SolEscrowWallet {
    pub const LEN: usize = (8 + 32);

    pub fn new(owner: Pubkey) -> Self {
        Self { owner }
    }
}
