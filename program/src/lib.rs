use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    burn, mint_to, transfer_checked, Burn, Mint, MintTo, TokenAccount, TokenInterface,
    TransferChecked,
};

declare_id!("Veil111111111111111111111111111111111111111");

/// Veil Vault Program
///
/// v1 focuses on non-custodial, auditable stablecoin wrapping with Token-2022
/// vAssets. Wrap and unwrap execute real token CPIs and enforce that internal
/// accounting remains 1:1. Confidential transfers are performed by Token-2022
/// vAsset accounts; the protocol records transfer commitments for indexing and
/// audit trails while clients submit the confidential-transfer proof payloads.
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
        require_keys_eq!(
            ctx.accounts.reserve_vault.mint,
            ctx.accounts.underlying_mint.key(),
            VeilError::InvalidReserveVault
        );
        require_keys_eq!(
            ctx.accounts.reserve_vault.owner,
            ctx.accounts.vault_authority.key(),
            VeilError::InvalidVaultAuthority
        );
        require_keys_eq!(
            ctx.accounts.vasset_mint.mint_authority.unwrap(),
            ctx.accounts.vault_authority.key(),
            VeilError::InvalidVAssetMintAuthority
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

    pub fn set_asset_status(ctx: Context<SetAssetStatus>, status: AssetStatus) -> Result<()> {
        require_keys_eq!(
            ctx.accounts.governance.key(),
            ctx.accounts.asset_registry.governance,
            VeilError::UnauthorizedGovernance
        );
        ctx.accounts.asset_config.status = status;
        emit!(AssetStatusChanged {
            asset: ctx.accounts.asset_config.key(),
            status,
        });
        Ok(())
    }

    pub fn wrap(ctx: Context<Wrap>, amount: u64) -> Result<()> {
        require!(amount > 0, VeilError::InvalidAmount);
        validate_asset_accounts_for_wrap(&ctx)?;

        transfer_checked(
            CpiContext::new(
                ctx.accounts.underlying_token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.user_underlying_account.to_account_info(),
                    mint: ctx.accounts.underlying_mint.to_account_info(),
                    to: ctx.accounts.reserve_vault.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
            ctx.accounts.underlying_mint.decimals,
        )?;

        let underlying_mint_key = ctx.accounts.underlying_mint.key();
        let authority_seeds: &[&[u8]] = &[
            b"vault-authority",
            underlying_mint_key.as_ref(),
            &[ctx.bumps.vault_authority],
        ];

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_2022_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.vasset_mint.to_account_info(),
                    to: ctx.accounts.user_vasset_account.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
                &[authority_seeds],
            ),
            amount,
        )?;

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
            underlying_mint: asset.underlying_mint,
            vasset_mint: asset.vasset_mint,
            amount,
        });
        Ok(())
    }

    pub fn unwrap(ctx: Context<Unwrap>, amount: u64) -> Result<()> {
        require!(amount > 0, VeilError::InvalidAmount);
        validate_asset_accounts_for_unwrap(&ctx)?;

        burn(
            CpiContext::new(
                ctx.accounts.token_2022_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.vasset_mint.to_account_info(),
                    from: ctx.accounts.user_vasset_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        let underlying_mint_key = ctx.accounts.underlying_mint.key();
        let authority_seeds: &[&[u8]] = &[
            b"vault-authority",
            underlying_mint_key.as_ref(),
            &[ctx.bumps.vault_authority],
        ];

        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.underlying_token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.reserve_vault.to_account_info(),
                    mint: ctx.accounts.underlying_mint.to_account_info(),
                    to: ctx.accounts.user_underlying_account.to_account_info(),
                    authority: ctx.accounts.vault_authority.to_account_info(),
                },
                &[authority_seeds],
            ),
            amount,
            ctx.accounts.underlying_mint.decimals,
        )?;

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
            underlying_mint: asset.underlying_mint,
            vasset_mint: asset.vasset_mint,
            amount,
        });
        Ok(())
    }

    pub fn transfer_vasset(ctx: Context<TransferVAsset>, commitment: [u8; 32]) -> Result<()> {
        require!(
            ctx.accounts.asset_config.status == AssetStatus::Verified,
            VeilError::AssetNotVerified
        );
        require_keys_eq!(
            ctx.accounts.sender_vasset_account.mint,
            ctx.accounts.asset_config.vasset_mint,
            VeilError::InvalidVAssetAccount
        );
        require_keys_eq!(
            ctx.accounts.recipient_vasset_account.mint,
            ctx.accounts.asset_config.vasset_mint,
            VeilError::InvalidVAssetAccount
        );
        emit!(VAssetTransferCommitted {
            sender: ctx.accounts.sender.key(),
            asset: ctx.accounts.asset_config.key(),
            vasset_mint: ctx.accounts.asset_config.vasset_mint,
            commitment,
        });
        Ok(())
    }
}

fn validate_asset_accounts_for_wrap(ctx: &Context<Wrap>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.asset_config.underlying_mint,
        ctx.accounts.underlying_mint.key(),
        VeilError::InvalidUnderlyingMint
    );
    require_keys_eq!(
        ctx.accounts.asset_config.vasset_mint,
        ctx.accounts.vasset_mint.key(),
        VeilError::InvalidVAssetMint
    );
    require_keys_eq!(
        ctx.accounts.asset_config.reserve_vault,
        ctx.accounts.reserve_vault.key(),
        VeilError::InvalidReserveVault
    );
    require_keys_eq!(
        ctx.accounts.user_underlying_account.mint,
        ctx.accounts.underlying_mint.key(),
        VeilError::InvalidUnderlyingAccount
    );
    require_keys_eq!(
        ctx.accounts.user_underlying_account.owner,
        ctx.accounts.owner.key(),
        VeilError::InvalidUnderlyingAccount
    );
    require_keys_eq!(
        ctx.accounts.user_vasset_account.mint,
        ctx.accounts.vasset_mint.key(),
        VeilError::InvalidVAssetAccount
    );
    require_keys_eq!(
        ctx.accounts.user_vasset_account.owner,
        ctx.accounts.owner.key(),
        VeilError::InvalidVAssetAccount
    );
    assert_backing_invariant(&ctx.accounts.asset_config)
}

fn validate_asset_accounts_for_unwrap(ctx: &Context<Unwrap>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.asset_config.underlying_mint,
        ctx.accounts.underlying_mint.key(),
        VeilError::InvalidUnderlyingMint
    );
    require_keys_eq!(
        ctx.accounts.asset_config.vasset_mint,
        ctx.accounts.vasset_mint.key(),
        VeilError::InvalidVAssetMint
    );
    require_keys_eq!(
        ctx.accounts.asset_config.reserve_vault,
        ctx.accounts.reserve_vault.key(),
        VeilError::InvalidReserveVault
    );
    require_keys_eq!(
        ctx.accounts.user_vasset_account.mint,
        ctx.accounts.vasset_mint.key(),
        VeilError::InvalidVAssetAccount
    );
    require_keys_eq!(
        ctx.accounts.user_vasset_account.owner,
        ctx.accounts.owner.key(),
        VeilError::InvalidVAssetAccount
    );
    require_keys_eq!(
        ctx.accounts.user_underlying_account.mint,
        ctx.accounts.underlying_mint.key(),
        VeilError::InvalidUnderlyingAccount
    );
    require_keys_eq!(
        ctx.accounts.user_underlying_account.owner,
        ctx.accounts.owner.key(),
        VeilError::InvalidUnderlyingAccount
    );
    assert_backing_invariant(&ctx.accounts.asset_config)
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
    #[account(mut)]
    pub reserve_vault: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: PDA authority over reserve vault and vAsset mint. Seeds enforce address.
    #[account(seeds = [b"vault-authority", underlying_mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    pub token_2022_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAssetStatus<'info> {
    #[account(seeds = [b"asset-registry"], bump = asset_registry.bump)]
    pub asset_registry: Account<'info, AssetRegistry>,
    pub governance: Signer<'info>,
    #[account(mut, seeds = [b"asset", asset_config.underlying_mint.as_ref()], bump = asset_config.bump)]
    pub asset_config: Account<'info, AssetConfig>,
}

#[derive(Accounts)]
pub struct Wrap<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, seeds = [b"asset", underlying_mint.key().as_ref()], bump = asset_config.bump)]
    pub asset_config: Account<'info, AssetConfig>,
    pub underlying_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub vasset_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub user_underlying_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub reserve_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_vasset_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: PDA authority over reserve vault and vAsset mint. Seeds enforce address.
    #[account(seeds = [b"vault-authority", underlying_mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    pub underlying_token_program: Interface<'info, TokenInterface>,
    pub token_2022_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct Unwrap<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut, seeds = [b"asset", underlying_mint.key().as_ref()], bump = asset_config.bump)]
    pub asset_config: Account<'info, AssetConfig>,
    pub underlying_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub vasset_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub user_vasset_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub reserve_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user_underlying_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: PDA authority over reserve vault and vAsset mint. Seeds enforce address.
    #[account(seeds = [b"vault-authority", underlying_mint.key().as_ref()], bump)]
    pub vault_authority: UncheckedAccount<'info>,
    pub underlying_token_program: Interface<'info, TokenInterface>,
    pub token_2022_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct TransferVAsset<'info> {
    pub sender: Signer<'info>,
    #[account(seeds = [b"asset", asset_config.underlying_mint.as_ref()], bump = asset_config.bump)]
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
pub struct AssetStatusChanged {
    pub asset: Pubkey,
    pub status: AssetStatus,
}

#[event]
pub struct Wrapped {
    pub owner: Pubkey,
    pub asset: Pubkey,
    pub underlying_mint: Pubkey,
    pub vasset_mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct Unwrapped {
    pub owner: Pubkey,
    pub asset: Pubkey,
    pub underlying_mint: Pubkey,
    pub vasset_mint: Pubkey,
    pub amount: u64,
}

#[event]
pub struct VAssetTransferCommitted {
    pub sender: Pubkey,
    pub asset: Pubkey,
    pub vasset_mint: Pubkey,
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
    #[msg("The provided underlying mint does not match the registered asset.")]
    InvalidUnderlyingMint,
    #[msg("The provided vAsset mint does not match the registered asset.")]
    InvalidVAssetMint,
    #[msg("The provided reserve vault does not match the registered asset.")]
    InvalidReserveVault,
    #[msg("The provided user underlying token account is invalid.")]
    InvalidUnderlyingAccount,
    #[msg("The provided vAsset token account is invalid.")]
    InvalidVAssetAccount,
    #[msg("The reserve vault is not owned by the expected PDA authority.")]
    InvalidVaultAuthority,
    #[msg("The vAsset mint authority must be the Veil vault PDA authority.")]
    InvalidVAssetMintAuthority,
}
