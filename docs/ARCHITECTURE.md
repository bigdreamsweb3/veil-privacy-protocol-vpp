# Veil Privacy Protocol Architecture

**Version:** v1 confidential-transfer foundation  
**Status:** Buildable foundation; pending full audit and deployment-specific IDL adapters

Veil Privacy Protocol (VPP) is a non-custodial confidential stablecoin infrastructure layer on Solana. It converts verified stablecoin mints into Token-2022 vAssets and keeps a strict 1:1 reserve accounting invariant.

## High-Level System

```text
Users / Wallets / Apps
        |
        v
+-----------------------------+
|          Veil SDK           |
| - mint-address asset lookup |
| - wrap / unwrap calls       |
| - confidential transfer UX  |
+-----------------------------+
        |
        v
+----------------------------------+
|       Veil Vault Program         |
| - governance asset registry      |
| - verified mint configuration    |
| - reserve vault accounting       |
| - Token/Token-2022 CPIs          |
| - commitment events              |
+----------------------------------+
        |                    |
        v                    v
Underlying Reserve      Token-2022 vAsset Mint
USDC/USDT Vaults        vUSDC/vUSDT Accounts
```

## Core Components

### Asset Registry

Supported assets are identified by canonical underlying mint address, never by symbol. The asset PDA is derived from:

```text
["asset", underlying_mint]
```

Each `AssetConfig` stores:

- canonical underlying mint;
- Token-2022 vAsset mint;
- reserve vault;
- total locked underlying;
- total vAsset minted;
- verification status.

### Vault Authority PDA

The vault authority PDA is derived from:

```text
["vault-authority", underlying_mint]
```

It is expected to control the reserve vault and vAsset mint authority. This prevents a normal admin wallet from minting unbacked vAssets or withdrawing reserves directly.

### Wrap Flow

```text
1. User selects a verified underlying mint address.
2. SDK derives/fetches AssetConfig and reserve accounts.
3. User sends underlying stablecoin into the reserve vault.
4. Veil Vault mints the exact same amount of Token-2022 vAsset.
5. Program updates total_locked and total_vAsset_minted.
6. Program checks total_locked == total_vAsset_minted.
7. Wrapped event is emitted.
```

### Unwrap Flow

```text
1. User burns vAsset from their Token-2022 account.
2. Veil Vault releases the same amount of underlying stablecoin.
3. Program decreases total_locked and total_vAsset_minted.
4. Program checks total_locked == total_vAsset_minted.
5. Unwrapped event is emitted.
```

### Confidential Transfer Flow

```text
1. User configures Token-2022 confidential transfer accounts.
2. Wallet/SDK creates the confidential transfer proof data.
3. Token-2022 processes the encrypted transfer amount.
4. Veil records a 32-byte transfer commitment for indexing/auditability.
5. Public observers can see protocol activity, but not confidential amounts.
```

Veil v1 relies on Token-2022 confidential transfer primitives for amount privacy. The Veil program enforces registry/accounting rules and emits commitment events; Token-2022 verifies confidential-transfer proof payloads.

## 1:1 Backing Invariant

The core invariant is:

```text
total_locked == total_vAsset_minted
```

The program checks this invariant after every wrap and unwrap. Any mismatch fails the transaction.

## TrustLink Pay Integration Direction

```text
Sender chooses protected payment
        |
        v
TrustLink Pay passes canonical mint address to Veil SDK
        |
        v
SDK wraps if needed and routes vAsset payment
        |
        v
Recipient keeps protected vAsset or unwraps to underlying stablecoin
```

TrustLink Pay and other apps must verify official assets by mint address and registry status before displaying a protected token as official.
