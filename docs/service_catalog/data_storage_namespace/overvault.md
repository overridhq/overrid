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

## Development Order

1. Define secret, encrypted state, and private record schemas.
2. Add access policies and audit events.
3. Add secret refs usable by Overrun.
4. Add escrowed record support where needed by disputes or regulated workflows.
5. Add private app data scopes.

## Contracts And Interfaces

- Secret ref contract.
- Encrypted record API.
- Access decision event.
- Key policy metadata.

## Detailed SDS

The detailed design contract lives in [Overvault SDS](../../sds/data_storage_namespace/overvault.md).

## Design Alignment

- Treat Overvault as the vault boundary for secret refs, encrypted private records, access decisions, grants, mount leases, rotation, revocation, escrow, and redaction policy.
- Keep identity and credential metadata in Overpass/Overkey, structured app state in Overbase, and object bytes in Overstore.
- Require explicit purpose, ttl, actor/service-account, tenant/app scope, data class, and Overguard policy refs before any release.
- Ensure Overrun receives only policy-approved mount leases and that raw secret values never enter logs, run results, audit payloads, or diagnostics.

## Validation

- Secrets are never exposed to workloads without policy approval.
- Access decisions are auditable.
- Private app state remains tenant/user scoped.

## Handoff

Overvault supports Overrun secret mounts, personal AI, workspace, messaging, mobile apps, and regulated workloads.
