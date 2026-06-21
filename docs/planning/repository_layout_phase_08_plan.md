# Repository Layout Phase 8 Plan

## Goal

Complete SUB BUILD PLAN #5 Phase 8: Service Contract Templates And New-Module Checklist.

## Scope

- Confirm the Phase 8 requirements from `docs/build_plan/sub_build_plan_005_repository_layout.md`.
- Preserve the Rust-first/native-Overrid constraints in `docs/overrid_tech_stack_choice.md`.
- Add Phase 8 gate outputs for service contract template usage, new-module checklist, module lifecycle, deprecation/removal, and cross-document maintenance.
- Add a `docs/specs/new_module_checklist.md` source contract and link it from `docs/specs/README.md`.
- Update `overrid.workspace.toml` with Phase 8 module lifecycle metadata, checklist paths, violation artifacts, and validation script wiring.
- Extend `packages/cli/src/runner.rs` so `overrid layout:check` emits Phase 8 lifecycle/checklist records and rejects invalid lifecycle metadata.
- Add `scripts/validate_repository_layout_phase8.py` and register it in `scripts/validate_overrid.py`.

## Validation Plan

- `docdexd run-tests --repo . --target scripts/validate_repository_layout_phase8.py`
- `cargo test -p overrid-cli layout_check_emits_phase8_lifecycle_records -- --nocapture`
- `cargo test -p overrid-cli layout_check_rejects_phase8_lifecycle_violations -- --nocapture`
- `cargo test -p overrid-cli`
- `python3 scripts/validate_repository_layout_phase8.py`
- `python3 scripts/validate_overrid.py`
