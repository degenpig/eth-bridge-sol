use anchor_lang::prelude::*;

#[account]
pub struct BaseAccount {
    pub default_admin_role: Pubkey,
    pub owner_role: Pubkey,
    pub exo_role: Pubkey,
    pub exo_address: Pubkey,
    pub gcred_address: Pubkey,
    pub max_reward: u64,
    pub total_reward_amount: u64,
    pub foundation_node: Pubkey,
    pub fn_reward: u64,
    staking_infos: Vec<StakingInfo>,
    pub interest_holder_counter: Vec<u32>,
    pub tier: Vec<Tier>,
    pub tier_candidate: Vec<TierCandiate>,
    pub paused: bool,
    pub bump: u8,
}


#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct StakingInfo {
    pub total_index: u16,
    pub holder: Pubkey,
    pub amount: u64,
    pub start_date: i64,
    pub expire_date: i64,
    pub duration: u8,
    pub claim_day: u8,
    pub interest_rate: u8,
    pub index: u16,
}
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Tier {
    pub address: Pubkey,
    pub value: u8,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct TierCandiate {
    pub address: Pubkey,
    pub value: bool,
}


