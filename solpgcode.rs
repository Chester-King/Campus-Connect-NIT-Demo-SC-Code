use anchor_lang::prelude::*;
use anchor_lang::solana_program::{self,system_program, sysvar::rent::Rent,};


// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("J6BPQhc8LdhweJDwqBm74YPBN9d23Tutjwwq5pSQ71RB");

#[program]
pub mod basic_crud {
    use super::*;

    pub fn createdata(ctx: Context<DataContext>, _integer : u64) -> Result<()> {


        let data_account = &mut ctx.accounts.data_account;

        data_account.integerdata = _integer;
        data_account.bump = *ctx.bumps.get("data_account").unwrap();
        
        Ok(())
    }

    pub fn updatedata(ctx: Context<UpdateDataContext>, _integer : u64) -> Result<()> {


        let data_account = &mut ctx.accounts.data_account;

        data_account.integerdata = _integer;
        
        Ok(())
    }

    pub fn closedata(ctx: Context<CloseDataContext>) -> Result<()> {



        
        Ok(())
    }

}

#[derive(Accounts)]
pub struct DataContext<'info>  {
    #[account(
        mut,
    )]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + (4 + 50) + 8 + 32 + 1,
        seeds = [b"data".as_ref()], 
        bump
    )]
    pub data_account: Box<Account<'info, DataAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

#[derive(Accounts)]
pub struct UpdateDataContext<'info>  {
    #[account(
        mut,
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"data".as_ref()], 
        bump=data_account.bump
    )]
    pub data_account: Box<Account<'info, DataAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}

#[derive(Accounts)]
pub struct CloseDataContext<'info>  {
    #[account(
        mut,
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        seeds = [b"data".as_ref()], 
        bump=data_account.bump
    )]
    pub data_account: Box<Account<'info, DataAccount>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

}




#[account]
#[derive(Default)]
pub struct DataAccount {
    integerdata: u64,
    bump: u8
}