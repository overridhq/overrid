SDS #5

# Repository Layout SDS

## Purpose

Define the physical workspace for Overrid services, packages, SDKs, CLI tools, specs, tests, and local infrastructure so early development does not sprawl.

This SDS treats repository layout as a governance and build-contract artifact, not a runtime service. Its job is to keep Phase 0 modular, inspectable, and locally runnable while preventing premature microservice fragmentation and hidden cross-package coupling.

## Source Documents

| Source | Path |
| --- | --- |
| Sub-build plan | [sub_build_plan_005_repository_layout.md](../../build_plan/sub_build_plan_005_repository_layout.md) |
| Service implementation plan | [repository_layout.md](../../service_catalog/foundation/repository_layout.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 0: Foundation](../../build_plan/phase_00_foundation.md) |

## Service Family

- Family: Foundation and developer tooling.
- Owning layer: Workspace structure, module boundaries, and build/test conventions.
- Primary data scope: directory map, package boundaries, service contract templates, dependency rules, root commands, documentation placement, and test layout.
- First build phase from service plan: [Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Problem Statement

Overrid has many planned components: control plane, node agent, shared schemas, SDK, CLI, local infrastructure, integration tests, data/storage services, accounting, policy, native apps, mobile, and adapters. If these begin as unstructured files or many premature microservices, the system will become impossible to run locally and hard to reason about. The repository layout must make ownership, contracts, and test paths obvious before service logic grows.

## Goals

- Define a concrete Phase 0 folder structure.
- Keep the first implementation modular but not over-split.
- Make shared schemas the required dependency path for service boundaries.
- Separate runtime services, shared packages, infrastructure, tests, specs, SDS docs, build plans, and generated artifacts.
- Provide root commands that can discover packages and tests without bespoke scripts per service.
- Make it obvious how to add a new service without bypassing schemas, API conventions, or documentation rules.

## Non-Goals

- Do not decide the final deployment topology for every future service.
- Do not force one programming language, framework, or package manager across all future components unless implementation later proves it necessary.
- Do not create empty folders for every whitepaper concept before there is a build phase need.
- Do not allow runtime code to depend on documentation files as executable configuration.
- Do not commit secrets, generated local state, build artifacts, package caches, or fixture outputs.
- Do not encode pricing, revenue forecasts, blockchain assumptions, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- Developers adding Phase 0 foundation code.
- Service implementers adding control-plane, node-agent, scheduling, accounting, policy, adapter, native app, or mobile modules.
- Integration Test Harness discovering scenarios and fixtures.
- Local Development Stack discovering service definitions.
- SDK and CLI builders consuming shared schemas.
- Docdex indexing and future agents that need predictable repo navigation.

## Dependencies

- Current whitepaper and build plan.
- Phase 0 decision that the first implementation is modular, not many premature microservices.
- Shared schema package requirements.
- Local development and test conventions.
- Documentation rules for service contracts, implementation plans, and SDS files.

This layout should be revised only through explicit documentation updates when a phase proves a new boundary is needed.

## Owned Responsibilities

Repository Layout is responsible for:

- Naming top-level directories and their allowed responsibilities.
- Defining package/service dependency direction.
- Defining where service contracts, schemas, tests, infrastructure, and docs live.
- Providing a service/module addition checklist.
- Defining root command names for build, test, local stack, lint/schema checks, and docs validation.
- Defining generated-file and secret-file exclusion rules.
- Making Docdex indexing and human navigation predictable.

It does not own runtime behavior, API semantics, service storage, or deployment orchestration.

## Data Model

The first implementation should define:

- `workspace_manifest`: root-level inventory of services, packages, docs, test roots, local infra, and generated-output locations.
- `module_record`: name, type, owner, path, phase, public contract, dependencies, test targets, and documentation links.
- `service_contract_stub`: purpose, owned data, public API, events emitted, events consumed, security boundary, operational checks, and test expectations.
- `package_boundary_rule`: allowed import direction and forbidden dependencies.
- `root_command_registry`: canonical names for build, test, integration test, local stack, schema check, docs check, and formatting.
- `generated_artifact_rule`: paths that must not be committed and should be ignored by Docdex/indexing when appropriate.
- `secret_file_rule`: approved names for examples versus local secret-bearing files.
- `new_service_checklist`: required docs, schema entries, tests, local-stack wiring, and crosswalk updates before a new module is accepted.

These records can start as Markdown and lightweight machine-readable manifests. They should not become runtime service configuration until a later phase explicitly designs that path.

## API Surface

The repository layout surface is a convention and validation API, not a network API.

Required directory contracts:

- `services/control-plane`: initial modular API and worker process.
- `services/node-agent`: Overcell agent and node simulator code.
- `packages/schemas`: shared request, command, manifest, event, audit, error, and view-model schemas.
- `packages/sdk`: client SDK generated or built from stable API contracts.
- `packages/cli`: developer and operator CLI.
- `infra/local`: Overrid-shaped local durable state, durable job table, object/artifact stub, service definitions, and development profiles.
- `tests/integration`: cross-service tests and scenario manifests.
- `docs/specs`: protocol, schema, and service contract documents.
- `docs/build_plan`: phase plans, crosswalks, and progress records.
- `docs/service_catalog`: per-service implementation plans.
- `docs/sds`: master and per-service design specifications.

## Phase 2 Workspace Shape Decisions

Phase 2 turns the required directory contracts into source-controlled workspace-shape evidence without adding runtime service behavior.

Gate states:

- `top_level_contracts_scaffolded`: `services`, `packages`, `infra`, `tests`, `docs/specs`, `docs/build_plan`, `docs/service_catalog`, and `docs/sds` exist as predictable top-level contracts.
- `service_path_rules_defined`: `services/control-plane` owns the initial modular Rust control-plane process boundary and `services/node-agent` owns the future Overcell node-agent/simulator boundary.
- `package_path_rules_defined`: `packages/schemas`, `packages/sdk`, and `packages/cli` have explicit ownership rules, with schemas as contract authority and generated code as consumer output.
- `package_path_rules_defined`: implemented companion roots such as `packages/admin_ui_shell`, `packages/integration_harness`, and `packages/local_stack` also carry package-level README ownership metadata, source SDS/build-plan refs, and test-target declarations without becoming core runtime authority.
- `local_infra_test_paths_defined`: `infra/local` and `tests/integration` separate source-controlled local/test contracts from ignored state, job-table, artifact, and run-output paths.
- `specs_contract_defined`: `docs/specs` owns protocol, schema, API, service-contract, reason-code, event-contract, audit-record, and validation-artifact docs, while `docs/specs/generated` is ignored generated output.

Phase 2 does not introduce Cargo service crates, runtime endpoints, production configuration loading from docs, deployment orchestration, or hidden discovery. It records path ownership so later phases can add Rust code, specs, schemas, tests, and local-stack wiring through layout-change governance.

## Phase 3 Workspace Manifest Decisions

Phase 3 defines `overrid.workspace.toml` as the root workspace manifest for validation and build discovery. The manifest is not runtime configuration, hidden service discovery, deployment orchestration, or production configuration.

Gate states:

- `workspace_manifest_defined`: the root manifest records schema version, manifest version, source document links, validation metadata, module-record schema enums, workspace inventory roots, and drift reason codes.
- `module_records_defined`: every implemented Phase 0 module has a `module_record` with `name`, `type`, `owner_layer`, `path`, `master_phase`, `public_contract_path`, `allowed_dependency_groups`, `generated_output_paths`, `test_targets`, `local_stack_participation`, and `documentation_links`.
- `inventory_discovery_defined`: layout checks compare manifest records against `Cargo.toml` workspace members, direct `packages/` roots, local profile roots, service-definition roots, test roots, specs roots, and generated-output ignore markers.
- `phase_owner_metadata_defined`: each module record uses accepted type, owner-layer, phase, dependency-group, lifecycle-state, and local-stack participation metadata; later phase promotions require matching SDS, service plan, phase, and crosswalk evidence.
- `manifest_drift_checks_defined`: manifest checks produce deterministic reason codes, including `missing_schema_version`, `missing_module_records`, `missing_required_field`, `duplicate_module_name`, `invalid_path`, `missing_path`, `unknown_module_type`, `unknown_owner_layer`, `invalid_master_phase`, `unknown_dependency_group`, `missing_public_contract`, `stale_documentation_link`, `missing_test_target`, `forbidden_generated_path`, `missing_local_stack_participation`, `cargo_member_drift`, and `unlisted_module`.

Initial Phase 3 records cover `packages/schemas`, `packages/sdk`, `packages/cli`, `packages/local_stack`, `packages/integration_harness`, `packages/admin_ui_shell`, `infra/local`, `tests/integration`, `docs/specs`, `services/control-plane`, and `services/node-agent`. Scaffolded service roots remain layout contracts until later phases add runtime code through layout-change governance.

Required command contracts:

## Phase 4 Contracts And Schema Authority Decisions

Phase 4 makes contract source-of-truth paths explicit before service logic grows. It does not add runtime services, executable configuration from docs, hidden discovery, or production topology.

Gate states:

- `schema_authority_defined`: `packages/schemas` owns canonical JSON Schema sources for commands, manifests, fixtures, signed payloads, view models, events, audit records, errors, and docs-facing examples; generated/projection code remains non-authoritative.
- `generated_binding_boundaries_defined`: Rust and TypeScript/web bindings must cite canonical schema paths, generated/projection output paths, source-of-truth status, non-authoritative status, and validation targets before consumers rely on them.
- `protobuf_placement_defined`: Protobuf may be added only for compact internal service/RPC/event contracts with owning SDS and `docs/specs` justification; it cannot replace JSON Schema for docs-facing or signed command contracts.
- `service_contract_template_defined`: implemented service/module contracts must include purpose, owned data, public API, events emitted, events consumed, security boundary, operational checks, test expectations, schema refs, and owning phase.
- `reason_event_contracts_defined`: reason-code families, event envelopes, audit records, validation artifacts, and error shapes remain discoverable from `docs/specs/reason_codes_and_events.md` and `packages/schemas`.

Required Phase 4 source contracts:

- `docs/specs/contract_authority.md`
- `docs/specs/service_contract_template.md`
- `docs/specs/reason_codes_and_events.md`
- `packages/schemas/README.md`
- `overrid.workspace.toml`

- `build`: compile/check all implemented packages.
- `test`: run unit and fast package tests.
- `test:integration`: run bounded local integration scenarios.
- `dev:start`, `dev:stop`, `dev:reset`, `dev:seed`, `dev:status`: local stack lifecycle commands.
- `schema:check`: validate shared schemas and fixtures.
- `docs:check`: validate markdown links, headings, and stale-note/revenue scans.
- `layout:check`: verify expected directories, manifests, package boundaries, and generated-file exclusions.

Command names may be exposed through a task runner, package scripts, make targets, or CLI shims, but the semantic names should remain stable.

## Phase 5 Root Command Registry And Layout Check Decisions

Phase 5 defines the Rust-owned command registry and `layout:check` validation surface. It does not make Repository Layout a network service, runtime registry, deployment orchestrator, production configuration source, or hidden service discovery path.

Gate states:

- `root_command_registry_defined`: `overrid.workspace.toml` and the Rust `overrid` CLI record `build`, `test`, `test:integration`, `dev:start`, `dev:stop`, `dev:reset`, `dev:seed`, `dev:status`, `schema:check`, `docs:check`, and `layout:check` with purpose, inputs, outputs, owner, canonical invocation, machine-readable result envelope, failure classes, phase gate, and alias metadata.
- `rust_owned_command_execution_defined`: `packages/cli` owns `overrid command-registry` and `overrid layout:check`; shell, Make, just, npm, and CI aliases may only be thin wrappers around the Rust-owned behavior.
- `layout_check_defined`: `layout:check` validates required directories, workspace manifest records, module-record/test-target markers, service contract stubs, generated-output ignore markers, secret-file absence, package-boundary metadata, local-state markers, and docs contract paths.
- `schema_docs_check_orchestration_defined`: `schema:check`, `docs:check`, and `layout:check` remain local and CI-runnable semantic records with stable pass/fail/block output, not runtime Overwatch events.
- `validation_artifacts_defined`: layout validation artifacts use `layout_check.passed`, `layout_check.failed`, `package_boundary_violation`, `missing_service_contract`, `missing_test_target`, `generated_file_committed`, and `secret_file_committed`.

Phase 5 validation records must include reason code, path, owning phase, module id when available, and artifact refs. Secret-file findings must never include raw secret values.

## Phase 6 Package Boundary And Control-Plane Decisions

Phase 6 turns package-boundary intent into validation metadata. It does not add runtime services, production configuration, deployment topology, or hidden service discovery.

Gate states:

- `dependency_direction_groups_defined`: `overrid.workspace.toml` records dependency direction groups for contracts, SDK, CLI, local stack, integration harness, admin UI shell, docs, local infrastructure, control-plane modules, node-agent modules, and docs/specs helpers.
- `shared_schema_dependency_paths_enforced`: boundary objects for service APIs, events, commands, fixtures, read models, audit records, and errors must flow through `packages/schemas`, `overrid-contracts`, or documented `docs/specs` sources before consumers rely on them.
- `modular_control_plane_shape_preserved`: `services/control-plane` remains one modular Rust process through master Phase 3 by default, with internal modules or crates for control-plane domains rather than premature deployable microservices.
- `split_review_criteria_defined`: future service splits require measured API-load, failure-isolation, security-boundary, operational, or grid-resident backbone pressure plus updated SDS, service catalog, build-plan crosswalk, docs/specs contracts, and validation evidence.
- `local_test_only_separation_enforced`: runtime-facing modules must not import integration harness internals, local-stack internals, `infra/local`, fixture writers, integration artifacts, local simulator internals, or docs files as executable configuration.

Phase 6 validation artifacts include `package_boundary_violation`, `schema_ref_missing`, `premature_service_split`, `split_review_missing`, and `local_test_boundary_violation`. These are CI/build artifacts, not Overwatch runtime events.

## Phase 7 Artifact Hygiene And Indexing Decisions

Phase 7 turns generated/local/secret ignore intent into validation metadata. It does not add runtime services, production configuration, deployment topology, hidden service discovery, or external storage product boundaries.

Gate states:

- `generated_output_ignore_rules_defined`: `.gitignore`, `.docdexignore`, and `overrid.workspace.toml` record generated output, dependency cache, coverage, log, generated spec/type/doc, fixture-output, integration artifact, and temporary object-chunk paths.
- `local_state_ignore_rules_defined`: `.overrid`, `infra/local/state`, `infra/local/job-tables`, `infra/local/artifacts`, and `tests/integration/artifacts` are local/test-only, resettable, marker-gated paths and must not become production state.
- `secret_file_rules_defined`: source control may contain example files only; secret-like environment, local, key, token, private-key, and fixture-credential paths are rejected without exposing raw values.
- `docdex_indexing_hygiene_defined`: docs, specs, SDS, build plans, service catalog files, source schemas, handwritten fixtures, and service contract stubs remain indexable while generated artifacts and local caches are excluded.
- `artifact_redaction_expectations_defined`: layout validation artifacts, docs checks, CI bundles, local-stack exports, and harness artifacts must redact secrets, keys, tokens, signatures, private payloads, encrypted content, and local fixture credentials.

Phase 7 validation artifacts include `generated_file_committed`, `secret_file_committed`, `local_state_committed`, `docdex_index_hygiene_violation`, and `artifact_redaction_violation`.

## Phase 8 Service Contract And Lifecycle Decisions

Phase 8 turns new-module checklist and lifecycle intent into validation metadata. It does not add runtime service discovery, deployment orchestration, production configuration, or new top-level sprawl.

Gate states:

- `service_contract_template_implemented`: `docs/specs/service_contract_template.md` includes the required service/module sections plus usage notes and downstream dependency expectations.
- `new_service_checklist_defined`: `docs/specs/new_module_checklist.md` records required SDS, service catalog, build-plan/crosswalk, docs/specs, schema, test, local-stack, generated-output, and module-record evidence before acceptance.
- `module_addition_workflow_defined`: module records use `proposed`, `scaffolded`, `contracted`, `wired`, `validated`, and `accepted` lifecycle states.
- `deprecation_removal_workflow_defined`: deprecated and removed modules require replacement/removal notes, stale-reference cleanup, manifest updates, tests, local-stack, harness, generated-output, and Docdex reference updates.
- `cross_document_maintenance_rules_defined`: SDS, service catalog, sub-build plan, master plan, crosswalk, specs, manifest, and phase planning/progress evidence remain aligned when layout rules or service boundaries change.

Phase 8 validation artifacts include `missing_service_contract`, `missing_test_target`, `module_lifecycle_violation`, and `stale_layout_reference`.

## Phase 9 Foundation Integration Decisions

Phase 9 exposes repository-layout metadata to the Local Development Stack, Integration Test Harness, CI, Docdex, and future agents without turning Repository Layout into runtime service discovery, production configuration, deployment orchestration, or Overwatch event ownership.

Gate states:

- `local_stack_discovery_metadata_defined`: `overrid.workspace.toml` records Local Development Stack service-definition roots, profile roots, local state roots, generated env example paths, port-binding source refs, and safe reset markers.
- `harness_discovery_metadata_defined`: `overrid.workspace.toml` records Integration Test Harness scenario roots, fixture roots, artifact roots, schema refs, local-stack command refs, and test targets.
- `clean_checkout_ci_defined`: clean-checkout Linux CI uses canonical root commands for layout, schema, docs, unit, local-stack, and harness checks with `pass`, `fail`, and `blocked` result states.
- `validation_evidence_defined`: docs and layout validation evidence records link checks, phase headings, work-item structure, stale-note scans, rejected-assumption scans, Docdex indexing checks, and queue/progress updates.
- `validation_artifact_consumers_defined`: layout artifacts are consumed by CI, local developer commands, Docdex, and future agents only as build/CI evidence.

Phase 9 validation artifacts include `local_stack_discovery_violation`, `harness_discovery_violation`, `ci_command_sequence_violation`, `validation_evidence_missing`, and `artifact_consumer_violation`.

## Phase 10 Alignment And Handoff Decisions

Phase 10 closes Repository Layout by validating the source-document chain, tech-stack constraints, master-plan placement, SDS/service-catalog references, and downstream expansion rules. It does not add runtime service behavior, hidden discovery, production configuration, deployment orchestration, or new top-level sprawl.

Gate states:

- `sub_build_plan_structure_validated`: SUB BUILD PLAN #5 keeps its title prefix, attached SDS link, Phase 1 through Phase 10 headings, numbered work items, Design/Output/Validation fields, local Markdown links, Alignment Review, and Exit Gate.
- `tech_stack_alignment_validated`: repository layout remains a Rust-first Cargo workspace with language-neutral JSON/JSON Schema authority, Overrid-shaped local primitives, optional Protobuf only for justified compact internal contracts, generated binding boundaries, and no conventional cloud runtime authority.
- `master_plan_alignment_validated`: master Phase 0 through Phase 13 order remains unchanged, SDS #5 remains a Phase 0 foundation sub-build plan, and later phase responsibilities remain consumers of the established layout.
- `service_catalog_sds_alignment_validated`: SDS #5, the Repository Layout service plan, SUB BUILD PLAN #5, the master plan row, the service-catalog crosswalk row, and the tech-stack decision remain mutually linked and aligned.
- `downstream_phase_handoff_defined`: Phases 1 through 13 add modules, contracts, schemas, tests, local-stack profiles, generated outputs, and service boundaries inside the existing structure unless SDS-backed phase evidence justifies expansion.

Phase 10 validation artifacts include `sub_build_plan_structure_violation`, `tech_stack_alignment_violation`, `master_plan_alignment_violation`, `source_document_alignment_violation`, and `downstream_handoff_violation`.

The downstream handoff boundary is `existing_layout_with_sds_backed_expansion_no_top_level_sprawl`: later builders extend the manifest-backed, schema-governed layout through lifecycle evidence rather than inventing undocumented top-level roots.

## Event Surface

Repository layout does not emit runtime platform events.

It should produce validation artifacts:

- `layout_check.passed`
- `layout_check.failed`
- `package_boundary_violation`
- `missing_service_contract`
- `missing_test_target`
- `generated_file_committed`
- `secret_file_committed`
- `schema_ref_missing`
- `premature_service_split`
- `split_review_missing`
- `local_test_boundary_violation`
- `local_state_committed`
- `docdex_index_hygiene_violation`
- `artifact_redaction_violation`
- `module_lifecycle_violation`
- `stale_layout_reference`
- `local_stack_discovery_violation`
- `harness_discovery_violation`
- `ci_command_sequence_violation`
- `validation_evidence_missing`
- `artifact_consumer_violation`
- `sub_build_plan_structure_violation`
- `tech_stack_alignment_violation`
- `master_plan_alignment_violation`
- `source_document_alignment_violation`
- `downstream_handoff_violation`

These are CI/build artifacts, not Overwatch events. Integration tests may still exercise Overwatch through running services.

## Core Workflow

1. Create the Phase 0 folder structure.
2. Add root workspace manifest and package/module records.
3. Add shared schema package before service logic.
4. Add local development stack definitions.
5. Add integration test harness directories and first scenario.
6. Add service contract stubs for initial subsystems.
7. Add root command registry and layout check.
8. Document the new-service checklist.
9. Keep later service additions aligned with phase docs and the build-plan/service crosswalk.
10. Validate source-document alignment and hand downstream phases a bounded expansion path.

## State Machine

Layout change lifecycle:

1. `proposed`: new directory, package, or boundary is described in docs.
2. `scaffolded`: folder and manifest entries exist.
3. `contracted`: service/package contract and SDS links exist.
4. `wired`: root commands, local stack, or test harness can discover the module.
5. `validated`: layout checks and relevant tests pass.
6. `accepted`: build plan, service catalog, SDS, and crosswalk are aligned.
7. `deprecated`: boundary is intentionally being replaced.
8. `removed`: old boundary is deleted after references are updated.

No runtime service state should depend on this lifecycle.

## Phase-Gate Boundary Decisions

SDS #5 starts in [Phase 0: Foundation](../../build_plan/phase_00_foundation.md) and is tracked by [SUB BUILD PLAN #5 - Repository Layout](../../build_plan/sub_build_plan_005_repository_layout.md). Phase 1 freezes the attachment, boundary, and governance gates before any layout command or manifest implementation starts.

Repository Layout is a governance/build-contract/validation artifact, not a network service, not a runtime service registry, not a deployment orchestrator, not a configuration database, not a production configuration source, not hidden service discovery, not an external cloud account boundary, not a service-storage shortcut, and not a shortcut around service contracts.

Phase-gate states:

- `attached`: SDS, service catalog, master plan, build-plan crosswalk, and phase planning docs link to the SDS #5 sub-build plan.
- `boundary_frozen`: later implementation must preserve the non-runtime Repository Layout boundary.
- `master_phase_0_owned`: Phase 0 owns the initial workspace shape, command names, schema/source paths, generated/local ignore rules, and validation contracts.
- `later_phase_consumer`: Phases 1 through 13 consume the established layout and may expand it only when their SDS and phase docs justify new boundaries.
- `resolved_decision_carried`: later implementation must preserve Rust-owned command registry, modular control-plane through Phase 3, language-neutral `packages/schemas` authority, generated/local ignore rules, and minimal `overrid.workspace.toml` manifest.
- `governance_required`: every new top-level directory, package group, generated path, service boundary, deprecation, or removal must move through `proposed`, `scaffolded`, `contracted`, `wired`, `validated`, `accepted`, `deprecated`, and `removed` states as applicable.

Resolved Phase 1 decisions:

- Rust-owned command registry is canonical; wrappers may only call the same semantic command paths.
- The modular control-plane process through Phase 3 remains the default until measured operational/security pressure justifies a split.
- Language-neutral `packages/schemas` authority owns canonical JSON Schema contracts, with Protobuf allowed only for justified compact internal contracts.
- Generated/local ignore rules keep build output, local state, caches, fixture outputs, temporary object chunks, generated bindings, and secret-bearing files out of source control and Docdex indexing where appropriate.
- Minimal `overrid.workspace.toml` manifest records module inventory and validation metadata only; it must not become runtime configuration.

## Policy And Security

- Commit only examples for secret-bearing files, never real secrets.
- Keep local state, generated fixtures, logs, caches, coverage, build output, and package artifacts out of source control.
- Keep test-only fixtures and production-like config clearly separated.
- Enforce package dependency direction so runtime services cannot import test harness or local-only modules.
- Require shared schemas for service boundary objects.
- Avoid direct cross-service storage imports; services communicate through contracts.
- Keep docs and code paths predictable for Docdex indexing.
- Use explicit deprecation/removal steps for layout changes to avoid broken references.

## Metering And Accounting

Repository layout is not metered as a runtime service. It still supports accounting indirectly:

- Reserve clear locations for Overmeter, ORU, Seal Ledger, Overbill, Overgrant, Overasset, and accounting fixtures when those phases arrive.
- Ensure test fixture usage cannot be mistaken for real billable usage.
- Keep docs and schemas free of revenue projections and pricing assumptions.
- Require accounting-facing services to define usage events and ledger refs in their own SDS files.

## Observability And Operations

The layout should make these checks easy:

- Expected directories exist.
- Workspace manifest lists implemented modules.
- Package boundary rules pass.
- Root commands discover packages and tests.
- Service contract stubs exist for implemented services.
- Generated and secret-bearing files are ignored.
- Docdex can index docs/specs/build plans/SDS without large generated artifacts.
- New service checklist is complete before implementation starts.

## Failure Modes And Recovery

- Missing directory: layout check fails with expected path and owning phase.
- New module without contract: fail docs/layout check before implementation grows.
- Forbidden dependency: fail package boundary check with importer/imported paths.
- Runtime code imports test/local-only package: fail boundary check.
- Secret-like file committed: fail docs/security check and require removal.
- Generated artifact committed: fail layout check and add ignore rule.
- Crosswalk/doc mismatch: fail docs check and update service catalog/SDS/build plan together.
- Premature service split: require design review against Phase 0 modular-but-not-over-split rule.

## Validation Plan

Service-plan validation:

- Fresh checkout has all expected directories.
- Root commands can discover packages and tests.
- New service stubs can be added without inventing a new layout.

Additional SDS-level validation:

- `layout:check` verifies the required Phase 0 directory contracts.
- `docs:check` verifies service contract stubs, service catalog docs, SDS docs, and crosswalk links.
- `schema:check` proves shared schemas exist before boundary services use them.
- Root commands can find package tests and integration scenarios.
- Generated/local/secret files are excluded.
- Dependency-boundary checks prevent runtime imports from test harness/local-only modules.
- Adding a sample service through the checklist updates docs, schemas, tests, and crosswalk without inventing new top-level folders.

## Build Breakdown

1. Create required Phase 0 top-level directories.
2. Add `workspace_manifest` and module records for initial services/packages.
3. Add shared schema package directory and schema-check command.
4. Add local infrastructure directory and local-stack command names.
5. Add integration test directory and scenario convention.
6. Add service contract template under `docs/specs`.
7. Add docs/check and layout/check commands.
8. Add generated-file and secret-file ignore rules.
9. Add new-service checklist and update master docs with layout rules.

## Handoff And Downstream Use

This layout enables the shared schema package, local development stack, integration test harness, SDK, CLI, and first control-plane modules. Every later phase should add code inside the existing structure unless a phase document and SDS update justify a new boundary.

When implementation starts, repository layout should be validated before Phase 1 control-plane code is accepted.

## Open Design Questions

Resolved decisions:

- Semantic root commands should be exposed through a Rust-owned command registry executed by the Cargo workspace and the `overrid` CLI, with thin optional Make, just, or npm shims allowed only as aliases. The canonical contract is the command semantics, such as `build`, `test`, `test:integration`, `dev:start`, `dev:stop`, `dev:reset`, `dev:seed`, `dev:status`, `schema:check`, `docs:check`, and `layout:check`, not a specific shell wrapper. Phase 0 should implement the registry as workspace metadata plus Rust CLI subcommands or `cargo xtask`-style tooling so Linux CI, local development, and later SDK/UI work call the same validation paths.
- `services/control-plane` should remain one Rust modular process through Phase 3 by default. Phase 1 through Phase 3 modules for Overgate, Overtenant, Overpass-lite, Overkey-lite, Overregistry, Overwatch, Overqueue, scheduler handoff, leases, and metering interfaces should be separate crates/modules and contracts inside the process, not prematurely separate deployable services. Splitting becomes a documented Phase 4+ or Phase 7 decision only after measured API load, failure-isolation, security-boundary, or operational pressure proves an independent boundary; any split must preserve shared schemas, signed command envelopes, idempotency, trace ids, stable reason codes, and Overwatch audit behavior.
- Schema source of truth should be language-neutral contract files in `packages/schemas`, not Rust or TypeScript source files. Canonical JSON Schema should own human-readable commands, manifests, fixtures, signed payloads, view models, and docs-facing examples; Protobuf can be added for compact internal service/RPC/event contracts where needed. Rust and TypeScript SDK bindings should be generated or validated from those contracts, with Rust-first runtime support and compatibility tests preventing generated code from becoming the authority.
- Generated and local-only paths should be ignored by explicit rules in `.gitignore`, `.docdexignore` when present, and the workspace manifest. Phase 0 should ignore `target/`, `node_modules/`, package caches, coverage, logs, local state under `.overrid/` or `infra/local/state/`, generated SDK/types/docs under `packages/*/generated/` and `docs/specs/generated/`, integration artifacts under `tests/integration/artifacts/`, fixture outputs under `tests/fixtures/generated/`, temporary Overstore/object chunks under `infra/local/overstore/`, and any `*.local.*`, `*.secret.*`, `*.key`, or secret-bearing environment files. Source schemas, handwritten fixtures, examples, SDS/build-plan docs, and service contract stubs remain indexed and versioned.
- The smallest useful machine-readable workspace manifest should be one root `overrid.workspace.toml` or equivalent stable JSON/TOML file if implementation standardizes another format. It should contain only module inventory and validation metadata: schema version, module name, module type, path, phase, owner layer, public contract path, allowed dependency groups, generated-output paths, test targets, local-stack participation, and documentation links. It should not duplicate Cargo manifests or become runtime configuration; layout checks consume it to detect missing contracts, forbidden dependencies, stale documentation links, and generated/secret path violations.
