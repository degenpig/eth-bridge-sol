use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AuthorityType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount
}

#[account]
pub struct BaseAccount {
    pub default_admin_role : Pubkey,
    pub pause_role: Pubkey,
    pub minter_role: Pubkey,
    pub bridge_role: Pubkey,
    pub staking_reward: Pubkey,
    pub bridge_account: Pubkey,
    pub paused: bool,
    pub total_supply: u64,
}