# Repository Layout Implementation Plan

## Objective

Define the physical workspace for Overrid services, packages, SDKs, CLI tools, specs, tests, and local infrastructure so early development does not sprawl.

## First Build Phase

[Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Detailed SDS

[Repository Layout SDS](../../sds/foundation/repository_layout.md).

## Sub-Build Plan

[SUB BUILD PLAN #5 - Repository Layout](../../build_plan/sub_build_plan_005_repository_layout.md).

## Dependencies

- Current whitepaper and build plan.
- Agreement that the first implementation is modular, not many premature microservices.
- Local development and test conventions.

## Development Order

1. Freeze Phase 1 SDS attachment, non-runtime boundary, master Phase 0 gate, resolved SDS decisions, and layout-change governance lifecycle.
2. Create top-level folders for `services`, `packages`, `infra`, `tests`, and `docs/specs`.
3. Define ownership boundaries for control plane, node agent, SDK, CLI, schemas, and integration tests.
4. Add the root workspace manifest and module inventory records.
5. Add placeholder service contract docs for each initial subsystem.
6. Add build/test command conventions and a root task runner.
7. Document how new services are added without bypassing shared schemas.

## Contracts And Interfaces

- Folder naming convention.
- Package dependency rules.
- Service contract template.
- Test and local-stack command names.
- Workspace manifest and module record conventions.
- Generated-artifact and secret-file exclusion rules.
- New-service checklist.

## Design Alignment

- Repository layout is a governance/build-contract artifact, not a runtime service.
- Phase 0 keeps the first implementation modular but not over-split.
- Shared schemas are the required dependency path for service boundaries.

## Phase 1 Implementation Gates

- `attached`: this service plan, the SDS, master plan, build-plan crosswalk, tech-stack decision, and phase planning docs link to [SUB BUILD PLAN #5 - Repository Layout](../../build_plan/sub_build_plan_005_repository_layout.md).
- `boundary_frozen`: Repository Layout is not a network service, not a runtime service registry, not a deployment orchestrator, not a configuration database, not a production configuration source, not hidden service discovery, not an external cloud account boundary, not a service-storage shortcut, and not a shortcut around service contracts.
- `master_phase_0_owned`: Phase 0 owns initial workspace shape, command names, schema/source paths, generated/local ignore rules, and validation contracts.
- `later_phase_consumer`: Phase 1 through Phase 13 service work consumes the established layout and can expand it only with matching SDS, phase, service-catalog, and crosswalk evidence.
- `resolved_decision_carried`: Rust-owned command registry, modular control-plane process through Phase 3, language-neutral `packages/schemas` authority, generated/local ignore rules, and minimal `overrid.workspace.toml` manifest remain fixed decisions.
- `governance_required`: layout additions, generated paths, service boundary changes, deprecations, and removals move through `proposed`, `scaffolded`, `contracted`, `wired`, `validated`, `accepted`, `deprecated`, and `removed` states as applicable.

## Phase 2 Implementation Gates

- `top_level_contracts_scaffolded`: `services`, `packages`, `infra`, `tests`, `docs/specs`, `docs/build_plan`, `docs/service_catalog`, and `docs/sds` exist as Phase 0 workspace-shape contracts.
- `service_path_rules_defined`: `services/control-plane` remains the modular Rust control-plane process boundary through Phase 3 by default, and `services/node-agent` is reserved for Overcell node-agent and simulator code.
- `package_path_rules_defined`: `packages/schemas`, `packages/sdk`, and `packages/cli` have explicit ownership rules, with schemas as contract authority, SDK as Rust-first generated/validated client layer, and CLI as Rust operator tooling; already implemented companion roots such as `packages/admin_ui_shell`, `packages/integration_harness`, and `packages/local_stack` also carry README ownership metadata, source SDS/build-plan refs, and test-target declarations without becoming core runtime authority.
- `local_infra_test_paths_defined`: `infra/local` owns Overrid-shaped local profiles and service definitions, while `tests/integration` owns cross-service scenarios; local state, job tables, artifact stubs, integration artifacts, and run outputs are ignored by default.
- `specs_contract_defined`: `docs/specs` owns hand-authored protocol, schema, API, service-contract, reason-code, event-contract, audit-record, and validation-artifact docs; `docs/specs/generated` is ignored generated output.

## Phase 3 Implementation Gates

- `workspace_manifest_defined`: `overrid.workspace.toml` records schema version, manifest metadata, source document links, validation metadata, module-record schema enums, workspace inventory roots, and drift reason codes as validation/build metadata only.
- `module_records_defined`: each Phase 0 module record defines `name`, `type`, `owner_layer`, `path`, `master_phase`, `public_contract_path`, `allowed_dependency_groups`, `generated_output_paths`, `test_targets`, `local_stack_participation`, and `documentation_links`.
- `inventory_discovery_defined`: layout checks compare the manifest with `Cargo.toml` workspace members, direct `packages/` roots, local profile roots, service-definition roots, test roots, specs roots, and generated-output ignore markers.
- `phase_owner_metadata_defined`: module records use accepted phase, owner, type, dependency, lifecycle, and local-stack participation metadata, with later promotions requiring SDS, service-plan, phase, and crosswalk evidence.
- `manifest_drift_checks_defined`: `scripts/validate_repository_layout_phase3.py` reports deterministic reason codes for manifest drift, including `missing_schema_version`, `missing_module_records`, `missing_required_field`, `duplicate_module_name`, `invalid_path`, `missing_path`, `unknown_module_type`, `unknown_owner_layer`, `invalid_master_phase`, `unknown_dependency_group`, `missing_public_contract`, `stale_documentation_link`, `missing_test_target`, `forbidden_generated_path`, `missing_local_stack_participation`, `cargo_member_drift`, and `unlisted_module`.

## Phase 4 Implementation Gates

- `schema_authority_defined`: `packages/schemas` is the canonical JSON Schema authority for commands, manifests, fixtures, signed payloads, view models, events, audit records, errors, and docs-facing examples.
- `generated_binding_boundaries_defined`: Rust and TypeScript/web generated or projected bindings are non-authoritative consumers that must cite source schemas, output paths, source-of-truth status, and validation targets.
- `protobuf_placement_defined`: Protobuf is allowed only for compact internal service/RPC/event contracts with owning SDS and `docs/specs` justification, not for docs-facing commands, manifests, signed payloads, fixtures, reason codes, audit records, or errors.
- `service_contract_template_defined`: `docs/specs/service_contract_template.md` defines required service/module contract sections before implemented service logic is accepted.
- `reason_event_contracts_defined`: `docs/specs/reason_codes_and_events.md` and `packages/schemas` define reason-code, event-envelope, audit-record, validation-artifact, and error-shape placement.
- `phase4_validation_defined`: `scripts/validate_repository_layout_phase4.py` validates the Phase 4 docs, schema package metadata, workspace manifest metadata, planning trail, and local Markdown links.

## Phase 5 Implementation Gates

- `root_command_registry_defined`: `overrid.workspace.toml` and `overrid command-registry` list `build`, `test`, `test:integration`, `dev:start`, `dev:stop`, `dev:reset`, `dev:seed`, `dev:status`, `schema:check`, `docs:check`, and `layout:check` with purpose, inputs, outputs, owner, phase gate, canonical invocation, envelope support, failure classes, and alias metadata.
- `rust_owned_command_execution_defined`: `packages/cli` owns the canonical command registry and `layout:check` behavior. Thin aliases may call the Rust CLI, but authoritative behavior must not live only in shell, Make, just, npm, or docs.
- `layout_check_defined`: `overrid layout:check` validates required directories, manifest records, module/test-target markers, service contract stubs, generated-output ignore markers, secret-file absence, package-boundary metadata, local-state markers, and docs contract paths with human and JSON output.
- `schema_docs_check_orchestration_defined`: `schema:check`, `docs:check`, and `layout:check` are local/CI semantic records with stable pass/fail/block statuses and artifact refs.
- `validation_artifacts_defined`: `layout_check.passed`, `layout_check.failed`, `package_boundary_violation`, `missing_service_contract`, `missing_test_target`, `generated_file_committed`, and `secret_file_committed` are CI/build artifacts, not Overwatch events.
- `phase5_validation_defined`: `scripts/validate_repository_layout_phase5.py` validates the Phase 5 docs, manifest root-command registry, Rust CLI parser/runner wiring, secret-free layout-check output, planning trail, and suite registration.

## Phase 6 Implementation Gates

- `dependency_direction_groups_defined`: `overrid.workspace.toml` records Phase 6 dependency groups for contracts, SDK, CLI, local/test helpers, docs/specs helpers, control-plane modules, and node-agent modules.
- `shared_schema_dependency_paths_enforced`: service boundary objects must cite `packages/schemas`, `overrid-contracts`, or `docs/specs` before becoming cross-package or cross-service contracts.
- `modular_control_plane_shape_preserved`: `services/control-plane` remains one modular Rust process through master Phase 3 by default, while domain internals remain modules/crates/contracts inside that boundary.
- `split_review_criteria_defined`: service splits require measured API load, failure-isolation, security-boundary, operational, or grid-resident backbone pressure plus SDS/service-catalog/build-plan/spec updates.
- `local_test_only_separation_enforced`: runtime-facing modules must not depend on integration harness internals, local stack internals, `infra/local`, fixture writers, integration artifacts, local simulator internals, or docs files as executable configuration.
- `phase6_validation_defined`: `scripts/validate_repository_layout_phase6.py` validates Phase 6 docs, manifest boundary metadata, Rust CLI layout-check evidence, package/service README evidence, planning trail, suite registration, and local Markdown links.

## Phase 7 Implementation Gates

- `generated_output_ignore_rules_defined`: `.gitignore`, `.docdexignore`, and `overrid.workspace.toml` define generated-output, cache, coverage, log, generated spec/type/doc, integration artifact, fixture-output, and temporary object-chunk ignore rules.
- `local_state_ignore_rules_defined`: local state remains under `.overrid`, `infra/local/state`, `infra/local/job-tables`, `infra/local/artifacts`, and `tests/integration/artifacts` with source-visible ignore markers and resettable local/test-only semantics.
- `secret_file_rules_defined`: only example files may be source-controlled; secret-like env, local, key, token, private-key, and fixture-credential paths are rejected without exposing raw values.
- `docdex_indexing_hygiene_defined`: Docdex keeps docs/specs/SDS/build plans/service catalog files/source schemas/handwritten fixtures/service stubs indexed while excluding generated outputs, local state, caches, logs, and coverage.
- `artifact_redaction_expectations_defined`: validation artifacts, docs checks, CI bundles, local-stack exports, and harness artifacts redact secret-bearing values and report only safe path/reason metadata.
- `phase7_validation_defined`: `scripts/validate_repository_layout_phase7.py` validates Phase 7 docs, manifest artifact-hygiene metadata, ignore files, Rust CLI layout-check evidence, planning trail, suite registration, and local Markdown links.

## Phase 8 Implementation Gates

- `service_contract_template_implemented`: `docs/specs/service_contract_template.md` carries the reusable service/module contract sections, usage notes, and downstream dependency expectations.
- `new_service_checklist_defined`: `docs/specs/new_module_checklist.md` defines the module addition checklist for SDS, service catalog, build plan/crosswalk, specs, schemas, tests, local-stack participation, generated-output rules, and module records.
- `module_addition_workflow_defined`: `overrid.workspace.toml` records lifecycle states for proposed, scaffolded, contracted, wired, validated, and accepted modules.
- `deprecation_removal_workflow_defined`: deprecated and removed states require replacement/removal notes, stale-reference cleanup, manifest updates, tests, local-stack updates, harness scenario updates, generated-output updates, and Docdex reference updates.
- `cross_document_maintenance_rules_defined`: repository layout changes keep SDS, service catalog, sub-build plan, master plan, crosswalk, specs, manifest, and phase planning/progress docs aligned.
- `phase8_validation_defined`: `scripts/validate_repository_layout_phase8.py` validates Phase 8 docs, manifest lifecycle metadata, Rust CLI layout-check evidence, planning trail, suite registration, and local Markdown links.

## Phase 9 Implementation Gates

- `local_stack_discovery_metadata_defined`: `overrid.workspace.toml` exposes Local Development Stack service-definition roots, profile roots, local state roots, generated env example paths, port-binding source refs, and safe reset markers as development/test metadata.
- `harness_discovery_metadata_defined`: `overrid.workspace.toml` exposes Integration Test Harness scenario roots, fixture roots, artifact roots, schema refs, local-stack command refs, and test targets without bespoke per-service scripts.
- `clean_checkout_ci_defined`: clean-checkout Linux validation runs `layout:check`, `schema:check`, `docs:check`, unit tests, local-stack smoke/status, and harness smoke through canonical root commands or documented Cargo workspace invocations with `pass`, `fail`, and `blocked` statuses.
- `validation_evidence_defined`: progress evidence records link checks, phase headings, work-item structure, stale-note scans, rejected-assumption scans, Docdex indexing checks, and queue/progress updates.
- `validation_artifact_consumers_defined`: layout artifacts are consumed by CI, local developer commands, Docdex, and future agents only as build/CI evidence, not Overwatch runtime events.
- `phase9_validation_defined`: `scripts/validate_repository_layout_phase9.py` validates Phase 9 docs, manifest foundation-integration metadata, Rust CLI layout-check evidence, planning trail, suite registration, and local Markdown links.

## Validation

- Fresh checkout has all expected directories.
- Root commands can discover packages and tests.
- New service stubs can be added without inventing a new layout.
- `scripts/validate_repository_layout_phase1.py` verifies Phase 1 attachment, boundary, master-phase, resolved-decision, governance, and Markdown-link evidence.
- `scripts/validate_repository_layout_phase2.py` verifies Phase 2 directory contracts, ownership READMEs, ignored local/generated markers, planning evidence, and Markdown-link evidence.
- `scripts/validate_repository_layout_phase3.py` verifies the root manifest, module records, inventory discovery, deterministic drift fixtures, planning evidence, and Markdown-link evidence.
- `scripts/validate_repository_layout_phase4.py` verifies schema authority, generated/projection boundaries, optional Protobuf placement, service-contract template content, reason-code/event placement, and phase planning evidence.
- `scripts/validate_repository_layout_phase5.py` verifies root command registry metadata, Rust-owned CLI wiring, `layout:check` output, validation artifact schema, command orchestration records, and Phase 5 planning evidence.
- `scripts/validate_repository_layout_phase6.py` verifies dependency direction groups, shared-schema dependency paths, modular control-plane shape, split-review criteria, local/test-only separation, and Phase 6 planning evidence.
- `scripts/validate_repository_layout_phase7.py` verifies generated-output ignores, local-state markers, secret-file rules, Docdex indexing hygiene, artifact-redaction expectations, and Phase 7 planning evidence.
- `scripts/validate_repository_layout_phase8.py` verifies service contract template usage, new-module checklist, module lifecycle, deprecation/removal workflow, cross-document maintenance rules, and Phase 8 planning evidence.
- `scripts/validate_repository_layout_phase9.py` verifies local-stack discovery metadata, harness discovery metadata, clean-checkout CI behavior, validation evidence, validation artifact consumers, and Phase 9 planning evidence.

## Handoff

This enables the shared schema package, local development stack, SDK, CLI, and first control-plane modules.
