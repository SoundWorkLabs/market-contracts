use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Signer address does not math the initializer address")]
    UnrecognizedSigner,
    #[msg("Insufficient funds to complete the transaction")]
    InsufficientFunds,
    #[msg("Expected Authority not found")]
    InvalidAuthority,
    #[msg("You have already listed this NFT. Consider editing the listing instead")]
    NFTAlreadyListed,
}