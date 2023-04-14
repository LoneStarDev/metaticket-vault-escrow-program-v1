
use {
    anchor_lang::{prelude::*, solana_program::program::invoke_signed},
    anchor_spl::{associated_token, token},
    mpl_token_metadata::instruction as mpl_instruction,
    // spl_token::instruction::AuthorityType,
};

use crate::state::TicketMintAuthority;

pub fn mint_to(ctx: Context<MintTo>) -> Result<()> {
    let bump = ctx.accounts.metaticket_mint_authority.bump;
    // Mint the NFT to the user's wallet
    //
    msg!("Minting NFT to associated token account...");
    msg!(
        "Mint: {}",
        &ctx.accounts.mint_account.to_account_info().key()
    );
    msg!(
        "Token Address: {}",
        &ctx.accounts.associated_token_account.key()
    );
    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.mint_account.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(),
                authority: ctx.accounts.metaticket_mint_authority.to_account_info(),
            },
            &[&[TicketMintAuthority::SEED_PREFIX.as_bytes(), &[bump]]],
        ),
        1,
    )?;

    // We can make this a Limited Edition NFT through Metaplex,
    //      which will disable minting by setting the Mint & Freeze Authorities to the
    //      Edition Account.
    //
    msg!("Creating edition account...");
    msg!(
        "Edition account address: {}",
        ctx.accounts.edition_account.key()
    );
    invoke_signed(
        &mpl_instruction::create_master_edition_v3(
            ctx.accounts.token_metadata_program.key(), // Program ID
            ctx.accounts.edition_account.key(),        // Edition
            ctx.accounts.mint_account.key(),           // Mint
            ctx.accounts.metaticket_mint_authority.key(),         // Update Authority
            ctx.accounts.metaticket_mint_authority.key(),         // Mint Authority
            ctx.accounts.metadata_account.key(),       // Metadata
            ctx.accounts.metaticket_authority.key(),                  // Payer
            Some(1),                                   // Max Supply
        ),
        &[
            ctx.accounts.edition_account.to_account_info(),
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.mint_account.to_account_info(),
            ctx.accounts.metaticket_mint_authority.to_account_info(),
            ctx.accounts.metaticket_authority.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ],
        &[&[TicketMintAuthority::SEED_PREFIX.as_bytes(), &[bump]]],
    )?;



    msg!("NFT minted successfully.");

    Ok(())
}

#[derive(Accounts)]
pub struct MintTo<'info> {
    /// CHECK: Metaplex will check this
    #[account(mut)]
    pub edition_account: UncheckedAccount<'info>,
    /// CHECK: Metaplex will check this
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,
    #[account(
        mut,
        mint::decimals = 0,
        mint::authority = metaticket_mint_authority.key(),
        mint::freeze_authority = metaticket_mint_authority.key(),
    )]
    pub mint_account: Account<'info, token::Mint>,
    pub vault_authority: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [ TicketMintAuthority::SEED_PREFIX.as_bytes() ],
        bump = metaticket_mint_authority.bump,
    )]
    pub metaticket_mint_authority: Account<'info, TicketMintAuthority>,
    #[account(
        init_if_needed,
        payer = metaticket_authority,
        associated_token::mint = mint_account,
        associated_token::authority = vault_authority,
    )]
    pub associated_token_account: Account<'info, token::TokenAccount>,
    #[account(mut)]
    pub metaticket_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>,
}