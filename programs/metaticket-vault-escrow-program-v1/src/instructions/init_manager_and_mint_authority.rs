use anchor_lang::prelude::*;

use crate::state::TicketMintAuthority;
use crate::state::MetaTicketManager;



// first we initialize a metatickt manager account 
pub fn initialize_metaticket_manager(ctx: Context<InitializeMetaTicketManager>) -> Result<()> {

    // create a new account with a series of 0 //
let metaticket_manager = &mut ctx.accounts.metaticket_manager;
metaticket_manager.id = 0;
metaticket_manager.bump = *ctx.bumps.get("metaticket_manager").unwrap();
Ok(())
}

// now we increment a metatciket manager id and create a metaticket mint authority account from the manager.

pub fn initialize_minting_authority(ctx: Context<InitializeMintAuthority>, id: u64) -> Result<()> {

    // Increment MetaTicket Manager account
ctx.accounts.metaticket_manager.id = match ctx.accounts.metaticket_manager.id.checked_add(1) {
Some(v) => v,
None => return err!(MetaTicketError::InvalidSeriesId)
};

    // Ensure ID matches new MetaTicket manager account
require!(ctx.accounts.metaticket_manager.id.eq(&id),MetaTicketError::InvalidSeriesId);

    // Save mint id and bump
ctx.accounts.metaticket_mint_authority.id = id;
let metaticket_mint_authority = &mut ctx.accounts.metaticket_mint_authority;
metaticket_mint_authority.bump = *ctx.bumps.get("metaticket_mint_authority").unwrap();

Ok(())
}


#[derive(Accounts)]

pub struct InitializeMetaTicketManager<'info> {
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    #[account(
        init,
        payer = metaticket_authority,
        space = 8 + MetaTicketManager::INIT_SPACE,
        seeds = [b"manager".as_ref(), metaticket_authority.key().as_ref()],
        bump
    )]
    pub metaticket_manager: Account<'info, MetaTicketManager>,  
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
pub struct InitializeMintAuthority <'info> {
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    
    #[account(
        init,
        payer = metaticket_authority,
        space =  8 + TicketMintAuthority::INIT_SPACE,
        seeds = [ TicketMintAuthority::SEED_PREFIX.as_bytes() ],        
        bump 
    )]
    pub metaticket_mint_authority: Account<'info, TicketMintAuthority>,
    pub metaticket_manager: Account<'info, MetaTicketManager>,  

}


#[error_code]
pub enum MetaTicketError {
    #[msg("Invalid series ID")]
    InvalidSeriesId,

    #[msg("Ticket Purchase Limit Has Been Reached")]
    TicketLimitReached,
}
