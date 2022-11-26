use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, MintTo, SetAuthority, Transfer};

pub mod state;

declare_id!("Hq8c9YaMAFTJpfnAQjJat3aaXpaBF78EdesAgxfTqWpi");

use state::*;

#[program]
pub mod exo_token {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        base_account.default_admin_role = *user.to_account_info().key;
        base_account.pause_role = *user.to_account_info().key;
        base_account.minter_role = *user.to_account_info().key;

        let total_supply  = String::from("10000000000000000000");

        base_account.total_supply = total_supply.parse().unwrap();

        Ok(())
    }
    
    pub fn proxy_mint(ctx: Context<ProxyMintTo>, amount: String) -> Result<()> {
        let base_account = &ctx.accounts.base_account;
        let authority = &ctx.accounts.authority;

        let amount: u64 = amount.parse().unwrap();

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.minter_role != *authority.to_account_info().key {
            return Err(ErrorCode::NotMinterAccount.into());
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

        if base_account.default_admin_role != *user.to_account_info().key {
            return Err(ErrorCode::NotAdminRole.into());
        }

        if update_role_address != id() {
            return Err(ErrorCode::NotZeroAddress.into())
        }

        if update_type == 1 {
            base_account.bridge_role = update_role_address;
        } else {
            base_account.staking_reward = update_role_address;
        }
        
        Ok(())
    }

    pub fn pause(ctx: Context<UpdatePauseRole>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.paused = true;
        
        Ok(())
    }

    pub fn unpause(ctx: Context<UpdatePauseRole>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.paused = false;
        
        Ok(())
    }

    pub fn proxy_transfer (_ctx: Context<ProxyTransfer>, _amount: String) -> Result<()> {
        //implement the staking in the future
        Ok(())
    }
    
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
    pub mint: AccountInfo<'info>,
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
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub update_role_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct UpdatePauseRole<'info> {
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub user: AccountInfo<'info>,
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
    #[msg("You must be the owner!")]
    NotAdminRole,
    #[msg("You must be mint owner!")]
    NotMinterAccount,
    #[msg("Your account must have the staking role!")]
    NotStakingRoleAccount,
    #[msg("Your account must have the bridge role!")]
    NotBridgeRoleAccount,
    #[msg("Failed to burn!")]
    FailBurn,
    #[msg("The contract is paused now!")]
    ContractPaused,
    #[msg("Can not be zero address")]
    NotZeroAddress,
}