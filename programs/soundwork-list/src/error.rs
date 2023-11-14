use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Insufficient funds to complete the transaction")]
    InsufficientFunds,
    #[msg("You have already listed this NFT. Consider editing the listing instead")]
    NFTAlreadyListed,
}