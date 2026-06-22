# Veil Security Model

Veil is designed around explicit, auditable security guarantees.

## Core Guarantees

- vAssets must be backed 1:1 by underlying stablecoins.
- Supported assets are identified by canonical mint address, not symbol or logo.
- Users keep custody of their wallets and signing keys.
- Governance can register and pause assets, but must not be able to seize reserves.
- Reserve accounting is transparent and verifiable on-chain.
- Token-2022 confidential transfer primitives provide v1 amount privacy for vAsset transfers.

## Threat Model

| Risk | Mitigation |
| --- | --- |
| Fake stablecoin or fake vAsset | Asset registry keys assets by canonical mint address and SDKs verify registered mints. |
| Unbacked minting | vAsset mint authority is expected to be the vault authority PDA, and wrap/unwrap enforce `total_locked == total_vAsset_minted`. |
| Unauthorized asset registration | Governance signer checks are required for `register_asset` and status changes. |
| Reserve theft by admin | Reserve vault authority should be the program PDA, not a human wallet. |
| Paused/deprecated asset use | Active flows require `AssetStatus::Verified`. |
| User key compromise | Users must protect wallet keys; Veil cannot recover compromised funds. |
| Confidential transfer proof bug | Token-2022 proof generation and account configuration must be tested and audited. |
| Program upgrade risk | Future deployments should use transparent governance, timelocks, and audited upgrade authority policies. |

## v1 Security Requirements Before Mainnet

- Unit and integration tests for every instruction and failure path.
- Localnet and devnet Token-2022 confidential transfer tests.
- Independent audit of PDA authorities, CPI account validation, and reserve invariant handling.
- Formal review of governance powers and upgrade authority.
- Public list of canonical supported mint addresses.

## Current Limitations

The repository is now a stronger v1 foundation, but it is not yet audited mainnet software. The SDK requires an IDL-backed adapter for a deployed program, and confidential transfer setup still depends on Token-2022 client-side proof/account configuration.
