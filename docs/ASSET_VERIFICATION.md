# Asset Verification and vAsset Authenticity

Veil must prevent scammers from creating fake protected stablecoins that look like official vAssets. The protocol does this with a strict asset registry, deterministic addresses, controlled mint authorities, and public metadata conventions.

## Current State

The repository currently contains the first scaffold, not a production deployment. The program defines the intended asset registration and accounting surface. Production vAssets should only be treated as official when they are listed in the Veil asset registry and their mint authority is controlled by the Veil Vault program PDA.

## How Veil Knows a Stablecoin Is Legitimate

Veil does not accept arbitrary tokens from users. A stablecoin becomes supported only after `register_asset` creates an `AssetConfig` account for its canonical mint address.

For each asset, the registry stores:

- the underlying stablecoin mint address, such as the canonical USDC or USDT mint;
- the vAsset mint, such as vUSDC or vUSDT;
- the reserve vault that holds the underlying asset;
- the governance authority that approved the asset;
- a verification status showing whether the asset is official, paused, or deprecated.

Before mainnet support, governance should verify the stablecoin mint address against canonical issuer sources, major Solana token lists, liquidity venues, mint authority status, decimals, and issuer documentation. The protocol must not rely on a symbol like "USDC" because scammers can create tokens with the same name, symbol, and logo.

## Why Scammers Cannot Create Official vAssets

Anyone can create a random token named `vUSDC`, but they cannot make it official unless it matches the on-chain Veil registry.

Official vAssets should satisfy all of these checks:

```text
1. underlying_mint == registered canonical stablecoin mint
2. vasset_mint == registered Veil vAsset mint
3. reserve_vault == registered reserve vault PDA
4. vasset mint authority == Veil Vault PDA
5. registry status == Verified
6. total_locked == total_vasset_minted
```

Wallets, explorers, payment apps, and the Veil SDK should display a vAsset as official only if these checks pass.

## Governance Limits

Governance can add or pause assets, but the protocol design must not give governance a direct withdrawal path from reserve vaults. Reserve movement should only happen through user-authorized wrap and unwrap flows that preserve 1:1 backing.

## User Protection Rules for Integrators

Apps integrating Veil should follow these rules:

- Never trust token name, symbol, or logo by itself.
- Always verify the mint address against `AssetConfig`.
- Warn users when an asset is unregistered, paused, deprecated, or has mismatched metadata.
- Show reserve information so users can confirm the vAsset is backed.
- Prefer SDK helpers for asset lookup by mint address instead of hardcoding unchecked symbols.
