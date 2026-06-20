# Overrid Contract Schemas

This package contains the Phase 2 and Phase 3 CLI contract source and Rust
projection for `SUB BUILD PLAN #2 - CLI`.

The JSON Schema files under `v0/` are the canonical docs-facing source for this
slice. The Rust crate in `src/` is the initial generated/projection layer that
CLI and SDK code consume while later phases mature schema generation.

Generated or projected code must not become the contract authority. Changes to
CLI output envelopes, trace context, idempotency records, API errors,
diagnostic bundles, profile records, credential references, confirmation
policy, or signer handoff start in the canonical schema and manifest before
parser, SDK, or runtime behavior changes.

Phase 3 profile and credential contracts are intentionally secret-free. Profile
records carry environment class, endpoint fingerprint, tenant/actor ids,
credential namespace, allowed credential-reference classes, fixture allowance,
confirmation policy, and schema pins. Credential references point to approved
stores or signing agents and must not contain raw key material.
