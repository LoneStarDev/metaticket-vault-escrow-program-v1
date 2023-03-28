use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, spl_token::instruction::AuthorityType, Mint, SetAuthority, Token,
    TokenAccount, Transfer,
};

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

    pub fn minting_auth_and_escrow_state(ctx: Context<MetaTicketMintSetup>, id: u64) -> Result<()> {

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
        metaticket_mint_authority.bump = *ctx.bumps.get("mint_auth").unwrap();

                             // Save escrow bump
        let escrow_state = &mut ctx.accounts.escrow_state;
        escrow_state.bump = *ctx.bumps.get("escrow_state").unwrap();


        Ok(())
    }

    pub fn initialize_escrow(ctx: Context<InitializeEscrow,>,id:u64, metaticket_amount_nft_to_send_taker: u64, taker_amount_usdc_to_metaticket: u64) -> Result<()> {

        let escrow_state = &mut ctx.accounts.escrow_state;
        escrow_state.metaticket_authority = *ctx.accounts.metaticket_authority.key;
        escrow_state.metaticket_deposit_token_account = *ctx.accounts.metaticket_deposit_token_account.to_account_info().key;
        escrow_state.metaticket_receive_token_account = *ctx.accounts.metaticket_receive_token_account.to_account_info().key;
        escrow_state.metaticket_amount_nft_to_send_taker = metaticket_amount_nft_to_send_taker;
        escrow_state.taker_amount_usdc_to_metaticket = taker_amount_usdc_to_metaticket;

        
        if metaticket_amount_nft_to_send_taker > 4 {
            return err!(MetaTicketError::TicketLimitReached)
        }
        
        ctx.accounts.metaticket_mint_authority.id = id;
        let id_bytes = id.to_le_bytes();
        let authority_seed = &[b"auth", &id_bytes[..]];


        let (vault_authority, _vault_authority_bump) =
        Pubkey::find_program_address(authority_seed, ctx.program_id);

        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;

        token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            ctx.accounts.escrow_state.metaticket_amount_nft_to_send_taker,
        )?;

        Ok(())
    }


    pub fn exchange(ctx: Context<Exchange>, id:u64) -> Result<()> {

        ctx.accounts.metaticket_mint_authority.id = id;
        let id_bytes = id.to_le_bytes();
        let authority_seed = &[b"auth", &id_bytes[..]];

        let (_vault_authority, vault_authority_bump) =
        Pubkey::find_program_address(authority_seed, ctx.program_id);

        let authority_seeds = &[&authority_seed[..], &[&[vault_authority_bump]]];

        token::transfer(
            ctx.accounts.into_transfer_to_initializer_context(),
            ctx.accounts.escrow_state.taker_amount_usdc_to_metaticket,
        )?;

        token::transfer(
            ctx.accounts
                .into_transfer_to_taker_context()
                .with_signer(&authority_seeds[..]),
            ctx.accounts.escrow_state.metaticket_amount_nft_to_send_taker,
        )?;


        Ok(())
    }

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

#[account]
#[derive(InitSpace)]
pub struct EscrowState {
    pub bump: u8,
    pub metaticket_authority: Pubkey,
    pub metaticket_deposit_token_account: Pubkey,
    pub metaticket_receive_token_account: Pubkey,
    pub metaticket_amount_nft_to_send_taker: u64,
    pub taker_amount_usdc_to_metaticket: u64,
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
        payer = metaticket_authority,
        space =  8 + EscrowState::INIT_SPACE,
        seeds = [b"escrow_state", metaticket_manager.key().as_ref(), &id.to_le_bytes()], 
        bump 
    )]
    pub escrow_state: Box<Account<'info, EscrowState>>,
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
#[instruction(id: u64)]
pub struct InitializeEscrow<'info>{
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
    pub metaticket_mint_authority: Account<'info, TicketMintAuthority>,
/** 
 * * CREATE A VAULT ACCOUNT THAT IS AN ASSOCIATED TOKEN ACCOUNT *
 * * THIS IS WHERE THE ENTIRE COLLECTION WILL BE MINTED TO *
*/
    #[account(
        init,
        seeds = [b"vault".as_ref(), metaticket_mint_authority.key().as_ref(), &id.to_le_bytes()],
        bump,
        payer = metaticket_authority,
        token::mint = mint,
        token::authority = metaticket_authority,
    )]
    pub metaticket_nft_vault: Account<'info, TokenAccount>,
/** 
 * * KEEP RACK OF THE ESCROW STATE WITH A STATE ACCOUNT *
*/
    #[account(
        init,
        seeds = [b"state".as_ref(), &id.to_le_bytes()],
        bump,
        payer = metaticket_authority,
        space = EscrowState::INIT_SPACE
    )]
    pub escrow_state: Account<'info, EscrowState>,
    pub metaticket_deposit_token_account:Box<Account<'info, TokenAccount>>,
    pub metaticket_receive_token_account:Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(id: u64)]

pub struct Exchange<'info> {
    pub taker: Signer<'info>,
    #[account(mut)]
    pub taker_deposit_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub taker_receive_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub metaticket_deposit_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub metaticket_receive_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    pub metaticket_mint_authority: Account<'info, TicketMintAuthority>,
    #[account(
        mut,   constraint = escrow_state.taker_amount_usdc_to_metaticket == taker_deposit_token_account.amount,
        constraint = escrow_state.metaticket_deposit_token_account == *metaticket_deposit_token_account.to_account_info().key,
        constraint = escrow_state.metaticket_receive_token_account == *metaticket_receive_token_account.to_account_info().key,
        constraint = escrow_state.metaticket_authority == *metaticket_authority.key,
    )]
    pub escrow_state: Box<Account<'info, EscrowState>>,
    #[account(
        seeds = [b"vault".as_ref(), metaticket_mint_authority.key().as_ref(), &id.to_le_bytes()],
        bump,
    )]
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
    TicketLimitReached,}


impl<'info> InitializeEscrow<'info> {
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.metaticket_authority.to_account_info(),
            to: self.metaticket_nft_vault.to_account_info(),
            authority: self.metaticket_authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.metaticket_nft_vault.to_account_info(),
            current_authority: self.metaticket_authority.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}

impl<'info> Exchange<'info> {
    fn into_transfer_to_initializer_context(
        &self,
    ) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.taker_deposit_token_account.to_account_info(),
            to: self.metaticket_receive_token_account.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn into_transfer_to_taker_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.metaticket_nft_vault.to_account_info(),
            to: self.taker_receive_token_account.to_account_info(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
 }
