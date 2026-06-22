# Veil Integration Guide

Applications integrate Veil through the TypeScript SDK and an IDL-backed instruction adapter for the deployed Anchor program.

## Basic Flow

```ts
import { PublicKey } from '@solana/web3.js';
import { VeilSDK } from '@veil-protocol/sdk';

const usdcMint = new PublicKey('...canonical USDC mint...');
const veil = new VeilSDK({ connection, wallet, instructions });

await veil.wrap(100_000_000, usdcMint);
await veil.transferVAsset(recipient, 50_000_000, usdcMint, { commitment });
await veil.unwrap(10_000_000, usdcMint);
```

## Integration Requirements

- Resolve assets by canonical mint address.
- Fetch `AssetConfig` before showing a token as official.
- Verify `AssetStatus::Verified` before enabling wrap/unwrap/transfer UX.
- Configure Token-2022 confidential transfer accounts before protected transfers.
- Display reserve and vAsset mint information to users.

## Recommended UX

- Offer a clear "Protect balance" action for wrapping.
- Explain that vAssets are 1:1 redeemable for the underlying stablecoin.
- Let recipients choose whether they prefer protected vAssets or transparent stablecoins.
- Display privacy status clearly when Token-2022 confidential transfer accounts are configured.

## TrustLink Pay Placeholder

TrustLink Pay can use Veil as a protected settlement layer by wrapping sender funds into vAssets, routing protected payments, and letting recipients keep vAssets or unwrap during payout.
