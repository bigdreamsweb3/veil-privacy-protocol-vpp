# Veil Privacy Protocol (VPP)

**Private stablecoins on Solana. Real privacy, real money.**

Veil Privacy Protocol is non-custodial confidential stablecoin infrastructure for Solana. It lets users and applications convert transparent stablecoins, identified by canonical mint addresses, into **vAssets**: private, shielded representations such as vUSDC and vUSDT.

A vAsset is always designed to be redeemable 1:1 for its underlying stablecoin while hiding the user's protected balance and transfer amount from public observers.

## Why Veil Exists

Solana stablecoins are fast, liquid, and useful for payments, but standard token accounts expose balances and payment amounts publicly. That transparency can leak payroll details, treasury activity, merchant revenue, user spending patterns, and operational flows.

Veil adds a protocol-level privacy layer so wallets, payment apps, and other Solana applications can offer confidential stablecoin balances without becoming custodians.

## Core Principles

- **Protocol, not company product**: Veil is infrastructure that other applications can integrate.
- **Non-custodial**: Users keep control of their keys and funds.
- **1:1 backed**: Every vAsset must correspond to locked underlying stablecoin reserves.
- **No hidden admin withdrawal path**: Governance can register assets and evolve the protocol, but cannot steal vault reserves.
- **Audit-first design**: Vault accounting and reserve invariants are explicit and verifiable.
- **Token-2022 ready**: vAssets are designed around Token-2022 and its confidential transfer extension path.

## What Veil Enables

| Action | Description |
| --- | --- |
| Wrap | Deposit normal stablecoin into a vault and receive the matching vAsset. |
| Hold | Maintain a protected balance whose amount is not publicly exposed. |
| Transfer | Send vAssets using confidential transfer primitives and/or commitments. |
| Unwrap | Burn vAssets and redeem the underlying stablecoin 1:1. |
| Integrate | Add private stablecoin flows to wallets, payment apps, and DeFi interfaces. |

## vAsset Mental Model

| Asset Type | Balance Visible? | Transfer Amount Visible? | Primary Use Case |
| --- | --- | --- | --- |
| USDC / USDT | Yes | Yes | Transparent payments and DeFi. |
| vUSDC / vUSDT | No, when confidential extensions are active | No, when confidential extensions are active | Protected balances and private payments. |

## Repository Structure

```text
.
├── program/                  # Anchor Solana program for the Veil Vault
├── veil-sdk/                 # TypeScript SDK and integration surface
├── docs/                     # Protocol, architecture, security, and integration docs
│   └── discord/              # Copy-paste community messages
├── examples/                 # Future reference examples
├── integrations/             # Integration-specific notes and adapters
│   └── trustlink-pay/        # Placeholder for TrustLink Pay integration
├── governance/               # Future Veil Improvement Proposal materials
├── Cargo.toml                # Rust workspace configuration
├── Anchor.toml               # Anchor workspace configuration
└── package.json              # JavaScript workspace configuration
```

## SDK Preview

```ts
import { PublicKey } from '@solana/web3.js';
import { VeilSDK } from '@veil-protocol/sdk';

const veil = new VeilSDK({ connection, wallet });

const usdcMint = new PublicKey('...canonical USDC mint...');

await veil.wrap(100, usdcMint);
await veil.transferVAsset(recipient, 25, usdcMint);
await veil.unwrap(10, usdcMint);
```

## Documentation

- [Architecture](./ARCHITECTURE.md)
- [Security](./SECURITY.md)
- [Integration](./INTEGRATION.md)
- [Asset Verification](./ASSET_VERIFICATION.md)
- [Branding](./BRANDING.md)
- [Discord Introduction](./discord/intro.md)

## Creator

Created by **Agbaka Daniel Ugonna (Big Dreams Web3)**.

## Status

Veil is in early protocol design and implementation. The current codebase establishes the repository layout, documentation, Anchor instruction surface, and SDK interfaces before expanding into full production implementation.
