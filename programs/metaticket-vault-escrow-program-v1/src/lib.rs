use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token, Transfer, SetAuthority}, associated_token::AssociatedToken};

declare_id!("3NGfXZScUbEa57UNnSU95qM7Pu72DwsvHUPo7ca1rxvc");

#[program]
pub mod metaticket_vault_escrow_program_v1 {
    use super::*;

    pub fn initialize(ctx: Context<InitializeMetaTicketVault>, event_name: String) -> Result<()> {
        let metaticket_nft_vault = &mut ctx.accounts.metaticket_nft_vault;
        metaticket_nft_vault.metaticket_authority = ctx.accounts.metaticket_authority.key();
        metaticket_nft_vault.bump = *ctx.bumps.get("vault").unwrap();

        ctx.accounts.ticket_issue.issue.checked_add(1);


        Ok(())
    }


    pub fn metaticket_minting_to_vault(ctx: Context<MetaTicketMintToVault>) -> Result<()> {

      
        Ok(())
    }

    pub fn metaticket_clawback(ctx: Context<MetaTicketCancelEvent>) -> Result<()> {
      
        Ok(())
    }


    pub fn user_buy_ticket(ctx: Context<UserBuyTickets>) -> Result<()> {
        Ok(())
    }

}


#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub metaticket_authority: Pubkey,
    pub bump: u8,
    pub total_nfts_sent_to_vault: u64,
}


#[account]
#[derive(InitSpace)]
pub struct MintNFT {
    pub bump: u8,
    pub metaticket_as_minter: Pubkey,
    pub mint: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct TicketIssue {
    issue: u64
}





#[derive(Accounts)]
#[instruction(escrow_seed: String)]

pub struct InitializeMetaTicketVault<'info> {
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    #[account(
        init,
        space = 8 + Vault::INIT_SPACE, 
        seeds = [b"vault".as_ref(), metaticket_authority.key().as_ref(), ticket_issue.issue.to_le_bytes()], bump, 
        payer = metaticket_authority,
    )]
    pub metaticket_nft_vault: Account<'info, Vault>,
    #[account(
        init, 
        payer = metaticket_authority,
        seeds = [b"ticket_issue", metaticket_authority.key().as_ref(), ],
        bump,
        token::mint = mint,
        token::authority = metaticket_authority,

    )]
    
    pub ticket_issue: Account<'info, TicketIssue>,  
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct MetaTicketMintToVault {

}


#[derive(Accounts)]

pub struct MetaTicketCancelEvent{

}

#[derive(Accounts)]

pub struct UserBuyTickets {

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