use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct TicketMintAuthority {
    pub id: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct MetaTicketManager {
    pub id: u64,
    pub bump: u8,
}

impl TicketMintAuthority {
    pub const SEED_PREFIX: &'static str = "mint_authority";
    pub const SIZE: usize = 8 + 8;
}