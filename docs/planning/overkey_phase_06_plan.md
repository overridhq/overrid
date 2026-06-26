# Complete SUB BUILD PLAN #9 Phase 6 - Delegated Access, Service Accounts, Policy Handoff, And Usage Hooks

## Goal

Complete SUB BUILD PLAN #9 Phase 6 for Overkey and align the implementation, schema contract, fixtures, docs, and validation wiring with `docs/overrid_tech_stack_choice.md`.

## Scope

- Validate delegated access metadata against the existing Overkey credential repository boundary.
- Validate service-account scopes through an explicit local service-to-command matrix.
- Keep Overguard policy handoff as a referenced decision, not policy truth stored by Overkey.
- Add last-used and usage-relevant event reporting without mutating ORU balances or Seal Ledger state.
- Harden operator lifecycle controls with signed Overgate commands, operator role checks, strong protection-class metadata, evidence refs, and audit refs.
- Preserve the Rust-first stack: Axum/Tower/Hyper service surfaces, canonical JSON envelopes, JSON Schema contracts, Ed25519 semantics, BLAKE3/content refs, and native Overrid service refs.

## Gap Review

- The Rust service already contained most Phase 6 behavior in `packages/overkey/src/routes.rs`, `packages/overkey/src/records.rs`, and `packages/overkey/src/repository.rs`.
- The schema contract was misaligned because `delegation_record` did not require Phase 6 fields such as delegator/delegate refs, allowed scopes, command classes, revocation state, evidence refs, and policy decision refs.
- Phase 6 fixtures, suite registration, README documentation, and planning/progress evidence were missing.
- Existing tests covered adjacent behavior but did not explicitly prove every Phase 6 gate from the build plan.

## Implementation Tasks

1. Extend the Overkey JSON Schema delegation record contract with Phase 6 delegated access metadata.
2. Add valid and invalid Phase 6 fixtures for delegated access, service-account scopes, Overguard policy handoff, last-used usage-relevant event reporting, and operator lifecycle controls.
3. Add Rust tests for Overgate-only delegation ingress, evidence and policy handoff denials, expired delegation denial, cross-tenant delegation denial, narrow scope enforcement, service-account scope matrix denials, retry-safe usage hook metadata, and signed operator lifecycle controls.
4. Add `scripts/validate_overkey_phase6.py` and register it in `scripts/validate_overrid.py`.
5. Register `python3 scripts/validate_overkey_phase6.py` in `overrid.workspace.toml`.
6. Update `packages/overkey/README.md` with Phase 6 routes, fixtures, boundaries, and validation command.
7. Add this plan and `docs/planning/overkey_phase_06_progress.md` as durable handoff evidence.

## Validation Plan

- `cargo fmt --all -- --check`
- `cargo test -p overrid-overkey phase6 -- --nocapture`
- `python3 scripts/validate_overkey_phase5.py`
- `python3 scripts/validate_overkey_phase6.py`
- `python3 scripts/validate_overrid.py`
- `docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid --target scripts/validate_overkey_phase6.py`
- `git diff --check`
- `docdexd hook pre-commit --repo /Users/bekirdag/Documents/apps/overrid`

## Docdex And DAG Evidence

- Docdex repo inspection and index stats were checked before editing.
- Docdex profile and repo memory were loaded before the implementation audit.
- Docdex impact graph showed `packages/overkey/src/lib.rs` as the inbound Rust dependency for `routes.rs` and `records.rs`; `routes.rs` depends on `records.rs`, `repository.rs`, `schema.rs`, and Phase 2 fixtures.
- DAG export for the Overkey Phase 6 audit session was used to keep schema, route tests, docs, and validation wiring ordered from contract changes to evidence changes.

## Tech-Stack Alignment

The phase stays inside the selected Overrid stack. It uses Rust service code, Axum/Tower-compatible HTTP routing, canonical JSON responses, JSON Schema fixtures, Ed25519/BLAKE3-compatible metadata, Overgate/Overguard/Overmeter service refs, and local in-memory repository stubs. It does not introduce external cloud primitives, database products, queue products, blockchain assumptions, pricing assumptions, or direct secret storage.
