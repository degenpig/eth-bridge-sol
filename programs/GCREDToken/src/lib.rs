use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer};

pub mod state;

declare_id!("8TtckPi4UiCGeCAiErSHzSE9afMuURWuaLEM9T5hovxK");

use state::*;

#[program]
pub mod gcred_token {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>,md_account: Pubkey,dao_account: Pubkey) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        base_account.md_account = md_account;
        base_account.dao_account = dao_account;
        base_account.default_admin_role = *user.to_account_info().key;
        base_account.owner_role = *user.to_account_info().key;
        base_account.paused = true;

        Ok(())
    }
    pub fn proxy_buy_item(ctx: Context<ProxyBuyItem>, amount: String) -> Result<()> {
        let base_account = &ctx.accounts.base_account;
        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        let amount: u64 = amount.parse().unwrap();
        let burn_amount = 70 * amount / 100;
        let md_amount = 25 * amount / 100;
        let dao_amount = 5 * amount / 100;
        // transfer from owner to md_account
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.clone(),
            to: ctx.accounts.md_account.clone(),
            authority: ctx.accounts.authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let result = token::transfer(CpiContext::new(cpi_program, cpi_accounts), md_amount);

        match result {
            Ok(_n)  => println!("Success to transfer from owner to md_account"),
            Err(_e) => return Err(ErrorCode::FailTransferToMDAddress.into()),
        }

        // transfer from owner to dao_account
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.clone(),
            to: ctx.accounts.dao_account.clone(),
            authority: ctx.accounts.authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let result = token::transfer(CpiContext::new(cpi_program, cpi_accounts), dao_amount);

        match result {
            Ok(_n)  => println!("Success to transfer from owner to dao_account"),
            Err(_e) => return Err(ErrorCode::FailTransferToDAOAddress.into()),
        }

        // burn from owner
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.clone(),
            from: ctx.accounts.from.clone(),
            authority: ctx.accounts.authority.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let result = token::burn(CpiContext::new(cpi_program, cpi_accounts),burn_amount);

        match result {
            Ok(_n)  => println!("Success to burn!"),
            Err(_e) => return Err(ErrorCode::FailBurn.into()),
        }
        Ok(())

    }
    
    pub fn proxy_mint(ctx: Context<ProxyMintTo>, amount: String) -> Result<()> {
        let base_account = &ctx.accounts.base_account;
        let authority = &ctx.accounts.authority;

        let amount: u64 = amount.parse().unwrap();

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.owner_role != *authority.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }
        token::mint_to(ctx.accounts.into(), amount)
    }

    pub fn proxy_mint_for_reward(ctx: Context<ProxyMintTo>, amount: String) -> Result<()> {
        let base_account = &ctx.accounts.base_account;
        let authority = &ctx.accounts.authority;

        let amount: u64 = amount.parse().unwrap();

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.staking_reward != *authority.to_account_info().key {
            return Err(ErrorCode::NotStakingRoleAccount.into());
        }
        token::mint_to(ctx.accounts.into(), amount)
    }

    pub fn proxy_bridge_mint(ctx: Context<ProxyMintTo>, amount: String) -> Result<()> {
        let base_account = &ctx.accounts.base_account;
        let authority = &ctx.accounts.authority;

        let amount: u64 = amount.parse().unwrap();

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.bridge_role != *authority.to_account_info().key {
            return Err(ErrorCode::NotBridgeRoleAccount.into());
        }
        token::mint_to(ctx.accounts.into(), amount)
    }
    
    pub fn proxy_bridge_burn(ctx: Context<ProxyBurn>, amount: String) -> Result<()> {
        let base_account = &ctx.accounts.base_account;
        let authority = &ctx.accounts.authority;

        let amount: u64 = amount.parse().unwrap();

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.bridge_role != *authority.to_account_info().key {
            return Err(ErrorCode::NotBridgeRoleAccount.into());
        }
        token::burn(ctx.accounts.into(), amount)
    }
    pub fn update_role(ctx: Context<UpdateRole>, update_role_address: Pubkey, update_type: i32) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }

        if update_type == 1 {
            base_account.bridge_role = update_role_address;
        } else {
            base_account.staking_reward = update_role_address;
        }
        emit!(StakingRewardUpdated {
            staking_reward: update_role_address
        });
        
        Ok(())
    }

    pub fn update_md_account(ctx: Context<UpdateAddress>, md_account: Pubkey) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }
        base_account.md_account = md_account;

        emit!(MNAddressUpdated {
            md_address: md_account
        });

        Ok(())
    }

    pub fn update_dao_account(ctx: Context<UpdateAddress>, dao_account: Pubkey) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }
        base_account.dao_account = dao_account;

        emit!(DAOAddressUpdated {
            dao_address: dao_account
        });
        
        Ok(())
    }

    pub fn pause(ctx: Context<UpdatePauseRole>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }

        base_account.paused = true;
        
        Ok(())
    }

    pub fn unpause(ctx: Context<UpdatePauseRole>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;

        if base_account.owner_role != *user.to_account_info().key {
            return Err(ErrorCode::NotOwnerAccount.into());
        }

        base_account.paused = false;
        
        Ok(())
    }

    pub fn proxy_transfer (ctx: Context<ProxyTransfer>, amount: String) -> Result<()> {
        let base_account = &ctx.accounts.base_account;
        let amount: u64 = amount.parse().unwrap();

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.md_account == *ctx.accounts.to.to_account_info().key || base_account.dao_account == *ctx.accounts.to.to_account_info().key {
            let cpi_accounts = Transfer {
                from: ctx.accounts.from.clone(),
                to: ctx.accounts.to.clone(),
                authority: ctx.accounts.authority.clone(),
            };
            let cpi_program = ctx.accounts.token_program.clone();
            return token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount);
        } else {
            let md_amount = 2 * amount / 100;
            let burn_amount = 3 * amount / 100;
            let transfer_amount = amount - md_amount - burn_amount;
            // transfer from onwer to target account
            let cpi_accounts = Transfer {
                from: ctx.accounts.from.clone(),
                to: ctx.accounts.to.clone(),
                authority: ctx.accounts.authority.clone(),
            };
            let cpi_program = ctx.accounts.token_program.clone();
            let result = token::transfer(CpiContext::new(cpi_program, cpi_accounts), transfer_amount);

            match result {
                Ok(_n)  => println!("Success to transfer"),
                Err(_e) => return Err(ErrorCode::FailTransferToTargetAddress.into()),
            }

            // transfer from onwer to md account
            let cpi_accounts = Transfer {
                from: ctx.accounts.from.clone(),
                to: ctx.accounts.md.clone(),
                authority: ctx.accounts.authority.clone(),
            };
            

            let cpi_program = ctx.accounts.token_program.clone();
            let result = token::transfer(CpiContext::new(cpi_program, cpi_accounts), md_amount);

            match result {
                Ok(_n)  => println!("Success to transfer"),
                Err(_e) => return Err(ErrorCode::FailTransferToTargetAddress.into()),
            }

            // burn from owner
            let cpi_accounts = Burn {
                mint: ctx.accounts.mint.clone(),
                from: ctx.accounts.from.clone(),
                authority: ctx.accounts.authority.clone(),
            };
            let cpi_program = ctx.accounts.token_program.clone();
            let result = token::burn(CpiContext::new(cpi_program, cpi_accounts),burn_amount);

            match result {
                Ok(_n)  => println!("Success to burn!"),
                Err(_e) => return Err(ErrorCode::FailBurn.into()),
            }
        };

        Ok(())

    }

    // pub fn token_transfer (from: AccountInfo, to: AccountInfo, authority: AccountInfo,amount: u64) -> Result<()> {
    //     let cpi_accounts = Transfer {
    //         from: from.clone(),
    //         to: to.clone(),
    //         authority: authority.clone(),
    //     };
    //     let cpi_program = ctx.accounts.token_program.clone();
    //     token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)
    // }
    
    pub fn proxy_set_authority(
        ctx: Context<ProxySetAuthority>,
        authority_type: AuthorityType,
        new_authority: Option<Pubkey>,
    ) -> Result<()> {
        token::set_authority(ctx.accounts.into(), authority_type.into(), new_authority)
    }

    // impl <'a, 'b, 'c, 'info> From<&mut ProxyTransfer<'info>> 
    //     for CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>
    //     {
    //         fn from(accounts: &mut ProxyTransfer<'info>) -> CpiContext<'a, 'b, 'c, 'info, Transfer<'info>> {
    //             let cpi_accounts = Transfer {
    //                 from: accounts.from.clone(),
    //                 to: accounts.to.clone(),
    //                 authority: accounts.authority.clone(),
    //             };
    //             let cpi_program = accounts.token_program.clone();
    //             CpiContext::new(cpi_program, cpi_accounts)
    //         }
    //     }

    impl<'a, 'b, 'c, 'info> From<&mut ProxyMintTo<'info>>
        for CpiContext<'a, 'b, 'c, 'info, MintTo<'info>>
    {
        fn from(accounts: &mut ProxyMintTo<'info>) -> CpiContext<'a, 'b, 'c, 'info, MintTo<'info>> {
            let cpi_accounts = MintTo {
                mint: accounts.mint.clone(),
                to: accounts.to.clone(),
                authority: accounts.authority.clone(),
            };
            let cpi_program = accounts.token_program.clone();
            CpiContext::new(cpi_program, cpi_accounts)
        }
    }

    impl<'a, 'b, 'c, 'info> From<&mut ProxyBurn<'info>> for CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
            fn from(accounts: &mut ProxyBurn<'info>) -> CpiContext<'a, 'b, 'c, 'info, Burn<'info>> {
                let cpi_accounts = Burn {
                    mint: accounts.mint.clone(),
                    from: accounts.to.clone(),
                    authority: accounts.authority.clone(),
                };
                let cpi_program = accounts.token_program.clone();
                CpiContext::new(cpi_program, cpi_accounts)
            }
        }
    

    impl<'a, 'b, 'c, 'info> From<&mut ProxySetAuthority<'info>>
        for CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>>
    {
        fn from(
            accounts: &mut ProxySetAuthority<'info>,
        ) -> CpiContext<'a, 'b, 'c, 'info, SetAuthority<'info>> {
            let cpi_accounts = SetAuthority {
                account_or_mint: accounts.account_or_mint.clone(),
                current_authority: accounts.current_authority.clone(),
            };
            let cpi_program = accounts.token_program.clone();
            CpiContext::new(cpi_program, cpi_accounts)
        }
    }
    impl From<AuthorityType> for spl_token::instruction::AuthorityType {
        fn from(authority_ty: AuthorityType) -> spl_token::instruction::AuthorityType {
            match authority_ty {
                AuthorityType::MintTokens => spl_token::instruction::AuthorityType::MintTokens,
                AuthorityType::FreezeAccount => spl_token::instruction::AuthorityType::FreezeAccount,
                AuthorityType::AccountOwner => spl_token::instruction::AuthorityType::AccountOwner,
                AuthorityType::CloseAccount => spl_token::instruction::AuthorityType::CloseAccount,
            }
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 250,
    )]
    pub base_account: Account<'info,BaseAccount>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program: Program <'info,System>,
}

#[derive(Accounts)]
pub struct ProxyBuyItem<'info> {
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub md_account: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub dao_account: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(
        has_one = md_account @ ErrorCode::NotMDAccount,
        has_one = dao_account @ ErrorCode::NotDAOAccount,
    )]
    pub base_account: Account<'info,BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyTransfer<'info> {
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub to: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub md: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyMintTo<'info> {
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub base_account: Account<'info, BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyBurn<'info> {
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub base_account: Account<'info,BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxySetAuthority<'info> {
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub current_authority: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub account_or_mint: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdateRole<'info> {
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePauseRole<'info> {
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[event]
pub struct StakingRewardUpdated {
    pub staking_reward: Pubkey,
}

#[event]
pub struct MNAddressUpdated {
    pub md_address: Pubkey,
}

#[event]
pub struct DAOAddressUpdated {
    pub dao_address: Pubkey,
}

#[derive(Accounts)]
pub struct UpdateAddress<'info> {
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub user: AccountInfo<'info>,
}



#[error_code]
pub enum ErrorCode {
    #[msg("You must be owner!")]
    NotOwnerAccount,
    #[msg("Your account must have the staking role!")]
    NotStakingRoleAccount,
    #[msg("Your account must have the bridge role!")]
    NotBridgeRoleAccount,
    #[msg("Failed to transfer from owner to md_account!")]
    FailTransferToMDAddress,
    #[msg("Failed to transfer from owner to dao_account!")]
    FailTransferToDAOAddress,
    #[msg("Failed to transfer from owner to target_account!")]
    FailTransferToTargetAddress,
    #[msg("Failed to burn!")]
    FailBurn,
    #[msg("The contract is paused now!")]
    ContractPaused,
    #[msg("This is not md account!")]
    NotMDAccount,
    #[msg("This is not dao account")]
    NotDAOAccount,
}