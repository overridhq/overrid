# Overrid CLI

This package is the Phase 5 Rust CLI bootstrap slice for `SUB BUILD PLAN #2 - CLI`.

It is a Cargo workspace member and depends on the Rust contract projection in
`packages/schemas/overrid_contracts` plus the SDK transport skeleton in
`packages/sdk`. The package must remain a Rust CLI runtime. TypeScript bindings
may exist elsewhere as generated client projections, but they are not this CLI's runtime.

The canonical CLI schema source is
`packages/schemas/overrid_contracts/v0/cli_command.schema.json`; the Rust
projection is consumed by this crate but is not the contract authority.

Phase 5 keeps the CLI runtime Rust-first while exposing the Phase 1 bootstrap
surface for auth, tenant, identity, key, manifest, and synthetic workload
commands. Mutating bootstrap commands render signed command envelopes with
secret-free signature refs, deterministic idempotency keys, trace ids, and
acceptance/audit refs after SDK Overgate-only validation.

Synthetic workload commands intentionally stop at pending state without implying execution.
Real workload logs, cancellation, results, and follow-mode remain phase-gated
until the owning execution services and contracts are ready.
