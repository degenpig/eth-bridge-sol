use anchor_lang::prelude::*;


#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum AuthorityType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount
}

#[account]
pub struct BaseAccount {
    pub default_admin_role : Pubkey,
    pub owner_role: Pubkey,
    pub bridge_role: Pubkey,
    pub staking_role: Pubkey,
    pub md_account: Pubkey,
    pub dao_account:Pubkey,
    pub staking_reward: Pubkey,
    pub paused: bool,
}