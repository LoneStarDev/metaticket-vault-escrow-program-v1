use anchor_lang::prelude::*;
use anchor_spl::{token::{TokenAccount, Mint, Token, Transfer, SetAuthority}, associated_token::AssociatedToken, token_interface::Token2022};

declare_id!("3NGfXZScUbEa57UNnSU95qM7Pu72DwsvHUPo7ca1rxvc");

#[program]
pub mod metaticket_vault_escrow_program_v1 {
    use super::*;

    pub fn initialize(ctx: Context<InitializeMetaTicketVault>, event_name: String) -> Result<()> {
        let metaticket_nft_vault = &mut ctx.accounts.metaticket_nft_vault;
        metaticket_nft_vault.metaticket_authority = ctx.accounts.metaticket_authority.key();
        metaticket_nft_vault.bump = *ctx.bumps.get("vault").unwrap();

        let ticket_issue = &mut ctx.accounts.ticket_issue;

        ticket_issue.issue += 1;

        msg!("Initialized new vault. Current id value for vault is: {}!", ticket_issue.issue);


        ctx.accounts.ticket_issue.issue.checked_add(1);

    
        Ok(())
    }


    pub fn metaticket_minting_to_vault(ctx: Context<MetaTicketMintToVault>) -> Result<()> {

      
        Ok(())
    }

    pub fn metaticket_cancel_event(ctx: Context<MetaTicketCancelEvent>) -> Result<()> {
    
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
#[instruction(escrow_seed: String, issue: u64)]

pub struct InitializeMetaTicketVault<'info> {
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    #[account(
        init,
        space = 8 + Vault::INIT_SPACE, 
        seeds = [b"vault".as_ref(), metaticket_authority.key().as_ref(), issue.to_le_bytes().as_ref()], bump, 
        payer = metaticket_authority,
    )]
    pub metaticket_nft_vault: Account<'info, Vault>,
    #[account(mut)]
    pub ticket_issue: Account<'info, TicketIssue>,  
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
#[instruction(issue: u64)]
pub struct MetaTicketMintToVault <'info> {
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    #[account(
        seeds = [b"vault".as_ref(), metaticket_authority.key().as_ref(), issue.to_le_bytes().as_ref()],
        bump = metaticket_nft_vault.bump
    )]
    pub metaticket_nft_vault: Account<'info, Vault>,
    pub ticket_issue: Account<'info, TicketIssue>, 
    #[account(mut)]
    pub metaticket_mint: Account<'info, MintNFT>,
    #[account(
        init,
        payer = metaticket_authority,
        associated_token::mint = metaticket_mint,
        associated_token::authority = metaticket_authority
    )]
    pub nft_vault_assoc_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [
            b"metaticket_mint".as_ref(),
            metaticket_mint.key().as_ref(),
            metaticket_authority.key().as_ref()
        ],
        bump = metaticket_mint.bump,
        close = metaticket_authority
    )]

    pub mint_nft: Account<'info, MintNFT>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>


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