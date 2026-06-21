# Schemas Package Group Contract

`packages/schemas` is the language-neutral source-of-truth group for Overrid boundary contracts.

Owned content:

- canonical JSON Schema for commands, manifests, fixtures, signed payloads, view models, events, audit records, errors, and docs-facing examples.
- Rust projections or validation crates that consume the canonical schemas.
- Generated TypeScript or web bindings only where they are derived from canonical contracts.

Rules:

- Generated code is not the source of truth.
- Protobuf may be added later only for compact internal service/RPC/event contracts when an owning SDS and `docs/specs` entry justify it.
- Contract changes must update schema files, fixtures, validators, SDK/CLI consumers, and docs/specs references together.

## Schema Naming Conventions

- Canonical schemas use `*.schema.json`.
- Versioned schema families live under directories such as `v0/`.
- Valid examples live under `fixtures/valid`.
- Rejection examples live under `fixtures/invalid`.
- Each schema family keeps a `codegen_manifest.json` that names canonical schema paths, fixture roots, generated or projected output paths, source-of-truth status, and validation target.
- Top-level and nested manifest entries that name canonical schema sources or phase-gated schema artifacts must carry `schema_ownership` metadata.
- `packages/schemas/overrid_contracts/v0/shared_schema_package.schema.json` defines the Phase 2 layout and common primitive contract for approved source roots, generated-output roots, fixture roots, compatibility-report roots, internal binary projection roots, typed refs, lifecycle/idempotency fields, privacy classes, and reason-code/correction metadata.
- The same Shared Schema Package schema defines Phase 3 contract-module metadata for identity/tenant, command/API error, event/audit, manifest/registry, and queue/lease/credential/key module groups. These modules are schema definitions only; runtime authority stays with the owning Overrid services.

## Schema Ownership Metadata

Each schema module must carry ownership metadata before it can move from `draft` to `released`:

- `owning_service_family`: SDS/service family responsible for the schema.
- `downstream_consumers`: service, SDK, CLI, UI, adapter, native app, mobile, test, or fixture consumers.
- `release_status`: one of `draft`, `reviewed`, `validated`, `compatible`, `released`, `deprecated`, `retired`, or `blocked`.
- `privacy_class`: public, tenant-private, regulated, encrypted/private, user-content, system-service-only, redacted-diagnostic, or mixed boundary classification.
- `compatibility_class`: additive, deprecated, breaking, migration-required, phase-gated, or current-plus-previous stable major.
- `review_authority`: owning SDS, package maintainer, protocol/schema reviewer, security/privacy reviewer, accounting reviewer, or service owner required for release.
- `consumer_notes`: known downstream consumers, blockers, and handoff constraints.

Draft schema modules without owner, status, privacy class, compatibility metadata, and consumer notes are `metadata_required` and cannot be released. Released modules must preserve canonical JSON Schema authority, Rust-first validation, typed low-risk extension maps only, strict unknown-field rejection for sensitive objects, current-plus-previous stable major compatibility where external consumers exist, and formal migration plans for authority-sensitive fields.

`scripts/validate_shared_schema_package_phase1.py` checks both top-level manifest metadata and nested `codegen_manifest.json` entries, and fails if any canonical `*.schema.json` source under `packages/schemas` lacks manifest ownership coverage.

## Generated Binding Boundaries

Generated or projected Rust and TypeScript/web bindings are consumers only. They must cite their canonical JSON Schema source and mark generated/projection output as non-authoritative.

Approved Phase 4 generated/projection roots:

- `packages/schemas/admin_ui/generated`
- `packages/schemas/overrid_contracts/src/lib.rs`

`packages/schemas/overrid_contracts/src/lib.rs` is a Rust projection from JSON Schema source contracts, not an independent source of truth. TypeScript declaration files under `packages/schemas/admin_ui/generated/typescript` are web/client projections only.

## Optional Protobuf Placement

Protobuf is optional and internal-only. It may be added for compact internal service/RPC/event contracts only after an owning SDS, `docs/specs` entry, package metadata, generated-output root, and validator justify it.

Protobuf must not replace JSON Schema for commands, signed payloads, manifests, fixtures, docs-facing examples, reason codes, audit records, or errors.

## Reason Codes, Events, Audit, And Errors

Reason-code families, event envelopes, audit records, validation artifacts, and error shapes must be discoverable from both this package group and `docs/specs/reason_codes_and_events.md` before mutating service logic is accepted.
