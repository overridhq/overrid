# Shared Schema Package Implementation Plan

## Objective

Create the canonical schemas and generated types used across Overrid service boundaries.

## First Build Phase

[Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Dependencies

- Repository layout.
- Initial protocol object list from `master_services.md`.
- Schema versioning and deprecation policy.

## Development Order

1. Define schema format and code generation strategy.
2. Add identity, tenant, command, manifest, event, audit, queue, lease, usage, ledger, policy, and dispute schemas.
3. Add validators and fixture builders.
4. Add schema compatibility tests.
5. Add deprecation metadata before the first external SDK release.

## Contracts And Interfaces

- Generated types for services, SDK, CLI, and tests.
- Runtime validation helpers.
- Fixture builders for integration tests.
- Versioned schema registry metadata.

## Detailed SDS

- [Shared Schema Package SDS](../../sds/foundation/shared_schema_package.md)

## Detailed Build Plan

- [SUB BUILD PLAN #7 - Shared Schema Package](../../build_plan/sub_build_plan_007_shared_schema_package.md)

## Design Alignment

- Treat the shared schemas as the canonical contract package, not a deployed registry or policy service.
- Require strict schemas for command envelopes, events, audit records, usage records, ORU objects, Seal Ledger refs, AI assistant refs, encrypted Docdex RAG refs, and native-service payloads.
- Generate types, validators, fixtures, docs, and compatibility reports from one schema source.
- Make mutating command schemas require tenant, actor, trace id, idempotency key, command type, and schema version.
- Block public API, worker, SDK, or CLI surfaces that use untyped or forked payload definitions.

## Validation

- Valid fixtures pass and invalid fixtures fail deterministically.
- API, worker, node agent, SDK, and CLI use generated types.
- Mutating command schemas include tenant, actor, trace id, and idempotency fields.

## Handoff

This becomes the source of truth for [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md) control-plane APIs and all later service contracts.
