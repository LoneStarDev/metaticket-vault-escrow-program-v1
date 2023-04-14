use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;


declare_id!("3NGfXZScUbEa57UNnSU95qM7Pu72DwsvHUPo7ca1rxvc");



#[program]
pub mod metaticket_vault_escrow_program_v1 {
    use super::*;

    pub fn initialize_metaticket_manager(ctx: Context<InitializeMetaTicketManager>) -> Result<()> {
        init_manager_and_mint_authority::initialize_metaticket_manager(ctx)
    }

    pub fn initialize_minting_authority(ctx: Context<InitializeMintAuthority>, id:u64) -> Result<()> {
        init_manager_and_mint_authority::initialize_minting_authority(ctx, id)
    }

    pub fn create_token(
        ctx: Context<CreateToken>,
        nft_title: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<()> {
        create::create_token(ctx, nft_title, nft_symbol, nft_uri)
    }

    // pub fn mint_to(ctx: Context<MintTo>) -> Result<()> {
    //     mint::mint_to(ctx)
    // }


}




