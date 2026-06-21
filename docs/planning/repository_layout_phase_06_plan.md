# Complete SUB BUILD PLAN #5 Phase 6 - Package Boundary Enforcement And Modular Control-Plane Shape

## Scope

Implement the Repository Layout Phase 6 gates without turning Repository Layout into a runtime service. This phase makes package dependency direction explicit, keeps service boundary objects flowing through shared schemas, preserves the one modular Rust control-plane process through master Phase 3, and prevents local/test-only helpers from becoming runtime authority.

Source plan: [SUB BUILD PLAN #5 - Repository Layout](../build_plan/sub_build_plan_005_repository_layout.md).

Tech stack authority: [Overrid Tech Stack Choice](../overrid_tech_stack_choice.md).

## Work Items

- **6.1 Dependency direction groups**: add manifest boundary metadata for schemas, SDK, CLI, local tooling, integration tests, control-plane modules, node-agent modules, and docs/specs helpers.
- **6.2 Shared-schema dependency paths**: validate that Rust consumers cite `packages/schemas`/`overrid-contracts` for boundary objects instead of private cross-service structs.
- **6.3 Modular control-plane process**: document and validate the default `services/control-plane` shape as one modular Rust process through master Phase 3.
- **6.4 Split-review criteria**: define measured split criteria tied to API load, failure isolation, security boundary, operational pressure, or grid-resident backbone pressure in Phase 4+ or Phase 7.
- **6.5 Local/test-only separation**: validate that runtime-facing modules do not depend on integration harness, local stack internals, infra/local paths, fixture writers, or docs as executable configuration.

## Implementation Targets

- `docs/build_plan/sub_build_plan_005_repository_layout.md`
- `docs/sds/foundation/repository_layout.md`
- `docs/service_catalog/foundation/repository_layout.md`
- `docs/planning/repository_layout_phase_06_progress.md`
- `overrid.workspace.toml`
- `packages/README.md`
- `packages/cli/README.md`
- `packages/cli/src/runner.rs`
- `services/control-plane/README.md`
- `services/node-agent/README.md`
- `scripts/validate_repository_layout_phase6.py`
- `scripts/validate_overrid.py`

## Validation Plan

- Run `cargo test -p overrid-cli layout_check_emits_phase6_boundary_records -- --nocapture`.
- Run `cargo test -p overrid-cli`.
- Run `python3 scripts/validate_repository_layout_phase6.py`.
- Run `python3 scripts/validate_repository_layout_phase5.py` to prove earlier command-registry gates remain compatible with Phase 6 metadata.
- Run `python3 scripts/validate_overrid.py`.
- Run `docdexd run-tests --repo . --target scripts/validate_repository_layout_phase6.py` when available and record blockers if any.
