use anchor_lang::prelude::*;

use crate::state::bid::BiddingDataV1;
use soundwork_list::{
    cpi::accounts::DepositSol, error::CustomError, program::SoundworkList,
    state::sol_escrow::SolEscrowWallet,
};

#[derive(Accounts)]
pub struct EditBid<'info> {
    #[account(
        mut,
        address = bidding_data_acc.owner @ CustomError::UnrecognizedSigner
    )]
    pub bidder: Signer<'info>,

    #[account(mut)]
    pub bidding_data_acc: Account<'info, BiddingDataV1>,

    #[account(mut)]
    pub sol_escrow_wallet: Account<'info, SolEscrowWallet>,

    pub system_program: Program<'info, System>,
    pub soundwork_list: Program<'info, SoundworkList>,
}

pub fn edit_bid_handler(
    ctx: Context<EditBid>,
    new_expires: Option<i64>,
    new_lamports: Option<u64>,
) -> Result<()> {
    let mut bidding_data_acc = ctx.accounts.bidding_data_acc.clone();

    if let Some(expire_ts) = new_expires {
        bidding_data_acc.expires_ts = expire_ts;
    }

    if let Some(lamports) = new_lamports {
        if lamports > bidding_data_acc.lamports {
            let diff = lamports.checked_sub(bidding_data_acc.lamports);
            if diff.is_none() {
                return Err(error!(CustomError::Overflow));
            }

            soundwork_list::cpi::deposit_sol(ctx.accounts.update_escrow_diff_ctx(), diff.unwrap())?;

            bidding_data_acc.lamports += lamports;
        }

        bidding_data_acc.lamports = lamports;
    }

    Ok(())
}

impl<'info> EditBid<'info> {
    pub fn update_escrow_diff_ctx(&self) -> CpiContext<'_, '_, '_, 'info, DepositSol<'info>> {
        let cpi_program = self.soundwork_list.to_account_info();
        let cpi_accounts = DepositSol {
            owner: self.bidder.to_account_info(),
            sol_escrow_wallet: self.sol_escrow_wallet.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
}
