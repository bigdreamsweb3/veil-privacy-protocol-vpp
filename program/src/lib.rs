use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

declare_id!("Veil111111111111111111111111111111111111111");

/// Veil Vault Program
///
/// This v1 skeleton establishes the public API and accounting model for a
/// non-custodial stablecoin privacy layer. Full Token-2022 confidential
/// transfer proof verification and mint/burn CPI logic will be implemented
/// after review of these interfaces.
#[program]
pub mod veil_vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, governance: Pubkey) -> Result<()> {
        let registry = &mut ctx.accounts.asset_registry;
        registry.governance = governance;
        registry.asset_count = 0;
        registry.bump = ctx.bumps.asset_registry;
        emit!(VaultInitialized { governance });
        Ok(())
    }

    pub fn register_asset(ctx: Context<RegisterAsset>) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.governance.key(),
            ctx.accounts.asset_registry.governance,
            VeilError::UnauthorizedGovernance
        );

        let asset = &mut ctx.accounts.asset_config;
        asset.underlying_mint = ctx.accounts.underlying_mint.key();
        asset.vasset_mint = ctx.accounts.vasset_mint.key();
        asset.reserve_vault = ctx.accounts.reserve_vault.key();
        asset.total_locked = 0;
        asset.total_vasset_minted = 0;
        asset.status = AssetStatus::Verified;
        asset.bump = ctx.bumps.asset_config;

        ctx.accounts.asset_registry.asset_count = ctx
            .accounts
            .asset_registry
            .asset_count
            .checked_add(1)
            .ok_or(VeilError::ArithmeticOverflow)?;

        emit!(AssetRegistered {
            underlying_mint: asset.underlying_mint,
            vasset_mint: asset.vasset_mint,
            reserve_vault: asset.reserve_vault,
        });
        Ok(())
    }

    pub fn wrap(ctx: Context<Wrap>, amount: u64) -> Result<()> {
        require!(amount > 0, VeilError::InvalidAmount);
        let asset = &mut ctx.accounts.asset_config;
        asset.total_locked = asset
            .total_locked
            .checked_add(amount)
            .ok_or(VeilError::ArithmeticOverflow)?;
        asset.total_vasset_minted = asset
            .total_vasset_minted
            .checked_add(amount)
            .ok_or(VeilError::ArithmeticOverflow)?;
        assert_backing_invariant(asset)?;
        emit!(Wrapped {
            owner: ctx.accounts.owner.key(),
            asset: asset.key(),
            amount,
        });
        Ok(())
    }

    pub fn unwrap(ctx: Context<Unwrap>, amount: u64) -> Result<()> {
        require!(amount > 0, VeilError::InvalidAmount);
        let asset = &mut ctx.accounts.asset_config;
        asset.total_locked = asset
            .total_locked
            .checked_sub(amount)
            .ok_or(VeilError::InsufficientBackedLiquidity)?;
        asset.total_vasset_minted = asset
            .total_vasset_minted
            .checked_sub(amount)
            .ok_or(VeilError::InsufficientBackedLiquidity)?;
        assert_backing_invariant(asset)?;
        emit!(Unwrapped {
            owner: ctx.accounts.owner.key(),
            asset: asset.key(),
            amount,
        });
        Ok(())
    }

    pub fn transfer_vasset(ctx: Context<TransferVAsset>, commitment: [u8; 32]) -> Result<()> {
        emit!(VAssetTransferCommitted {
            sender: ctx.accounts.sender.key(),
            asset: ctx.accounts.asset_config.key(),
            commitment,
        });
        Ok(())
    }
}

fn assert_backing_invariant(asset: &AssetConfig) -> Result<()> {
    require!(
        asset.status == AssetStatus::Verified,
        VeilError::AssetNotVerified
    );
    require!(
        asset.total_locked == asset.total_vasset_minted,
        VeilError::BackingInvariantViolation
    );
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = payer, space = AssetRegistry::LEN, seeds = [b"asset-registry"], bump)]
    pub asset_registry: Account<'info, AssetRegistry>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterAsset<'info> {
    #[account(mut, seeds = [b"asset-registry"], bump = asset_registry.bump)]
    pub asset_registry: Account<'info, AssetRegistry>,
    pub governance: Signer<'info>,
    #[account(
        init,
        payer = governance,
        space = AssetConfig::LEN,
        seeds = [b"asset", underlying_mint.key().as_ref()],
        bump
    )]
    pub asset_config: Account<'info, AssetConfig>,
    pub underlying_mint: InterfaceAccount<'info, Mint>,
    pub vasset_mint: InterfaceAccount<'info, Mint>,
    pub reserve_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_2022_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Wrap<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub asset_config: Account<'info, AssetConfig>,
    #[account(mut)]
    pub user_underlying_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub reserve_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_vasset_account: InterfaceAccount<'info, TokenAccount>,
    pub token_2022_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct Unwrap<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub asset_config: Account<'info, AssetConfig>,
    #[account(mut)]
    pub user_vasset_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub reserve_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_underlying_account: InterfaceAccount<'info, TokenAccount>,
    pub token_2022_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct TransferVAsset<'info> {
    pub sender: Signer<'info>,
    pub asset_config: Account<'info, AssetConfig>,
    #[account(mut)]
    pub sender_vasset_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub recipient_vasset_account: InterfaceAccount<'info, TokenAccount>,
    pub token_2022_program: Interface<'info, TokenInterface>,
}

#[account]
pub struct AssetRegistry {
    pub governance: Pubkey,
    pub asset_count: u32,
    pub bump: u8,
}

impl AssetRegistry {
    pub const LEN: usize = 8 + 32 + 4 + 1;
}

#[account]
pub struct AssetConfig {
    pub underlying_mint: Pubkey,
    pub vasset_mint: Pubkey,
    pub reserve_vault: Pubkey,
    pub total_locked: u64,
    pub total_vasset_minted: u64,
    pub status: AssetStatus,
    pub bump: u8,
}

impl AssetConfig {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 8 + 8 + 1 + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum AssetStatus {
    Unverified,
    Verified,
    Paused,
    Deprecated,
}

#[event]
pub struct VaultInitialized {
    pub governance: Pubkey,
}

#[event]
pub struct AssetRegistered {
    pub underlying_mint: Pubkey,
    pub vasset_mint: Pubkey,
    pub reserve_vault: Pubkey,
}

#[event]
pub struct Wrapped {
    pub owner: Pubkey,
    pub asset: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Unwrapped {
    pub owner: Pubkey,
    pub asset: Pubkey,
    pub amount: u64,
}

#[event]
pub struct VAssetTransferCommitted {
    pub sender: Pubkey,
    pub asset: Pubkey,
    pub commitment: [u8; 32],
}

#[error_code]
pub enum VeilError {
    #[msg("Only the configured governance authority can perform this action.")]
    UnauthorizedGovernance,
    #[msg("Amount must be greater than zero.")]
    InvalidAmount,
    #[msg("The vault backing invariant was violated.")]
    BackingInvariantViolation,
    #[msg("Insufficient backed liquidity.")]
    InsufficientBackedLiquidity,
    #[msg("Arithmetic overflow or underflow.")]
    ArithmeticOverflow,
    #[msg("Asset is not verified for active wrapping or unwrapping.")]
    AssetNotVerified,
}
