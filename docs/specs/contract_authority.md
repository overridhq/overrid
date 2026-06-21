# Contract Authority

## Purpose

Define where Overrid contract sources live before services, SDKs, CLI code, UI code, local-stack helpers, or integration tests consume boundary objects.

## Canonical Schema Sources

`packages/schemas` is the language-neutral source of truth for:

- commands;
- manifests;
- fixtures;
- signed payloads;
- view models;
- events;
- audit records;
- errors;
- docs-facing examples.

Canonical human-readable contracts use JSON Schema files named `*.schema.json` under versioned schema-family directories such as `v0/`. Fixture evidence is split into `fixtures/valid` and `fixtures/invalid`.

Each schema family must keep a `codegen_manifest.json` that names the canonical schema, fixture paths, generated or projected outputs, source-of-truth status, and validation target.

## Generated Binding Boundaries

Generated or projected Rust and TypeScript/web bindings are consumers. They are not contract authority.

Generated/projection metadata must name:

- canonical source schema path;
- generated or projected output path;
- source-of-truth status, normally `json_schema`;
- non-authoritative status;
- validator or test target.

Approved generated/projection roots for Phase 4 are:

- `packages/schemas/admin_ui/generated`
- `packages/schemas/overrid_contracts/src/lib.rs`
- `docs/specs/generated`

The Rust projection at `packages/schemas/overrid_contracts/src/lib.rs` is a temporary Rust-first projection from canonical JSON Schema sources. It must continue to expose projection status and schema-family/version constants so consumers cannot mistake it for the source of truth.

Generated files outside approved generated roots are layout violations until an owning SDS/spec update adds the path, ignore rule, and validation evidence.

## Optional Protobuf Placement

Protobuf is optional and internal-only for Phase 4. It may be introduced only for compact internal service/RPC/event contracts when all of these exist:

- owning SDS reference;
- `docs/specs` entry describing why JSON Schema is not enough for that internal contract;
- package metadata naming generated output roots and validation target;
- proof that JSON Schema remains canonical for docs-facing commands, manifests, signed payloads, fixtures, view models, reason codes, audit records, and errors.

Until then, no `.proto` files are required.

## Validation

`scripts/validate_repository_layout_phase4.py` checks this contract, `packages/schemas/README.md`, codegen manifests, generated/projection metadata, `overrid.workspace.toml`, and local Markdown links.
