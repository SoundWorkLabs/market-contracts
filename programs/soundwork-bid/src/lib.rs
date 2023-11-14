use anchor_lang::prelude::*;

declare_id!("5uPHCfQ6wJWf5efi3W6HK9a6mHeiF33kS6QkdhpEbN81");

#[program]
pub mod soundwork_bid {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    dummy: AccountInfo<'info>
}
