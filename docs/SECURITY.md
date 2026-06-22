# Veil Security Model

Veil is designed around explicit, auditable security guarantees.

## Guarantees

- vAssets must be backed 1:1 by underlying stablecoins.
- Users keep custody of their wallets and signing keys.
- Governance can register assets, but must not be able to seize reserves.
- Reserve accounting is transparent and verifiable on-chain.

## Threat Model

| Risk | Mitigation |
| --- | --- |
| Unbacked minting | Enforce `total_locked == total_vasset_minted` accounting. |
| Unauthorized asset registration | Governance signer checks on `register_asset`. |
| User key compromise | Users must protect wallet keys; the protocol cannot recover compromised funds. |
| Program upgrade risk | Future deployments should use transparent governance and timelocks. |
| Privacy implementation bugs | Token-2022 confidential transfer integration must be audited before production use. |

## Audit Notes

The current repository is a v1 skeleton and is not production-ready. Before mainnet deployment, Veil requires comprehensive tests, independent audit review, and formal verification of critical reserve invariants where practical.
