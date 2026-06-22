# Veil Privacy Protocol (VPP)

**Private stablecoins on Solana. Real privacy, real money.**

Veil Privacy Protocol is non-custodial confidential stablecoin infrastructure for Solana. It lets users and applications convert verified stablecoin mints into Token-2022 **vAssets**: private, shielded representations such as vUSDC and vUSDT.

A vAsset is designed to be redeemable 1:1 for its underlying stablecoin while Token-2022 confidential transfers hide protected transfer amounts.

## Current Status

Veil is a v1 foundation. The Anchor program now models real wrap/unwrap token CPIs, PDA-controlled vault authority, governance-controlled asset registration, verified asset status, and the reserve invariant. The SDK exposes typed methods that operate on canonical mint addresses and delegate transaction construction to an IDL-backed adapter.

This code is not yet audited mainnet software.

## Core Principles

- **Protocol, not company product**: Veil is infrastructure that other applications can integrate.
- **Mint-address identity**: Supported tokens are identified by canonical mint addresses, not symbols.
- **Non-custodial**: Users keep control of their keys and funds.
- **1:1 backed**: Every vAsset must correspond to locked underlying stablecoin reserves.
- **No hidden admin withdrawal path**: Governance can register and pause assets, but cannot directly seize reserves.
- **Token-2022 privacy**: vAssets are built for Token-2022 confidential transfers.

## Quick Start

```bash
npm install
cargo build --workspace
npm --workspace veil-sdk run build
```

## SDK Preview

```ts
import { PublicKey } from '@solana/web3.js';
import { VeilSDK } from '@veil-protocol/sdk';

const usdcMint = new PublicKey('...canonical USDC mint...');
const veil = new VeilSDK({ connection, wallet, instructions });

await veil.wrap(100_000_000, usdcMint);
await veil.transferVAsset(recipient, 25_000_000, usdcMint, { commitment });
await veil.unwrap(10_000_000, usdcMint);
```

Amounts are passed in base units. For USDC-like assets, `100_000_000` means 100 tokens when the mint has 6 decimals.

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

## Documentation

- [Architecture](./ARCHITECTURE.md)
- [Security](./SECURITY.md)
- [Integration](./INTEGRATION.md)
- [Asset Verification](./ASSET_VERIFICATION.md)
- [Branding](./BRANDING.md)
- [Discord Introduction](./discord/intro.md)

## Creator

Created by **Agbaka Daniel Ugonna (Big Dreams Web3)**.
