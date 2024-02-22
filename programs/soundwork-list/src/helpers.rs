use anchor_lang::{
    prelude::{Account, AccountInfo, CpiContext, Program, Result},
    solana_program::pubkey::Pubkey,
    system_program::{self, System, Transfer as SolanaTransfer},
    ToAccountInfo,
};
use anchor_spl::token::{self, Mint, Token, TransferChecked as TokenTransferChecked};

#[derive(Clone)]
pub struct MplBubblegum;

impl anchor_lang::Id for MplBubblegum {
    fn id() -> Pubkey {
        mpl_bubblegum::ID
    }
}

#[derive(Clone)]
pub struct Noop;

impl anchor_lang::Id for Noop {
    fn id() -> Pubkey {
        mpl_bubblegum::programs::SPL_NOOP_ID
    }
}

pub struct SplAccountCompression;

impl anchor_lang::Id for SplAccountCompression {
    fn id() -> Pubkey {
        mpl_bubblegum::programs::SPL_ACCOUNT_COMPRESSION_ID
    }
}

// todo (Jimii) use references for the accounts
pub fn transfer_nft<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    mint: Account<'a, Mint>,
    authority: AccountInfo<'a>,
    token_program: Program<'a, Token>,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    let cpi_accounts = TokenTransferChecked {
        from,
        mint: mint.to_account_info(),
        to,
        authority,
    };

    let cpi_program = token_program.to_account_info();
    let cpi_context: CpiContext<'_, '_, '_, '_, TokenTransferChecked<'_>> =
        CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

    token::transfer_checked(cpi_context, 1, 0)?;

    Ok(())
}

// transfer lamports from on person to another
pub fn transfer_lamports<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    system_program: &Program<'a, System>,
    lamports: u64,
) -> Result<()> {
    let cpi_accounts = SolanaTransfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };
    let cpi_program = system_program.to_account_info();

    let cpi_context: CpiContext<'_, '_, '_, '_, SolanaTransfer<'_>> =
        CpiContext::new(cpi_program, cpi_accounts);

    system_program::transfer(cpi_context, lamports)?;

    Ok(())
}
