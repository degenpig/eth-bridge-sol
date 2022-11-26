use anchor_lang::prelude::*;
use exo_token::cpi as exocpl;
use gcred_token::cpi as gcredcpl;

use exo_token::cpi::accounts::{ ProxyMintTo as ProxyMintToExo, ProxyBurn as ProxyBurnExo};
use gcred_token::cpi::accounts::{ ProxyMintTo as ProxyMintToGCRED, ProxyBurn as ProxyBurnGCRED};

use exo_token::program::ExoToken;
use gcred_token::program::GcredToken;
declare_id!("WsJjwkmq88guAz4yWckCkTiKzJPA51D2a8vHoLsFNft");

#[program]
pub mod bridge {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        let exo_mint = &ctx.accounts.exo_mint;
        let gcred_mint = &ctx.accounts.gcred_mint;

        base_account.owner_role = *user.to_account_info().key;
        base_account.exo_mint = *exo_mint.to_account_info().key;
        base_account.gcred_mint = *gcred_mint.to_account_info().key;
        base_account.paused = false;

        Ok(())
    }

    pub fn proxy_bridge_mint(ctx: Context<ProxyBridgeMintTo>, amount: String, target:String) -> Result<()> {
        let base_account = &ctx.accounts.base_account;
        let authority = &ctx.accounts.authority;
        let exo_mint = base_account.exo_mint;
        let gcred_mint = base_account.gcred_mint;
        let mint_address = &ctx.accounts.mint;

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.owner_role != *authority.to_account_info().key {
            return Err(ErrorCode::NotOwnerRole.into());
        }

        if *mint_address.to_account_info().key == exo_mint {
            
            let cpi_program = ctx.accounts.exo_token_program.to_account_info();
            let cpi_accounts = ProxyMintToExo {
                authority: ctx.accounts.authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                base_account: ctx.accounts.exo_token_program_base_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            emit!(Transfer {
                from: *ctx.accounts.authority.to_account_info().key,
                to: *ctx.accounts.to.to_account_info().key,
                target: target,
                amount: amount.parse().unwrap(),
                step: Step::Mint,

            });
            return exocpl::proxy_bridge_mint(cpi_ctx,amount);
        }

        if *mint_address.to_account_info().key == gcred_mint {
            
            let cpi_program = ctx.accounts.gcred_token_program.to_account_info();
            let cpi_accounts = ProxyMintToGCRED {
                authority: ctx.accounts.authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                base_account: ctx.accounts.gcred_token_program_base_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            emit!(Transfer {
                from: *ctx.accounts.authority.to_account_info().key,
                to: *ctx.accounts.to.to_account_info().key,
                target: target,
                amount: amount.parse().unwrap(),
                step: Step::Mint,
            });
            return gcredcpl::proxy_bridge_mint(cpi_ctx,amount);
        }

        return Err(ErrorCode::NotCorrectTokenAddress.into());
    }
    
    pub fn proxy_bridge_burn(ctx: Context<ProxyBridgeBurn>, amount: String,target:String) -> Result<()> {
        let base_account = &ctx.accounts.base_account;
        let authority = &ctx.accounts.authority;
        let exo_mint = base_account.exo_mint;
        let gcred_mint = base_account.gcred_mint;
        let mint = &ctx.accounts.mint;

        if base_account.paused {
            return Err(ErrorCode::ContractPaused.into());
        }

        if base_account.owner_role != *authority.to_account_info().key {
            return Err(ErrorCode::NotOwnerRole.into());
        }
        
        if *mint.to_account_info().key == exo_mint {
            
            let cpi_program = ctx.accounts.exo_token_program.to_account_info();
            let cpi_accounts = ProxyBurnExo {
                authority: ctx.accounts.authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                base_account: ctx.accounts.exo_token_program_base_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            emit!(Transfer {
                from: *ctx.accounts.authority.to_account_info().key,
                to: *ctx.accounts.to.to_account_info().key,
                target: target,
                amount: amount.parse().unwrap(),
                step: Step::Burn,
            });
            return exocpl::proxy_bridge_burn(cpi_ctx,amount);
        }

        if *mint.to_account_info().key == gcred_mint {
            
            let cpi_program = ctx.accounts.gcred_token_program.to_account_info();
            let cpi_accounts = ProxyBurnGCRED {
                authority: ctx.accounts.authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.to.to_account_info(),
                base_account: ctx.accounts.gcred_token_program_base_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            emit!(Transfer {
                from: *ctx.accounts.authority.to_account_info().key,
                to: *ctx.accounts.to.to_account_info().key,
                target: target,
                amount: amount.parse().unwrap(),
                step: Step::Burn,
            });
            return gcredcpl::proxy_bridge_burn(cpi_ctx,amount);
        }

        return Err(ErrorCode::NotCorrectTokenAddress.into());
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
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 250,
    )]
    pub base_account: Account<'info,BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub exo_mint: AccountInfo<'info>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(mut)]
    pub gcred_mint: AccountInfo<'info>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program: Program <'info,System>,
}


#[derive(Accounts)]
pub struct ProxyBridgeMintTo<'info> {
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
    pub exo_token_program: Program<'info, ExoToken>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub exo_token_program_base_account: AccountInfo<'info>,
    pub gcred_token_program: Program<'info, GcredToken>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub gcred_token_program_base_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct ProxyBridgeBurn<'info> {
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
    pub exo_token_program: Program<'info, ExoToken>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub exo_token_program_base_account: AccountInfo<'info>,

    pub gcred_token_program: Program<'info, GcredToken>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    pub gcred_token_program_base_account: AccountInfo<'info>,
}


#[derive(Accounts)]
pub struct UpdatePauseRole<'info> {
    #[account(mut)]
    pub base_account: Account<'info,BaseAccount>,
    /// CHECK:` doc comment explaining why no checks through types are necessary.
    #[account(signer)]
    pub user: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum Step {
    Burn,
    Mint,
}

#[event]
pub struct Transfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub target: String,
    pub amount: u64,
    pub step: Step,
}

#[account]
pub struct BaseAccount {
    pub owner_role : Pubkey,
    pub exo_mint: Pubkey,
    pub gcred_mint: Pubkey,
    pub paused: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You must be the owner!")]
    NotOwnerRole,
    #[msg("The contract is paused now!")]
    ContractPaused,
    #[msg("This is not correct token address!")]
    NotCorrectTokenAddress,
}