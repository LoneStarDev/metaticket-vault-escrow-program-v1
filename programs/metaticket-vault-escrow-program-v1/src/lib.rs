

use {
    anchor_lang::{
        prelude::*,
        solana_program::program::invoke,
    },
    anchor_spl::token,
    mpl_token_metadata::instruction as mpl_instruction,
};


use anchor_spl::{token::{Token,
    TokenAccount,
}, associated_token::AssociatedToken};
// pub mod constant;

// use crate::constant::NUM_TOKENS_TO_SEND_TO_VAULT;

declare_id!("3NGfXZScUbEa57UNnSU95qM7Pu72DwsvHUPo7ca1rxvc");



#[program]
pub mod metaticket_vault_escrow_program_v1 {
    use super::*;

   
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


    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>,
        token_title: String,
        token_symbol: String,
        token_uri: String,
        _token_decimals: u8,
    ) -> Result<()> {

        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", &ctx.accounts.metadata_account.key());
        invoke(
            &mpl_instruction::create_metadata_accounts_v3(
                ctx.accounts.token_metadata_program.key(),      // Program ID (the Token Metadata Program)
                ctx.accounts.metadata_account.key(),            // Metadata account
                ctx.accounts.mint_account.key(),                // Mint account
                ctx.accounts.metaticket_mint_authority.key(),              // Mint authority
                ctx.accounts.payer.key(),                       // Payer
                ctx.accounts.metaticket_mint_authority.key(),              // Update authority
                token_title,                                    // Name
                token_symbol,                                   // Symbol
                token_uri,                                      // URI
                None,                                           // Creators
                0,                                              // Seller fee basis points
                true,                                           // Update authority is signer
                false,                                          // Is mutable
                None,                                           // Collection
                None,                                           // Uses
                None,                                           // Collection Details
            ),
            &[
                ctx.accounts.metadata_account.to_account_info(),
                ctx.accounts.mint_account.to_account_info(),
                ctx.accounts.metaticket_mint_authority.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.metaticket_mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ]
        )?;
    
        msg!("Token mint created successfully.");

        Ok(())
    }












    // pub fn exchange(ctx: Context<Exchange>) -> Result<()> {

        

    //     Ok(())
    // }

}



// metaticket manager controls the creation of ticket mint authorities in charge of signing the mint to vault
#[account]
#[derive(InitSpace)]
pub struct MetaTicketManager {
    pub id: u64,
    pub bump: u8,
}

// sequentialized mint authority accounts for each ticket collection created by metatciket manager.
#[account]
#[derive(InitSpace)]
pub struct TicketMintAuthority {
    pub id: u64,
    pub bump: u8,
}

// #[account]
// #[derive(InitSpace)]
// pub struct EscrowState {
//     pub bump: u8,
//     pub metaticket_authority: Pubkey,
//     pub metaticket_nft_atas_to_vault: Pubkey,
//     pub metaticket_receive_usdc_account: Pubkey,
//     pub metaticket_amount_nft_to_send_vault: u64,
//     pub taker_amount_usdc_to_metaticket: u64,
// }

#[derive(Accounts)]

pub struct InitializeMetaTicketManager<'info> {
    
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,


    #[account(
        init,
        payer = metaticket_authority,
        space = 8 + MetaTicketManager::INIT_SPACE,
        seeds = [b"manager".as_ref(), metaticket_authority.key().as_ref()
        ],
        bump
    )]

    pub metaticket_manager: Account<'info, MetaTicketManager>,  
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
#[instruction(id: u64)]
pub struct InitializeMintAuthority <'info> {
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        mut,
        seeds = [b"manager".as_ref(), metaticket_authority.key().as_ref()], 
        bump = metaticket_manager.bump
     )]
    pub metaticket_manager: Account<'info, MetaTicketManager>,
    #[account(
        init,
        payer = metaticket_authority,
        space =  8 + TicketMintAuthority::INIT_SPACE,
        seeds = [b"mint_auth".as_ref(), metaticket_manager.key().as_ref(), &id.to_le_bytes()], 
        bump 
    
    )]
    pub metaticket_mint_authority: Account<'info, TicketMintAuthority>,
}




#[derive(Accounts)]
#[instruction(
    token_title: String, 
    token_symbol: String, 
    token_uri: String,
    token_decimals: u8,
)]

pub struct CreateTokenMint<'info>{
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        mint::decimals = token_decimals,
        mint::authority = metaticket_mint_authority.key(),
    )]
    pub mint_account: Account<'info, token::Mint>,
    pub metaticket_mint_authority: Account<'info, TicketMintAuthority>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}











// #[derive(Accounts)]
// #[instruction(id: u64)]
// pub struct InitializeEscrow<'info>{
//     #[account(mut)]
//     pub metaticket_authority: Signer<'info>,
//     pub mint: Account<'info, Mint>,
//     pub metaticket_mint_authority: Account<'info, TicketMintAuthority>,

//     #[account(
//         init,
//         seeds = [b"vault".as_ref(), &id.to_le_bytes()],
//         bump,
//         payer = metaticket_authority,
//         token::mint = mint,
//         token::authority = metaticket_authority,
//     )] 
//     pub metaticket_nft_vault: Account<'info, TokenAccount>,
//     pub escrow_state: Account<'info, EscrowState>,
//     pub metaticket_nft_atas_to_vault:Box<Account<'info, TokenAccount>>,
//     #[account(mut)]
//     pub metaticket_receive_usdc_account: Box<Account<'info, TokenAccount>>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub system_program: Program<'info, System>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub token_program: Program<'info, Token>,
// }

#[derive(Accounts)]
#[instruction(id: u64)]

pub struct Exchange<'info> {
    pub taker: Signer<'info>,
    #[account(mut)]
    pub taker_deposit_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub taker_receive_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub metaticket_nft_atas_to_vault:Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub metaticket_receive_usdc_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    pub metaticket_mint_authority: Account<'info, TicketMintAuthority>,
 
    pub metaticket_nft_vault: Account<'info, TokenAccount>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
     /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}






 #[error_code]
 pub enum MetaTicketError {
     #[msg("Invalid series ID")]
     InvalidSeriesId,
 
     #[msg("Ticket Purchase Limit Has Been Reached")]
     TicketLimitReached,
 }
 