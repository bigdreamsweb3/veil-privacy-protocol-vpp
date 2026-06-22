# Veil Privacy Protocol Architecture

**Version:** v1 design skeleton  
**Status:** Early implementation

Veil Privacy Protocol (VPP) is a non-custodial confidential stablecoin infrastructure layer on Solana. It converts standard stablecoins into vAssets, allowing protected balances and private transfers while enforcing 1:1 reserve backing.

## High-Level System

```text
Users / Apps / Wallets
        |
        v
+------------------+
|     Veil SDK     |
+------------------+
        |
        v
+--------------------------+
|   Veil Vault Program     |
| - asset registry         |
| - reserve accounting     |
| - wrap / unwrap          |
| - transfer commitments   |
+--------------------------+
    |                 |
    v                 v
Reserve Vaults   Token-2022 vAssets
(USDC/USDT)      (vUSDC/vUSDT)
```

## Core Components

### Veil Vault Program

The Anchor program is the protocol's on-chain accounting and policy layer. It owns the asset registry, tracks per-asset reserve state by canonical underlying mint address, and exposes the core instructions:

- `initialize_vault`
- `register_asset`
- `wrap`
- `unwrap`
- `transfer_vasset`

### Asset Registry

The registry records supported stablecoins by canonical token mint address, not by token symbol. Governance can add assets, but registration does not create a path to withdraw user reserves or mint unbacked vAssets.

### Reserve Vaults

Each supported asset has a reserve vault PDA that holds the underlying stablecoin. The main invariant is:

```text
total_locked_underlying == total_vasset_minted
```

This invariant is checked whenever wrapping or unwrapping updates accounting.

### vAssets

A vAsset is the shielded version of a stablecoin, such as vUSDC or vUSDT. vAssets are intended to use Token-2022 mints so Veil can adopt confidential transfers and related extensions as the implementation matures.

## Wrap Flow

```text
1. User approves/sends the canonical underlying stablecoin mint to the reserve vault.
2. Veil Vault verifies the asset configuration.
3. Vault accounting increases total_locked by amount.
4. Protocol mints the matching vAsset for that registered mint address.
5. Backing invariant is checked.
```

## Private Transfer Flow

```text
1. Sender prepares confidential transfer data or a v1 commitment.
2. Sender calls transfer_vasset with recipient accounts and commitment.
3. Token-2022 confidential transfer logic hides amount details.
4. Veil emits a commitment event for auditability and future indexing.
```

In the initial skeleton, `transfer_vasset` records a commitment placeholder. The production implementation should replace this with Token-2022 confidential transfer CPI calls and proof validation.

## Unwrap Flow

```text
1. User burns vAsset.
2. Vault accounting decreases total_vasset_minted by amount.
3. Vault releases the same amount of underlying stablecoin.
4. Backing invariant is checked.
```

## Privacy Model

Veil's privacy roadmap has two layers:

1. **v1 practical privacy:** Token-2022 confidential transfer support plus commitment events for shielded accounting flows.
2. **v2 enhanced privacy:** Stronger note/nullifier and zero-knowledge proof systems for richer sender, receiver, and balance privacy.

The public chain should reveal that protocol interactions occurred, but not expose protected balances or confidential transfer amounts once Token-2022 confidential transfers are fully wired.

## TrustLink Pay Integration Direction

TrustLink Pay can integrate Veil through the SDK:

```text
Sender chooses "protected payment"
        |
        v
SDK wraps USDC -> vUSDC if needed
        |
        v
TrustLink payment flow routes vAsset
        |
        v
Recipient keeps vUSDC or unwraps to USDC
```

This allows privacy-preserving payment experiences while keeping user funds non-custodial and redeemable.
