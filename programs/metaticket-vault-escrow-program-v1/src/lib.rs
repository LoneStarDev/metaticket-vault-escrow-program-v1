use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token, Transfer, SetAuthority}, associated_token::AssociatedToken, token_interface::Token2022};

declare_id!("3NGfXZScUbEa57UNnSU95qM7Pu72DwsvHUPo7ca1rxvc");

#[program]
pub mod metaticket_vault_escrow_program_v1 {
    use super::*;

    pub fn initialize_ticket_manager(ctx: Context<InitializeMetaTicketManager>) -> Result<()> {

                            // create a new account with a series of 0 //
        let metaticket_manager = &mut ctx.accounts.metaticket_manager;
        metaticket_manager.id = 0;
        metaticket_manager.bump = *ctx.bumps.get("metaticket_manager").unwrap();

        Ok(())
    }

    
    pub fn metaticket_initialize_minting_setup(ctx: Context<MetaTicketMintSetup>, id: u64) -> Result<()> {

                            // Increment MetaTicket Manager account
        ctx.accounts.metaticket_manager.id = match ctx.accounts.metaticket_manager.id.checked_add(1) {
            Some(v) => v,
            None => return err!(MetaTicketError::InvalidSeriesId)
        };

                            // Ensure ID matches new MetaTicket manager account
        require!(ctx.accounts.metaticket_manager.id.eq(&id),MetaTicketError::InvalidSeriesId);

                            // Set mint id and bump
        ctx.accounts.metaticket_mint_authority.id = id;
        let metaticket_mint_authority = &mut ctx.accounts.metaticket_mint_authority;
        metaticket_mint_authority.bump = *ctx.bumps.get("mint_auth").unwrap();

                            // create a new vault associated with the mint authority //
        let metaticket_nft_vault = &mut ctx.accounts.metaticket_nft_vault;
        metaticket_nft_vault.metaticket_authority = ctx.accounts.metaticket_authority.key();
        metaticket_nft_vault.bump = *ctx.bumps.get("vault").unwrap();


        Ok(())
    }

    pub fn metaticket_mint_tickets_to_vault(ctx: Context<MintToVault>) -> Result<()> {
    
        Ok(())
    }


    pub fn user_buy_metaticket(ctx: Context<UserBuyTickets>) -> Result<()> {
        Ok(())
    }

}


#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub metaticket_authority: Pubkey,
    pub bump: u8,
    pub issue: u64,
}


#[account]
#[derive(InitSpace)]
pub struct MetaTicketManager {
    pub id: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct TicketMintAuthority {
    pub id: u64,
    pub bump: u8,
}




#[derive(Accounts)]
#[instruction(id: u64)]

pub struct InitializeMetaTicketManager<'info> {
    
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,


    #[account(
        init,
        payer = metaticket_authority,
        space = 8 + MetaTicketManager::INIT_SPACE,
        seeds = [b"manager", metaticket_authority.key().as_ref()
        ],
        bump
    )]

    pub metaticket_manager: Account<'info, MetaTicketManager>,  
    pub system_program: Program<'info, System>
}





#[derive(Accounts)]
#[instruction(id: u64)]
pub struct MetaTicketMintSetup <'info> {
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"manager", metaticket_authority.key().as_ref()], 
        bump = metaticket_manager.bump
    
    )]
    pub metaticket_manager: Account<'info, MetaTicketManager>,
    #[account(
        init,
        payer = metaticket_authority,
        space =  8 + TicketMintAuthority::INIT_SPACE,
        seeds = [b"mint_auth", metaticket_manager.key().as_ref(), &id.to_le_bytes()], 
        bump 
    
    )]
    pub metaticket_mint_authority: Account<'info, TicketMintAuthority>,
    #[account(
        init,
        space = 8 + Vault::INIT_SPACE, 
        seeds = [b"vault".as_ref(), metaticket_mint_authority.key().as_ref(), id.to_le_bytes().as_ref()], bump, 
        payer = metaticket_authority,
    )]
    pub metaticket_nft_vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>
}




#[derive(Accounts)]

pub struct MintToVault{
   
}






#[derive(Accounts)]

pub struct UserBuyTickets {

}





#[error_code]
pub enum MetaTicketError {
    #[msg("Invalid series ID")]
    InvalidSeriesId,
}








// impl<'info> InitializeMetaTicketVault<'info> {
//     fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
//         let cpi_accounts = Transfer {
//             from: self.metaticket_authority.to_account_info(),
//             to: self.metaticket_nft_vault.to_account_info(),
//             authority: self.metaticket_authority.to_account_info(),
//         };
//         CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
//     }

//     fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
//         let cpi_accounts = SetAuthority {
//             account_or_mint: self.vault.to_account_info(),
//             current_authority: self.initializer.to_account_info(),
//         };
//         CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
//     }
// }