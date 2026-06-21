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
4. Add placeholder service contract docs for each initial subsystem.
5. Add build/test command conventions and a root task runner.
6. Document how new services are added without bypassing shared schemas.

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

## Validation

- Fresh checkout has all expected directories.
- Root commands can discover packages and tests.
- New service stubs can be added without inventing a new layout.
- `scripts/validate_repository_layout_phase1.py` verifies Phase 1 attachment, boundary, master-phase, resolved-decision, governance, and Markdown-link evidence.
- `scripts/validate_repository_layout_phase2.py` verifies Phase 2 directory contracts, ownership READMEs, ignored local/generated markers, planning evidence, and Markdown-link evidence.

## Handoff

This enables the shared schema package, local development stack, SDK, CLI, and first control-plane modules.
