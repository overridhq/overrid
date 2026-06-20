# Overrid Contract Schemas

This package contains the Phase 2 through Phase 5 CLI contract source plus
Phase 6 hardening contracts and the Rust projection for `SUB BUILD PLAN #2 - CLI`.

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

Phase 5 bootstrap contracts define `phase1_bootstrap_command`,
`signed_command_envelope`, `bootstrap_acceptance_record`,
`manifest_bootstrap_ref`, and `synthetic_workload_pending_state`. These keep
Phase 1 control-plane bootstrap commands on the SDK/Overgate path, require
signature refs instead of key material for mutations, and preserve synthetic
workloads as pending-only records without implying real execution.

Phase 6 hardening contracts define `canonical_idempotency_fingerprint`,
`retry_timeout_policy`, `error_decode_record`, and
`local_idempotency_cache_record`. These lock deterministic retry-safe command
keys, bounded SDK retry/timeout metadata, stable reason-code decoding, and
owner-only cache records that do not contain private payloads.
