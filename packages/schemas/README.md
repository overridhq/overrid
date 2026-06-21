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
