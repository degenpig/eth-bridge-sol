use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};

pub mod state;

const ACCOUNT_PREFIX: &[u8] = b"stake_account"; 

use exo_token::cpi as exocpl;
use gcred_token::cpi as gcredcpl;
use exo_token::cpi::accounts::{ ProxyTransfer as ProxyTransferExo };
use gcred_token::cpi::accounts::{ ProxyTransfer as ProxyTransferGCRED };

declare_id!("BAeLrCEWmMtB4yMddmcryMGuHMuTfRVuftmJkDspXRbV");

use state::*;

#[program]
pub mod staking_reward {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,exo_address: Pubkey, gcred_address: Pubkey,bump: u8) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        base_account.exo_address = exo_address;
        base_account.gcred_address = gcred_address;

        base_account.default_admin_role = *user.to_account_info().key;
        base_account.owner_role = *user.to_account_info().key;
        base_account.exo_role = exo_address;
        base_account.bump = bump;

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: String, duration:u8) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let holder = &mut ctx.accounts.holder;
        let my_token_account = &mut ctx.accounts.my_token_account;
        let foundation_node_token_account = &mut ctx.accounts.foundation_node_token_account;
        let foundation_node = &mut ctx.accounts.foundation_node;

        let clock = &ctx.accounts.clock;

        let amount : u64= amount.parse().unwrap();

        if base_account.paused {
            return Err(ErrorCode::ErrorForPause.into());
        }

        if amount > my_token_account.amount {
            return Err(ErrorCode::NotEnoughExoToken.into());
        }

        if duration >= 4 {
            return Err(ErrorCode::DurationNotMatch.into());
        }

        let tier_holder: u8 = 0;

        for tier in base_account.tier.iter() {
            if *holder.to_account_info.key == tier.address {
                tier_holder = tier.value;
            }
        }
        
        let interest_rate: u8 = tier_holder * 4 + duration;

        if *holder.to_account_info.key == foundation_node.to_account_info.key() {
            base_account.fn_reward = foundation_node_token_account.amount * 75 / 1000 /365;
        } else {
            let min_amount = getTierAmount();
            let period = getStakingPeriod();

            let index = 1;
            for staking_info in base_account.staking_infos.iter_mut() {
                if *holder.to_account_info.key  == staking_info.holder {
                    index = stake_info.index + 1;
                }
            }

            let new_staking_info = StakingInfo {
                holder: *holder.to_account_info.key,
                amount,
                start_date: clock.unix_timestamp,
                expire_date: clock.unix_timestamp + period[duration],
                duration,
                0,
                interest_rate,
                index
            };
            base_account.staking_infos.push(new_staking_info);

            base_account.interest_holder_counter[interest_rate] += 1;

            if tier_holder < 3 && amount > min_amount[tier_holder + 1] && duration > tier_holder {

                let tier_candidate_flag:bool = false;
                for tier_candidate in base_account.tier_candidate.tier.iter_mut() {
                    if *holder.to_account_info.key == tier_candidate.address {
                        tier_candidate.value = true;
                    }
                }
                if !tier_candidate {
                    base_account.tier_candidate.push({
                        address: *holder.to_account_info.key,
                        value: false
                    });
                }
            }
        }

        let cpi_program = ctx.accounts.exo_token_program.to_account_info();

        let cpi_accounts = ProxyTransferExo {
            authority: ctx.accounts.holder.to_account_info(),
            from: ctx.accounts.holder.to_account_info(),
            to: ctx.accounts.base_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            base_account: ctx.accounts.exo_token_program_base_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        exocpl::proxy_trasnfer(cpi_ctx,amount).expect("transfer has just been failed");

        emit!(Stake{
            *holder.to_account_info.key,
            amount,
            interest_rate
        })

        Ok(())
    }

    pub fn unstake(ctx: Context<UnStake>, staking_index: u16) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let holder = &mut ctx.accounts.holder;
        let clock =  &ctx.accounts.clock;

        if base_account.paused {
            return Err(ErrorCode::ErrorForPause.into());
        }

        let mut target_staking = null;
        let mut last_staking = null;
        let mut tier_holder = null;
        let mut tier_candidate = null;

        let max_length: u16 = 0;
        for staking_info in base_account.staking_infos.iter_mut() {
            if *holder.to_account_info.key  == staking_info.holder {
                max_length = stake_info.index;
                last_staking = stake_info;

                if stake_info.index == staking_index {
                    target_staking = stake_info;
                }
            }
        }

        if max_length <= staking_index {
            return Err(ErrorCode::InvalidStakingIndex.into());
        }

        if clock.unix_timestamp < target_staking.expire_date && target_staking.interest_rate % 4 != 0 {
            return Err(ErrorCode::UnStakingFailed.into());
        }

        //call the claim function

        for temp_tier_candidate in base_account.tier_candidate.iter() {
            if *holder.to_account_info.key == tier_candidate.address {
                tier_candidate = temp_tier_candidate;
            }
        }

        for temp_tier in base_account.tier.iter() {
            if *holder.to_account_info.key == tier.address {
                tier = temp_tier;
            }
        }

        if target_staking.duration >= tier.value && tier_candidate.value {
            if tier.value < 3{
                tier.value += 1;
            }
            tier_candidate.value = false;
        }

        base_account.interest_holder_counter[target_staking.interest_rate] -= 1;

        target_staking.index = max_length - 1;
        last_staking.index = 0;
        base_account.staking_infos.retain(|&x| x.index != 0);

        let cpi_program = ctx.accounts.exo_token_program.to_account_info();

        let base_account_seeds = &[
            &id().to_bytes(),
            ACCOUNT_PREFIX,
            &[base_account.bump],
        ];

        let base_account_signer = &[&base_account_seeds[..]];

        let cpi_accounts = ProxyTransferExo {
            authority: ctx.accounts.base_account.to_account_info(),
            from: ctx.accounts.base_account.to_account_info(),
            to: ctx.accounts.holder.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            base_account: ctx.accounts.exo_token_program_base_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts,base_account_signer);

        exocpl::proxy_trasnfer(cpi_ctx,target_staking.amount).expect("transfer has just been failed");

        emit!(UnStake{
            holder,
            amount: target_staking.amount,
            interest_rate: target_staking.interest_rate
        })
        Ok(())

    }

    pub fn pause(ctx: Context<UpdatePauseRoleOrAddress>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }

        base_account.paused = true;
        
        Ok(())
    }
    
    pub fn unpause(ctx: Context<UpdatePauseRoleOrAddress>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }

        base_account.paused = false;
        
        Ok(())
    }

    pub fn set_gcred_address(ctx: Context<UpdatePauseRoleOrAddress>, gcred_address: Pubkey) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }

        base_account.gcred_address = gcred_address;

        emit!(GCREDAddressUpdated {
            gcred_address
        });
        
        Ok(())
    }

    pub fn set_fn_address(ctx: Context<UpdatePauseRoleOrAddress>, foundation_node: Pubkey) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }

        base_account.foundation_node = foundation_node;

        emit!(FoundationNodeUpdated {
            foundation_node
        });
        
        Ok(())
    }

    pub fn set_exo_address(ctx: Context<UpdatePauseRoleOrAddress>, exo_address: Pubkey) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }

        base_account.exo_address = exo_address;

        emit!(ExoAddressUpdated {
            exo_address
        });
        
        Ok(())
    }

    
    
}

pub fn getTierAmount() -> Vec<u64> {
    return [0,
        2000000000000000,
        4000000000000000,
        8000000000000000
    ]
}

pub fn getStakingPeriod() -> Vec<i64> {
    return [
        0,
        30 * 3600 * 24,
        60 * 3600 * 24,
        90 * 3600 * 24
    ]
}

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 10240,
        seeds = [&id().to_bytes(), ACCOUNT_PREFIX],
        bump
    )]
    pub base_account: Account<'info,BaseAccount>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program: Program <'info,System>,
}


#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(signer)]
    pub holder: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [&id().to_bytes(), ACCOUNT_PREFIX],
        bump = base_account.bump,
    )]
    pub base_account: Account<'info, BaseAccount>,

    pub exo_token_program: Program<'info, ExoToken>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub exo_token_program_base_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub my_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub foundation_node_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, TokenAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}


#[derive(Accounts)]
pub struct UnStake<'info> {
    #[account(signer)]
    pub holder: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [&id().to_bytes(), ACCOUNT_PREFIX],
        bump = base_account.bump,
    )]
    pub base_account: Account<'info, BaseAccount>,

    #[account(mut)] 
    pub stake_token_account: Account<'info, TokenAccount>,

    pub exo_token_program: Program<'info, ExoToken>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub exo_token_program_base_account: AccountInfo<'info>,
    pub gcred_token_program: Program<'info, GcredToken>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub gcred_token_program_base_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub mint: Account<'info, TokenAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub token_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct UpdatePauseRoleOrAddress<'info> {
    #[account(
        mut,
        seeds = [&id().to_bytes(), ACCOUNT_PREFIX],
        bump = base_account.bump,
    )]
    pub base_account: Account<'info, BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[event]
pub struct Stake {
    pub holder: Pubkey,
    pub amount: u64,
    pub interest_rate: u8,
}

#[event]
pub struct UnStake {
    pub holder: Pubkey,
    pub amount: u64,
    pub interest_rate: u8,
}

#[event]
pub struct GCREDAddressUpdated {
    pub gcred_address: Pubkey,
}

#[event]
pub struct FoundationNodeUpdated {
    pub foundation_node: Pubkey,
}


#[event]
pub struct ExoAddressUpdated {
    pub exo_address: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You must be owner!")]
    NotOwnerAccount,
    #[msg("Not enough EXO token to stake!")]
    NotEnoughExoToken,
    #[msg("Duration does not match")]
    DurationNotMatch,
    #[msg("The contract is paused!")]
    ErrorForPause,
    #[msg("Invalid staking index!")]
    InvalidStakingIndex,
    #[msg("Cannot unstake!")]
    UnStakingFailed
}

