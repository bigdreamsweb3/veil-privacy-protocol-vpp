# Veil Integration Guide

Applications integrate Veil through the TypeScript SDK.

## Basic Flow

```ts
import { PublicKey } from '@solana/web3.js';
import { VeilSDK } from '@veil-protocol/sdk';

const veil = new VeilSDK({ connection, wallet });
const usdcMint = new PublicKey('...canonical USDC mint...');

await veil.wrap(100, usdcMint);
await veil.transferVAsset(recipient, 50, usdcMint);
await veil.unwrap(10, usdcMint);
```

## Recommended UX

- Offer a clear "Protect balance" action for wrapping.
- Explain that vAssets are 1:1 redeemable for the underlying stablecoin.
- Let recipients choose whether they prefer protected vAssets or transparent stablecoins.
- Display privacy status clearly when Token-2022 confidential transfer accounts are configured.

## TrustLink Pay Placeholder

TrustLink Pay can use Veil as a protected settlement layer by wrapping sender funds into vAssets, routing protected payments, and letting recipients keep vAssets or unwrap during payout.
