# Overrid CLI

This package contains the Phase 5 Rust CLI bootstrap slice, Phase 6
automation hardening, Phase 7 seed private swarm/execution command slice,
Phase 8 policy/package/accounting read command slice, Phase 9 product
integration/automation hardening slice, and Phase 10 validation/release
readiness handoff slice
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

Phase 9 product integration attaches `product_workflow_recipe` output to
Docdex, Mcoda, and Codali workload paths. The recipes document submit,
inspect/status, logs/result, cancellation, usage, and receipt commands through
the existing Rust CLI/SDK/Overgate route, with authorized refs only and no
direct internal APIs, raw HTTP, private storage, hardcoded model/node/provider,
or paid-service assumptions. CI automation outputs `ci_automation_profile`
metadata for explicit `environment=ci` profiles, requiring short-lived or
mounted credential refs, stable secret-free JSON, non-interactive behavior, and
no ambient persistent keychain defaults.

Phase 10 adds `release-readiness` as a local validation evidence command. It
renders contract snapshot, help/output, exit-class, reason-code, security
redaction, phase-availability, integration reliability, automation
compatibility, and Phase 7/13 handoff evidence through the same stable output
envelope. It does not enable deployment, governance, compliance, incident,
break-glass, migration, or backbone operations early; those routes remain
denied with `not_available_in_phase` until owning contracts exist.
