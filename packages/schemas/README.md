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
