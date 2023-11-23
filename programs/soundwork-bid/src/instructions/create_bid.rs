use anchor_lang::prelude::*;
use anchor_spl::token::Mint ;

use crate::state::bid::BiddingDataV1;
use soundwork_list::{
    self,
    cpi::accounts::DepositSol,
    program::SoundworkList,
    state::listing::ListingDataV1,
    error::CustomError,
};

#[derive(Accounts)]
pub struct 
CreateBid<'info> {
    #[account(
        mut,
        constraint = bidder.lamports() > listing_data_acc.lamports @CustomError::InsufficientFunds 
    )]
    pub bidder: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = bidder,
        space = BiddingDataV1::LEN,
        seeds = [mint.key().as_ref(), "Ikuyo".as_bytes()],
        bump
    )]
    pub bidding_data_acc: Account<'info, BiddingDataV1>,

    #[account(mut)]
    pub listing_data_acc: Account<'info, ListingDataV1>,

    /// CHECK: initialized by list program through the CPI
    #[account(
        mut,
        owner = soundwork_list.key()
    )]
    pub sol_escrow_wallet: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub soundwork_list: Program<'info, SoundworkList>,
}

pub fn create_bid_handler(ctx: Context<CreateBid>, lamports: u64, expires_ts: i64) -> Result<()> {
    let bidding_data_acc = &mut ctx.accounts.bidding_data_acc;

    // save bid data
    **bidding_data_acc = BiddingDataV1::new(lamports, expires_ts, ctx.accounts.bidder.key());

    // todo (Jimii) : check the 1000 extra
    soundwork_list::cpi::deposit_sol(
        ctx.accounts.initialize_escrow_ctx(), ctx.accounts.listing_data_acc.lamports + 1000
    )?;

    Ok(())
}

impl <'info> CreateBid <'info> {
    pub fn initialize_escrow_ctx(&self) -> CpiContext<'_, '_, '_, 'info, DepositSol<'info>> { 
        let cpi_program = self.soundwork_list.to_account_info();
        let cpi_accounts = DepositSol {
            owner: self.bidder.to_account_info(),
            sol_escrow_wallet: self.sol_escrow_wallet.to_account_info(),
            system_program: self.system_program.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)

    }
}