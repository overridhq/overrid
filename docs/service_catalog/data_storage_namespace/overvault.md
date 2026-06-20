# Overvault Implementation Plan

## Objective

Build secure storage for sensitive material, encrypted state, secrets, escrowed records, private app data, and protected access policies.

## First Build Phase

[Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md), with minimal secret references earlier.

## Dependencies

- Overkey.
- Overguard policy.
- Overpass identity.
- Overtenant context.
- Overwatch access, emergency, escrow, revocation, and cleanup evidence.
- Overmeter raw vault operation usage.
- Overrun and Overcell for lease-bound secret delivery and cleanup evidence.
- Overbase private-record refs and access-scoped projection refs.
- Overstore encrypted payload/object refs where large private objects need content-addressed storage.
- Overmesh secure service or node delivery paths where required.

## Development Order

1. Define secret, secret version, encrypted record, access policy, access request, decision, grant, mount lease, rotation, revocation, escrow, emergency, and redaction schemas.
2. Add Overvault-owned envelope encryption metadata and metadata-only secret/encrypted-record APIs.
3. Add access request, Overguard decision, bounded grant, and denial records.
4. Add minimal founder-local secret refs for Overrun, then full Phase 8 mount leases and cleanup hooks.
5. Add rotation, revocation, ttl expiry, quarantine, tombstones, and redaction checks.
6. Add private app data scopes and access-scoped search projections for private/RAG records.
7. Add escrow, emergency access, regulated-workload admission gates, and post-action review.

## Contracts And Interfaces

- Secret ref contract.
- Encrypted record API.
- Access request, access decision, and access grant events.
- Mount lease and cleanup contract.
- Key policy metadata.
- Rotation, revocation, quarantine, and tombstone events.
- Escrow and emergency-access workflow records.
- Redaction report and raw usage event contracts.

## Detailed SDS

The detailed design contract lives in [Overvault SDS](../../sds/data_storage_namespace/overvault.md).

- [SUB BUILD PLAN #29 - Overvault](../../build_plan/sub_build_plan_029_overvault.md)

## Design Alignment

- Treat Overvault as the vault boundary for secret refs, encrypted private records, access decisions, grants, mount leases, rotation, revocation, escrow, and redaction policy.
- Keep identity and credential metadata in Overpass/Overkey, structured app state in Overbase, and object bytes in Overstore.
- Require explicit purpose, ttl, actor/service-account, tenant/app scope, data class, and Overguard policy refs before any release.
- Ensure Overrun receives only policy-approved mount leases and that raw secret values never enter logs, run results, audit payloads, or diagnostics.
- Use Overvault-owned envelope encryption before persistence or delivery; external Vault/KMS-style systems may be benchmark or bridge adapters later but must not become the product boundary.
- Keep private search projections access-scoped and treat private embeddings as private or secret-bearing artifacts.
- Block regulated workloads until escrow, evidence hold, appeal, redaction, retention, and emergency release paths are defined for the required data class.

## Validation

- Secrets are never exposed to workloads without policy approval.
- Access decisions are auditable.
- Private app state remains tenant/user scoped.
- Raw secret values and decrypted private payloads never appear in logs, Overwatch payloads, Overmeter events, Overrun results, diagnostics, environment dumps, normal artifact storage, or shared indexes.
- Mount leases expire, revoke, and clean up with evidence.
- Rotation, revocation, quarantine, escrow, emergency access, and private search projection paths are replayable from stored refs.

## Handoff

Overvault supports Overrun secret mounts, personal AI, workspace, messaging, mobile apps, and regulated workloads.
