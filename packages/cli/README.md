# Overrid CLI

This package contains the Phase 5 Rust CLI bootstrap slice, Phase 6
automation hardening, Phase 7 seed private swarm/execution command slice, and
Phase 8 policy/package/accounting read command slice
for `SUB BUILD PLAN #2 - CLI`.

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

Synthetic workload submit commands intentionally stop at pending state without implying execution.
Phase 7 adds node registration/inspection/health plus real workload status,
timeline, logs, cancellation, result, and follow-mode output through SDK/Overgate
contracts and authorized service refs.

Phase 6 automation hardening keeps that same SDK/Overgate path and adds
canonical idempotency fingerprints, `--new-idempotency-key` behavior, bounded retry and timeout policy,
trace/audit propagation, stable error decode records, and a secret-free local idempotency cache
with inspect/reset output.

Phase 7 seed private swarm support keeps the CLI on the same Rust SDK/Overgate path.
Node commands render `node_status_record` data with profile-scoped credential
checks, capability refs, and no direct node access. Workload execution commands
render `execution_timeline`, `execution_log_bundle`, `execution_result_ref`,
and `polling_plan` data with redaction, bounded streaming/polling, trace-linked
diagnostics, and no direct queue, runner, node, or object-store paths.

Phase 8 commands keep policy dry-runs, package validation, usage/ORU reads,
receipt and ledger refs, and dispute read models fixture-backed until owning
services expose live backends. Outputs use authorized SDK/Overgate refs only,
stay dry-run or read-only, and explicitly avoid pricing, revenue,
customer-count, market-volume, blockchain, or NFT assumptions.
