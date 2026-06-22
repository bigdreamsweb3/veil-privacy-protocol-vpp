# Veil vAsset Branding System

Veil vAssets need a visual identity that clearly communicates "this is the protected version of a known stablecoin" without pretending to be the original issuer.

## Design Concept

Each vAsset icon is a composite mark:

```text
+---------------------------------------+
| Veil hood silhouette                  |
| - dark protective outer hood          |
| - V-shaped bottom                     |
| - visible lower-quarter token insert  |
+---------------------------------------+
```

The stablecoin logo appears only as a partial insert inside the hood, usually the lower quarter or lower third. This makes the vAsset recognizable while still making the Veil protection layer visually dominant.

## Rules

- The base hood shape belongs to Veil.
- The visible token insert represents the underlying stablecoin.
- The final mark should be named `vUSDC`, `vUSDT`, and so on.
- Apps must still verify the vAsset on-chain; logos are only a visual aid.
- Token issuer trademarks must be respected. Production assets should use issuer-approved artwork or compliant metadata.

## 1D / Simple Mark

The 1D mark is a minimal hood outline with a V-shaped bottom. It is useful for small UI sizes, favicons, monochrome contexts, and places where the underlying token insert would be too small.

See [`veil-hood.svg`](./assets/branding/veil-hood.svg).

## 2D Protected Token Mark

The 2D mark layers a hood over a token-colored circular insert. The insert can be replaced per underlying stablecoin.

See [`vasset-template.svg`](./assets/branding/vasset-template.svg).

## Metadata Convention

Each official vAsset should publish metadata with:

- `name`: `Veil Protected USDC`
- `symbol`: `vUSDC`
- `description`: clear explanation that this is the Veil-protected representation backed 1:1 by the registered underlying mint;
- `image`: URI to the composite vAsset logo;
- `external_url`: Veil docs or explorer page;
- `attributes`: underlying mint, reserve vault, Veil program ID, verification status.
