# Repository Layout Phase 10 Plan

## Goal

Complete SUB BUILD PLAN #5 Phase 10: Validation, Documentation Alignment, And Downstream Handoff.

## Scope

- Confirm the Phase 10 requirements from `docs/build_plan/sub_build_plan_005_repository_layout.md`.
- Preserve the Rust-first/native-Overrid constraints in `docs/overrid_tech_stack_choice.md`.
- Add Phase 10 alignment and handoff metadata to `overrid.workspace.toml`.
- Extend `packages/cli/src/runner.rs` so `overrid layout:check --json` emits Phase 10 alignment and downstream handoff records.
- Add `scripts/validate_repository_layout_phase10.py` and register it in `scripts/validate_overrid.py`.
- Update Repository Layout SDS and service-catalog docs with Phase 10 closure gates.
- Check master-plan, crosswalk, local links, work-item structure, tech-stack denial terms, and downstream expansion rules.

## Phase Tasks

1. Validate sub-build-plan structure: title prefix, attached SDS link, phase headings 1 through 10, work item structure, Design/Output/Validation fields, local links, and exit gate.
2. Validate tech-stack alignment: Rust-first workspace ownership, language-neutral schema authority, Overrid-shaped local primitives, modular control-plane boundaries, and no forbidden conventional cloud/product assumptions except explicit rejection text.
3. Validate master-plan alignment: SDS #5 remains a Phase 0 foundation sub-build plan and the master Phase 0 through Phase 13 order remains unchanged.
4. Validate SDS and service-catalog alignment: source documents link back to the sub-build plan and preserve Repository Layout as a non-runtime build-contract artifact.
5. Prepare downstream handoff: define phase-by-phase rules for later builders to add modules, contracts, tests, local profiles, generated outputs, and docs without top-level sprawl or tech-stack drift.

## Validation Plan

- `cargo fmt --all -- --check`
- `cargo test -p overrid-cli layout_check_emits_phase10_alignment_handoff_records -- --nocapture`
- `cargo test -p overrid-cli layout_check_rejects_phase10_alignment_handoff_violations -- --nocapture`
- `python3 scripts/validate_repository_layout_phase10.py`
- `python3 scripts/validate_overrid.py`
- `docdexd run-tests --repo /Users/bekirdag/Documents/apps/overrid --target scripts/validate_repository_layout_phase10.py`
- `git diff --check`

## Tech-Stack Alignment

This phase remains a Rust-first repository-layout validation and handoff closure. It must not introduce PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions as Overrid runtime authority. TypeScript remains limited to web/client surfaces and generated bindings.
