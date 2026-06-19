# Overasset Implementation Plan

## Objective

Represent operational ownership and resource rights for resources, credits, reservations, capacity claims, storage leases, grants, and transferable utility rights where legally enabled.

## First Build Phase

[Phase 5: Metering, ORU, Seal Ledger, and Overbill](../../build_plan/phase_05_metering_oru_seal_ledger_overbill.md) or [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) depending on first right type.

## Dependencies

- Overpass namespace.
- Seal Ledger.
- Overregistry.
- Overgrant.
- Overclaim.

## Development Order

1. Define resource-right and ownership evidence schemas.
2. Add rights for storage leases, capacity claims, grant rights, and namespace-linked rights.
3. Add delegation and transfer where legally enabled.
4. Link every right to Seal Ledger or registry evidence.
5. Add dispute hooks through Overclaim.

## Contracts And Interfaces

- Resource-right record.
- Delegation record.
- Transfer record.
- Ownership evidence refs.

## Detailed SDS

The detailed design contract lives in [Overasset SDS](../../sds/accounting/overasset.md).

## Design Alignment

- Treat Overasset as operational rights and entitlement metadata, not an NFT, collectible, speculative asset, market, or broad legal-title system.
- Store resource-right records, ownership evidence refs, storage entitlements, namespace bindings, capacity claims, grant rights, service ownership refs, delegations, transfers, revocations, disputes, and replay bundles.
- Link every right to ledger, registry, grant, namespace, storage, vault, claim, or signed service evidence and keep transferability blocked unless policy and legal flags allow it.
- Feed Overguard, Universal Namespace Service, Overstore, Overbase, Overvault, Overgrant, Overclaim, Wallet and Usage Center, and central AI stewardship with scoped right refs.

## Validation

- Rights are utility records, not speculative NFTs.
- Transfers cannot bypass policy, legal, or dispute restrictions.
- Ownership evidence is explainable from ledger/registry facts.

## Handoff

Overasset supports namespace rights, storage leases, capacity reservations, grants, and operational ownership flows.
