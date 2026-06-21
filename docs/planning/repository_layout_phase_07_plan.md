# Complete SUB BUILD PLAN #5 Phase 7 - Generated Artifacts, Secrets, Local State, And Index Hygiene

## Scope

Implement the Repository Layout Phase 7 gates without turning Repository Layout into a runtime service. This phase makes generated-output ignore rules, local-state hygiene, secret-file policy, Docdex indexing hygiene, and validation-artifact redaction explicit in the Rust-first layout contract.

Source plan: [SUB BUILD PLAN #5 - Repository Layout](../build_plan/sub_build_plan_005_repository_layout.md).

Tech stack authority: [Overrid Tech Stack Choice](../overrid_tech_stack_choice.md).

## Work Items

- **7.1 Generated-output ignore rules**: record and validate ignored build outputs, dependency caches, coverage, logs, generated specs/docs/types, integration artifacts, fixture outputs, and temporary object chunks.
- **7.2 Local-state ignore rules**: keep local state under `.overrid/`, `infra/local/state/`, `infra/local/job-tables/`, `infra/local/artifacts/`, or approved local/test-only paths with marker-gated reset expectations.
- **7.3 Secret-file rules**: allow example files only and reject committed `.env`, local, secret, key, token, private-key, and fixture-credential paths outside approved generated/local-test paths.
- **7.4 Docdex indexing hygiene**: keep docs, specs, SDS, build plans, service catalog files, source schemas, handwritten fixtures, and service contract stubs indexed while excluding generated artifacts and local caches.
- **7.5 Artifact redaction expectations**: ensure layout validation artifacts, docs checks, and CI bundles report secret findings by path/reason only and do not expose keys, tokens, signatures, private payloads, encrypted content, or local fixture credentials.

## Implementation Targets

- `.gitignore`
- `.docdexignore`
- `docs/build_plan/sub_build_plan_005_repository_layout.md`
- `docs/sds/foundation/repository_layout.md`
- `docs/service_catalog/foundation/repository_layout.md`
- `docs/planning/repository_layout_phase_07_progress.md`
- `overrid.workspace.toml`
- `packages/README.md`
- `packages/cli/README.md`
- `infra/local/README.md`
- `tests/integration/README.md`
- `packages/cli/src/runner.rs`
- `scripts/validate_repository_layout_phase7.py`
- `scripts/validate_overrid.py`

## Validation Plan

- Run `cargo test -p overrid-cli layout_check_emits_phase7_hygiene_records -- --nocapture`.
- Run `cargo test -p overrid-cli layout_check_rejects_phase7_hygiene_violations -- --nocapture`.
- Run `cargo test -p overrid-cli`.
- Run `python3 scripts/validate_repository_layout_phase7.py`.
- Run `python3 scripts/validate_repository_layout_phase6.py` to prove boundary checks remain compatible.
- Run `python3 scripts/validate_overrid.py`.
- Run `docdexd run-tests --repo . --target scripts/validate_repository_layout_phase7.py` when available and record blockers if any.

