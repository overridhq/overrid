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

## Phase 1 Implementation Gates

Phase 1 freezes SDS #7 as a package boundary and metadata gate:

- `attached`: keep links among the SDS, service catalog entry, master build plan, crosswalk, tech-stack decision, package docs, and sub-build plan.
- `boundary_frozen`: treat the package as canonical contract source plus generated/projection outputs, not as runtime authority.
- `phase_0_authority`: keep the first build point in Phase 0.
- `downstream_phase_gated`: expand later schema families only through owning service phase gates.
- `resolved_decision_carried`: preserve the resolved source-of-truth and compatibility decisions.
- `metadata_required`: require schema ownership metadata before release.

The Shared Schema Package is not a deployed microservice, not a runtime registry, not a policy engine, not an audit authority, not an accounting authority, and not a production record store. It must not own runtime policy decisions, service state transitions, audit finality, ledger/accounting finality, production queue state, production registry state, production storage, secret storage, pricing, revenue assumptions, blockchain mechanics, NFT mechanics, PostgreSQL/Redis/S3/Vault product boundaries, or TypeScript core runtime authority.

Required schema ownership metadata:

- `owning_service_family`
- `downstream_consumers`
- `release_status`
- `privacy_class`
- `compatibility_class`
- `review_authority`
- `consumer_notes`

Resolved decisions remain canonical JSON plus JSON Schema authority, Rust generation first, TypeScript/web projections second, Protobuf internal-only, strict unknown-field rejection for sensitive objects, typed low-risk extension maps only, current-plus-previous stable major support, and formal migration plans for authority-sensitive fields.

## Validation

- Valid fixtures pass and invalid fixtures fail deterministically.
- API, worker, node agent, SDK, and CLI use generated types.
- Mutating command schemas include tenant, actor, trace id, and idempotency fields.

## Handoff

This becomes the source of truth for [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md) control-plane APIs and all later service contracts.
