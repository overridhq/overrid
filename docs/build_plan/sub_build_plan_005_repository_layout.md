# SUB BUILD PLAN #5 - Repository Layout

Attached SDS: [docs/sds/foundation/repository_layout.md](../sds/foundation/repository_layout.md)

## Purpose

This sub-build plan turns SDS #5 into an implementation sequence for the Repository Layout foundation artifact. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Repository Layout is a governance and build-contract artifact, not a runtime service. It must make the Phase 0 workspace concrete, keep the first implementation modular without premature service sprawl, define how shared schemas and service contracts gate new modules, and give developers, CI, Docdex, the Local Development Stack, and the Integration Test Harness predictable paths to discover code, specs, tests, and documentation.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #5: Repository Layout](../sds/foundation/repository_layout.md) | Controls repository-layout purpose, directory contracts, data records, command semantics, state lifecycle, validation artifacts, security rules, and resolved open-question decisions. |
| [Repository Layout service plan](../service_catalog/foundation/repository_layout.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Controls the first implementation point for repository structure, shared schemas, local development, integration tests, and documentation rules. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps Repository Layout aligned to master Phase 0 as the first build phase. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires a Rust-first Cargo workspace, language-neutral JSON/JSON Schema contracts, optional Protobuf where useful, and Overrid-shaped primitives instead of conventional cloud product boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 0 | Attach SDS #5 to the build-plan layer and freeze layout boundaries before implementation. |
| 2 | Master Phase 0 | Create the top-level workspace shape and directory contracts that every later phase uses. |
| 3 | Master Phase 0 | Define `workspace_manifest` and `module_record` inventory so layout checks can reason about modules. |
| 4 | Master Phase 0 | Establish shared-schema, docs/specs, and contract-source-of-truth paths before service logic grows. |
| 5 | Master Phase 0 | Implement the Rust-owned root command registry and `layout:check` semantics. |
| 6 | Master Phase 0 with Phase 1 through 3 guardrails | Enforce package boundaries and keep the initial control plane modular inside one Rust process. |
| 7 | Master Phase 0 | Enforce generated-artifact, secret-file, local-state, and Docdex indexing hygiene. |
| 8 | Master Phase 0 with handoff to all later phases | Define service contract templates, new-module checklist, and layout lifecycle rules. |
| 9 | Master Phase 0 with handoff to SDS #3 and SDS #4 | Wire layout discovery into the Local Development Stack, Integration Test Harness, CI, and validation artifacts. |
| 10 | Master Phase 0 with handoff to Phases 1 through 13 | Validate alignment, update documentation references, and define downstream expansion rules. |

## Tech Stack Guardrails

- The authoritative workspace is a Rust-first Cargo workspace with service crates, shared contract crates, test utilities, CLI/local tooling, and optional web/client packages only where appropriate.
- Semantic root commands must be Rust-owned through the `overrid` CLI, Cargo workspace metadata, or `cargo xtask`-style tooling; Make, just, npm, or shell wrappers may exist only as thin aliases.
- `packages/schemas` owns language-neutral contract files. Canonical JSON Schema owns human-readable commands, manifests, fixtures, signed payloads, view models, and docs-facing examples; Protobuf may be added for compact internal service/RPC/event contracts.
- Generated Rust and TypeScript SDK bindings must be generated or validated from contracts. Generated code is not the source of truth.
- `services/control-plane` remains one modular Rust process through master Phase 3 by default, with separate crates/modules and contracts for internal domains instead of premature deployable microservices.
- `infra/local` must describe Overrid-shaped local durable state, durable job tables, object/artifact stubs, service definitions, and profiles, not PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, or other product boundaries.
- Generated output, local state, logs, caches, coverage, fixture outputs, temporary object chunks, and secret-bearing files must be ignored by source control and Docdex indexing where appropriate.
- Repository layout must not encode pricing, revenue forecasts, blockchain assumptions, NFT mechanics, customer-count targets, or per-transaction fee economics.

## Phase 1: SDS Attachment, Boundary, And Master-Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #5.**
  - Design: Link this document from the numbered SDS, service catalog entry, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/foundation/repository_layout.md`, `docs/service_catalog/foundation/repository_layout.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #5 returns both the SDS and this sub-build plan.

- **1.2 Freeze the repository-layout boundary.**
  - Design: Record that Repository Layout is a build-contract and validation artifact, not a network service, runtime service registry, deployment orchestrator, configuration database, or shortcut around service contracts.
  - Output: Boundary guardrails documented in this plan and referenced by later layout changes.
  - Validation: Architecture review confirms no work item requires runtime endpoints, production config loading from docs, hidden service discovery, external cloud accounts, or service-storage shortcuts.

- **1.3 Preserve master Phase 0 as the first build point.**
  - Design: Keep all initial layout work inside master Phase 0 while treating later service additions as consumers of the established structure.
  - Output: Phase-gate note that Repository Layout starts in Phase 0 and only expands when later phase docs and SDS files justify new boundaries.
  - Validation: Review proves this plan does not move Phase 1 through Phase 13 service responsibilities earlier than the master build plan.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve the Rust-owned command registry, modular control-plane process through Phase 3, language-neutral schema authority, generated/local ignore rules, and minimal `overrid.workspace.toml` or equivalent manifest.
  - Output: Implementation checklist tied to the SDS resolved decisions.
  - Validation: Review proves the plan does not re-open or contradict any resolved open design question in SDS #5.

- **1.5 Define layout-change governance.**
  - Design: Require every new top-level directory, package group, generated path, service boundary, or deprecation to move through proposed, scaffolded, contracted, wired, validated, accepted, deprecated, and removed states.
  - Output: Governance rule for layout changes and removals.
  - Validation: Review checklist rejects undocumented new top-level folders, uncontracted modules, and removed paths with stale references.

### Phase 1 Gate Outputs

#### Link Attachment Matrix

| Source | Required link | Gate state |
| --- | --- | --- |
| Numbered SDS | `docs/sds/foundation/repository_layout.md` links to `docs/build_plan/sub_build_plan_005_repository_layout.md`. | `attached` |
| Service catalog | `docs/service_catalog/foundation/repository_layout.md` links to this sub-build plan and the detailed SDS. | `attached` |
| Master build plan | `docs/build_plan/master_plan.md` lists SDS #5 and this sub-build plan in the per-SDS table. | `attached` |
| Build-plan crosswalk | `docs/build_plan/service_catalog_alignment.md` maps SDS #5 to Phase 0 and this sub-build plan. | `attached` |
| Tech stack decision | `docs/overrid_tech_stack_choice.md` remains the Rust-first/native-Overrid stack constraint for later layout implementation. | `attached` |
| Phase planning trail | `docs/planning/repository_layout_phase_01_plan.md` and `docs/planning/repository_layout_phase_01_progress.md` record implementation and validation evidence. | `attached` |

#### Frozen Repository-Layout Boundary

Repository Layout is a governance, build-contract, and validation artifact. It is not a network service, not a runtime service registry, not a deployment orchestrator, not a configuration database, not a production configuration source, not hidden service discovery, not an external cloud account boundary, not a service-storage shortcut, and not a shortcut around service contracts.

The `boundary_frozen` gate means later phases may consume or validate repository-layout records, but they must not move this artifact into runtime service behavior.

The allowed Phase 1 output is documented boundary evidence plus validation. Runtime folders, Rust crates, command execution, manifests, local state, and `layout:check` implementation start only in later Repository Layout phases.

#### Master Phase Gate Matrix

| Master phase | Repository Layout gate state | Rule |
| --- | --- | --- |
| 0 | `master_phase_0_owned` | Repository Layout first becomes buildable in Phase 0 and owns workspace shape, command names, schema/source paths, generated/local ignore rules, and validation contracts. |
| 1 | `later_phase_consumer` | Control-plane work consumes the Phase 0 layout and must not move layout governance into runtime services. |
| 2 | `later_phase_consumer` | Seed private swarm work consumes service/node-agent paths after owning SDS docs justify implementation. |
| 3 | `later_phase_consumer` | Private execution loop work consumes packages, tests, and specs without bypassing shared contracts. |
| 4 | `later_phase_consumer` | Trust/policy work may add modules only through layout-change governance. |
| 5 | `later_phase_consumer` | Accounting work may add modules only through layout-change governance and no pricing/revenue assumptions. |
| 6 | `later_phase_consumer` | Product integration work may add UI/client paths only as documented consumers of Overgate/admin contracts. |
| 7 | `later_phase_consumer` | Grid-resident work may justify service splits only with measured operational/security pressure. |
| 8 | `later_phase_consumer` | Storage/namespace work may add native primitives only through SDS-backed contracts. |
| 9 | `later_phase_consumer` | Deployment work consumes package/deployment specs without turning Repository Layout into an orchestrator. |
| 10 | `later_phase_consumer` | Federation/public-interest work consumes existing module and specs paths unless docs justify expansion. |
| 11 | `later_phase_consumer` | Public-provider work cannot add uncontracted public/test paths. |
| 12 | `later_phase_consumer` | Native app work adds client/app paths only after phase and SDS docs justify them. |
| 13 | `later_phase_consumer` | Governance hardening may deprecate or remove paths only through documented lifecycle states. |

#### Resolved SDS Decision Checklist

- `resolved_decision_carried`: Rust-owned command registry remains the canonical semantic command path; shell, Make, just, or npm wrappers may only be thin aliases.
- `resolved_decision_carried`: `services/control-plane` remains one modular Rust process through Phase 3 by default, with future splits requiring measured rationale and updated SDS/spec links.
- `resolved_decision_carried`: language-neutral `packages/schemas` authority owns canonical JSON Schema contracts, with optional Protobuf only for justified compact internal contracts.
- `resolved_decision_carried`: generated/local ignore rules keep build output, generated SDK/types/docs, fixture outputs, local state, logs, caches, temporary object chunks, and secret-bearing files out of source control and Docdex indexing where appropriate.
- `resolved_decision_carried`: the minimal `overrid.workspace.toml` manifest is module inventory and validation metadata, not runtime configuration.

#### Layout-Change Governance Lifecycle

Every new top-level directory, package group, generated path, service boundary, deprecation, or removal must move through `proposed`, `scaffolded`, `contracted`, `wired`, `validated`, `accepted`, `deprecated`, and `removed` states as applicable. The `governance_required` gate rejects undocumented top-level folders, uncontracted modules, generated paths without ignore rules, and removed paths with stale references.

#### Documentation Update Rule

Any accepted layout change must update the numbered SDS, service catalog entry, sub-build plan, master build plan or phase doc when phase placement changes, build-plan crosswalk, relevant docs/specs contract, and phase progress evidence before implementation is accepted.

## Phase 2: Top-Level Directory Contracts And Workspace Shape

### Work Items

- **2.1 Create the required Phase 0 top-level directories.**
  - Design: Scaffold `services`, `packages`, `infra`, `tests`, and `docs/specs` in the shape defined by SDS #5, while preserving existing `docs/build_plan`, `docs/service_catalog`, and `docs/sds` structure.
  - Output: Directory contracts for services, packages, local infrastructure, integration tests, specs, build plans, service catalog docs, and SDS docs.
  - Validation: `layout:check` verifies every required Phase 0 directory exists and reports missing paths with owning phase and remediation.

- **2.2 Define `services/control-plane` and `services/node-agent` ownership.**
  - Design: Establish `services/control-plane` for the initial modular API/worker process and `services/node-agent` for Overcell agent and simulator code, without splitting every future service into deployable folders early.
  - Output: Service path rules for control-plane modules, node-agent modules, internal crates, and future split candidates.
  - Validation: Review confirms control-plane domains can be separate Rust crates/modules while remaining one process through Phase 3 unless later measured pressure justifies a split.

- **2.3 Define package directories.**
  - Design: Establish `packages/schemas`, `packages/sdk`, and `packages/cli` with clear ownership for contracts, generated/built clients, and command-line tooling.
  - Output: Package path rules and expected README/manifest placeholders for implemented packages.
  - Validation: `layout:check` rejects package directories that lack ownership metadata, contract refs where required, or test-target declarations.

- **2.4 Define local infrastructure and test directories.**
  - Design: Establish `infra/local` for Overrid-shaped local state, job-table, artifact stub, service definitions, profiles, and safe local state markers; establish `tests/integration` for cross-service scenarios and artifacts.
  - Output: Local infrastructure and integration-test path rules aligned with SDS #3 and SDS #4.
  - Validation: Checks prove local state and test artifacts stay in ignored subpaths while source fixtures and scenario manifests remain versioned.

- **2.5 Define docs/specs placement.**
  - Design: Reserve `docs/specs` for protocol, schema, API, service-contract, reason-code, and event-contract documents that implementation can cite without executing docs as config.
  - Output: Specs directory contract and allowed generated-specs subpath.
  - Validation: Docs validation confirms service contracts live under docs/specs or documented SDS/service-catalog locations and generated specs stay out of hand-authored sources.

### Phase 2 Gate Outputs

#### Phase 2 Directory Contract Matrix

| Path | Owner | Gate state | Rule |
| --- | --- | --- | --- |
| `services` | Runtime service source roots | `top_level_contracts_scaffolded` | Owns service code roots only after an SDS-backed build phase creates them; not hidden discovery or production config. |
| `services/control-plane` | Initial control-plane process | `service_path_rules_defined` | One modular Rust API/worker process through Phase 3 unless measured pressure justifies a split. |
| `services/node-agent` | Overcell node agent and simulator | `service_path_rules_defined` | Owns node-agent and simulator code only when seed private swarm phases start. |
| `packages/schemas` | Contract source of truth | `package_path_rules_defined` | Canonical JSON Schema owns human-readable contracts; generated bindings are consumers. |
| `packages/sdk` | Rust SDK first | `package_path_rules_defined` | SDK code is generated or validated from shared contracts and must preserve SDK/Overgate routing. |
| `packages/cli` | Rust CLI | `package_path_rules_defined` | CLI remains a Rust package using shared contracts, signing, idempotency, trace propagation, and stable JSON output. |
| `infra/local` | Local development stack contracts | `local_infra_test_paths_defined` | Describes Overrid-shaped loopback local state, job tables, artifact stubs, service definitions, and profiles. |
| `tests/integration` | Cross-service scenarios | `local_infra_test_paths_defined` | Owns scenario manifests and ignored run artifacts; runtime services must not import test-only helpers. |
| `docs/specs` | Protocol and service contracts | `specs_contract_defined` | Owns hand-authored protocol, schema, API, service-contract, reason-code, event-contract, audit, and validation specs. |

#### Service Path Ownership

The `services/control-plane` path is the initial modular Rust control-plane process boundary, not a signal to split every future subsystem into deployable services. It may contain domain modules/crates once later phases implement Overpass-lite, Overtenant, Overkey-lite, Overgate, Overregistry, Overwatch, and Overqueue, but it remains one process through Phase 3 by default.

The `services/node-agent` path is reserved for Overcell node agent and simulator code. It must not bypass Overgate, Overmesh, Overlease, Overrun, Overmeter, or Overwatch contracts with direct private storage, queue, or artifact access.

#### Package Path Ownership

`packages/schemas`, `packages/sdk`, and `packages/cli` are required Phase 0 package roots. Package acceptance requires ownership metadata in README or manifest form, contract refs where required, and test-target declarations once executable code exists. TypeScript or generated bindings may exist for UI/client surfaces, but generated code is never the source of truth and TypeScript is not the core runtime authority.

Already implemented companion roots under `packages/admin_ui_shell`, `packages/integration_harness`, and `packages/local_stack` must also carry package-level README ownership metadata, source SDS/build-plan refs, and test-target declarations. `packages/admin_ui_shell` remains a TypeScript client surface only; `packages/integration_harness` and `packages/local_stack` remain Rust Phase 0 helper crates that consume shared contracts without becoming runtime services or production configuration sources.

#### Local Infrastructure And Integration Test Paths

`infra/local` tracks source-controlled local profiles and service definitions while local state, local job tables, object/artifact stub outputs, logs, and temporary chunks remain ignored. `tests/integration` tracks scenario manifests and source fixtures while generated run artifacts remain ignored unless a later acceptance path explicitly promotes redacted fixtures.

#### Specs Placement Contract

`docs/specs` is reserved for hand-authored protocol, schema, API, service-contract, reason-code, event-contract, audit-record, and validation-artifact documents. `docs/specs/generated` is the allowed generated-specs subpath and is ignored by default. Docs/specs files are citeable contracts, not runtime configuration or hidden service discovery.

## Phase 3: Workspace Manifest And Module Inventory

### Work Items

- **3.1 Define the root workspace manifest.**
  - Design: Add `overrid.workspace.toml` or an equivalent stable JSON/TOML file with schema version, module inventory, validation metadata, and documentation links.
  - Output: Root manifest contract that layout checks can consume.
  - Validation: Manifest tests reject missing schema version, duplicate module names, invalid paths, unknown module types, and stale documentation links.

- **3.2 Define module records.**
  - Design: Model each implemented module with name, type, owner layer, path, master phase, public contract path, allowed dependency groups, generated-output paths, test targets, local-stack participation, and documentation links.
  - Output: `module_record` schema and initial records for Phase 0 foundation modules.
  - Validation: Layout tests prove each implemented module has a contract or explicit reason for no public contract.

- **3.3 Define workspace inventory generation.**
  - Design: Let the command registry discover Cargo members, schema packages, CLI packages, SDK packages, test roots, local profiles, and docs/specs entries through the manifest and native workspace metadata.
  - Output: Inventory loader used by `layout:check`, `build`, `test`, `schema:check`, and docs validation.
  - Validation: Tests prove inventory discovery does not depend on hardcoded per-service scripts or docs files as runtime configuration.

- **3.4 Define phase and ownership metadata.**
  - Design: Require every module to name its first build phase and owner layer so later phase additions remain tied to master Phase 0 through Phase 13.
  - Output: Phase and owner metadata fields with accepted values and error classes.
  - Validation: Checks reject modules that omit phase, claim a future phase without a matching SDS/service plan, or bypass the build-plan crosswalk.

- **3.5 Define manifest drift checks.**
  - Design: Compare manifest records against filesystem paths, Cargo workspace members, docs links, test targets, local stack records, and generated-output ignores.
  - Output: Drift report with stable fields for missing path, stale link, missing test target, forbidden generated path, and unlisted module.
  - Validation: Fixture tests create drift cases and prove `layout:check` reports deterministic reason codes.

### Phase 3 Gate Outputs

#### Workspace Manifest Contract

The root `overrid.workspace.toml` file is the Phase 3 workspace manifest. It records schema version, manifest version, source document links, validation metadata, module-record schema enums, workspace inventory roots, drift reason codes, and initial module records. It is validation/build metadata only and must not become runtime service configuration, hidden service discovery, deployment orchestration, or production configuration.

The `workspace_manifest_defined` gate requires:

- `schema_version` is present and equals `1`.
- `source_sds`, `source_build_plan`, and `tech_stack_decision` point to existing repo documents.
- `validation_metadata.command_consumers` names `layout:check`, `build`, `test`, `schema:check`, and `docs:check` as consumers of the inventory.
- `workspace_inventory` lists Cargo workspace members, direct package roots, local profile roots, test roots, and specs roots without inventing runtime service discovery.

#### Module Record Schema

The `module_records_defined` gate requires every `module_record` to include:

- `name`
- `type`
- `owner_layer`
- `path`
- `master_phase`
- `public_contract_path`
- `allowed_dependency_groups`
- `generated_output_paths`
- `test_targets`
- `local_stack_participation`
- `documentation_links`

Initial Phase 0 records cover `packages/schemas`, `packages/sdk`, `packages/cli`, `packages/local_stack`, `packages/integration_harness`, `packages/admin_ui_shell`, `infra/local`, `tests/integration`, `docs/specs`, `services/control-plane`, and `services/node-agent`. Cargo-backed records also declare their Cargo member and package name. Scaffolded service roots remain layout contracts until their owning later phases add runtime code.

#### Inventory Discovery Rules

The `inventory_discovery_defined` gate requires layout checks to compare `overrid.workspace.toml` against native workspace metadata and source-controlled paths:

- `Cargo.toml` workspace members must match manifest `cargo_member` entries.
- Every direct implemented `packages/` root must have exactly one manifest module record.
- Local profile roots, service-definition roots, test roots, and specs roots must exist and remain source contracts.
- Generated-output paths must be protected by standard ignore markers before they can appear in a module record.
- Inventory discovery must not depend on hardcoded per-service shell scripts or docs files as runtime configuration.

#### Phase And Owner Metadata

The `phase_owner_metadata_defined` gate requires module records to name accepted `type`, `owner_layer`, `master_phase`, dependency groups, lifecycle state, and local-stack participation values. Phase 3 records use master Phase 0 for the layout contract itself; later phases may add or promote records only when the owning SDS, service plan, phase document, and crosswalk justify the change.

#### Manifest Drift Reason Codes

The `manifest_drift_checks_defined` gate requires deterministic drift reason codes:

- `missing_schema_version`
- `missing_module_records`
- `missing_required_field`
- `duplicate_module_name`
- `invalid_path`
- `missing_path`
- `unknown_module_type`
- `unknown_owner_layer`
- `invalid_master_phase`
- `unknown_dependency_group`
- `missing_public_contract`
- `stale_documentation_link`
- `missing_test_target`
- `forbidden_generated_path`
- `missing_local_stack_participation`
- `cargo_member_drift`
- `unlisted_module`

Focused fixture checks must prove at least the missing schema version, duplicate module name, missing path, unknown module type, stale documentation link, missing test target, forbidden generated path, and unlisted module cases.

## Phase 4: Contracts, Specs, And Schema Source Of Truth

### Work Items

- **4.1 Establish `packages/schemas` as contract authority.**
  - Design: Store canonical JSON Schema contracts for commands, manifests, fixtures, signed payloads, view models, events, audit records, errors, and docs-facing examples in language-neutral files.
  - Output: Schema-source directory rules and naming conventions.
  - Validation: `schema:check` proves source contracts exist before services, SDKs, CLI, or tests consume boundary objects.

- **4.2 Define generated binding boundaries.**
  - Design: Generate or validate Rust SDK bindings and TypeScript/web bindings from contracts while preventing generated code from becoming the authority.
  - Output: Generated-output paths and source-of-truth notes for Rust and TypeScript bindings.
  - Validation: Checks reject generated files outside approved generated paths and reject hand-edited generated binding sources when source contracts changed.

- **4.3 Define optional Protobuf placement.**
  - Design: Allow Protobuf only for compact internal service/RPC/event contracts where the owning SDS and specs justify it, while keeping JSON Schema canonical for human-readable commands and manifests.
  - Output: Protobuf path and ownership rule tied to docs/specs and package metadata.
  - Validation: Review confirms Protobuf additions do not replace JSON Schema for docs-facing contract examples or signed command payloads.

- **4.4 Define service contract stub content.**
  - Design: Require every implemented service/module contract to include purpose, owned data, public API, events emitted, events consumed, security boundary, operational checks, test expectations, schema refs, and owning phase.
  - Output: Service contract template under docs/specs or a documented equivalent path.
  - Validation: Docs checks reject implemented modules without service contract stubs or required contract sections.

- **4.5 Define reason-code and event-contract placement.**
  - Design: Keep stable reason codes, event envelopes, audit records, validation artifacts, and error shapes discoverable from docs/specs and packages/schemas.
  - Output: Reason-code and event-contract path rules.
  - Validation: Schema and docs checks prove mutating service contracts cite reason-code and event-contract sources before service logic is accepted.

### Phase 4 Gate Outputs

#### Schema Authority Contract

The `schema_authority_defined` gate makes `packages/schemas` the language-neutral contract authority for commands, manifests, fixtures, signed payloads, view models, events, audit records, errors, and docs-facing examples. Canonical source files use `*.schema.json` under versioned schema-family directories such as `v0/`, with fixtures split into `fixtures/valid` and `fixtures/invalid`, and each family must maintain a `codegen_manifest.json` that names canonical schemas, fixtures, generated/projection outputs, and source-of-truth status.

Service, SDK, CLI, UI, local-stack, and harness code may consume these contracts only as generated or validated projections. They must not treat Rust structs, TypeScript declarations, docs examples, fixture outputs, or generated artifacts as the contract source of truth.

Current Phase 4 schema authority paths include `packages/schemas/admin_ui/v0`, `packages/schemas/overrid_contracts/v0`, `packages/schemas/admin_ui/codegen_manifest.json`, and `packages/schemas/overrid_contracts/codegen_manifest.json`.

#### Generated Binding Boundary

The `generated_binding_boundaries_defined` gate requires generated or projected Rust and TypeScript/web bindings to declare:

- canonical source schema path;
- generated/projection output path;
- `source_of_truth = json_schema`;
- `non_authoritative = true`;
- validator or test target that checks projection alignment.

Approved generated-output roots are documented in `docs/specs/contract_authority.md`, `packages/schemas/README.md`, and `overrid.workspace.toml`. Generated files outside approved roots are layout violations unless a later SDS-backed contract update adds the path and ignore/validation evidence.

#### Optional Protobuf Placement

The `protobuf_placement_defined` gate allows Protobuf only for compact internal service/RPC/event contracts when an owning SDS, `docs/specs` entry, and package metadata justify it. Protobuf may not replace JSON Schema for human-readable commands, signed command payloads, manifests, fixtures, view models, docs-facing examples, reason codes, audit records, or error shapes.

#### Service Contract Stub Template

The `service_contract_template_defined` gate requires implemented service/module contracts to include these sections before service logic is accepted:

- Purpose
- Owned Data
- Public API
- Events Emitted
- Events Consumed
- Security Boundary
- Operational Checks
- Test Expectations
- Schema Refs
- Owning Phase

The template lives in `docs/specs/service_contract_template.md`. Equivalent SDS/service-catalog locations may be used only when the module record documents the path and the docs check verifies every required section.

#### Reason-Code And Event Contract Placement

The `reason_event_contracts_defined` gate keeps stable reason codes, event envelopes, audit records, validation artifacts, and error shapes discoverable from both `docs/specs/reason_codes_and_events.md` and `packages/schemas`. Mutating service contracts must cite the relevant schema family and reason/event contract source before implementation is accepted.

## Phase 5: Root Command Registry And Layout Check

### Work Items

- **5.1 Define semantic root commands.**
  - Design: Preserve the SDS command names: `build`, `test`, `test:integration`, `dev:start`, `dev:stop`, `dev:reset`, `dev:seed`, `dev:status`, `schema:check`, `docs:check`, and `layout:check`.
  - Output: Root command registry records with command purpose, inputs, outputs, owning tool, machine-readable result envelope, and failure classes.
  - Validation: Registry tests reject duplicate command names, missing machine-readable output, and commands without owner or phase metadata.

- **5.2 Implement Rust-owned command execution.**
  - Design: Expose canonical commands through the Rust `overrid` CLI, Cargo workspace metadata, or `cargo xtask`-style tooling; optional shell, Make, just, or npm aliases may call the same canonical path.
  - Output: Command-entrypoint plan with alias rules.
  - Validation: Review confirms no command's authoritative behavior lives only in a shell script, npm script, or docs page.

- **5.3 Implement `layout:check`.**
  - Design: Check required directories, workspace manifest records, module records, service contract stubs, generated-output ignores, secret-file rules, package boundary rules, local state markers, and docs links.
  - Output: Layout checker with human and JSON output.
  - Validation: Fixture tests cover missing directory, unlisted module, missing contract, forbidden dependency, generated file committed, secret-like file committed, and stale docs link.

- **5.4 Wire schema and docs checks.**
  - Design: Make `schema:check` validate contracts and fixtures, and `docs:check` validate markdown links, headings, stale-note markers, revenue/pricing assumption scans, SDS/service/build-plan alignment, and crosswalk references.
  - Output: Check orchestration records that can run locally and in CI.
  - Validation: CI/local smoke proves `layout:check`, `schema:check`, and `docs:check` produce stable pass/fail/block statuses and artifact refs.

- **5.5 Define validation artifacts.**
  - Design: Produce `layout_check.passed`, `layout_check.failed`, `package_boundary_violation`, `missing_service_contract`, `missing_test_target`, `generated_file_committed`, and `secret_file_committed` as CI/build artifacts, not Overwatch runtime events.
  - Output: Artifact schema and retention rule for layout checks.
  - Validation: Tests prove validation artifacts include reason code, path, owning phase, module id when available, and no raw secret values.

### Phase 5 Gate Outputs

#### Semantic Root Command Registry

The `root_command_registry_defined` gate records these semantic root commands in `overrid.workspace.toml` and the Rust `overrid` CLI:

- `build`
- `test`
- `test:integration`
- `dev:start`
- `dev:stop`
- `dev:reset`
- `dev:seed`
- `dev:status`
- `schema:check`
- `docs:check`
- `layout:check`

Each command record includes command purpose, inputs, outputs, owning tool, canonical invocation, machine-readable result envelope support, failure classes, phase gate, and thin-alias metadata. Duplicate names, missing owner metadata, missing phase metadata, and commands without machine-readable output are layout violations.

#### Rust-Owned Command Execution

The `rust_owned_command_execution_defined` gate makes `packages/cli` and the `overrid` binary the canonical owner for Repository Layout Phase 5 command behavior. `overrid command-registry` prints the root command registry, and `overrid layout:check` runs the layout check with human and JSON output. Shell, Make, just, npm, or CI aliases may exist only as thin wrappers around the Rust-owned command path and must not become the authoritative behavior.

#### Layout Check Contract

The `layout_check_defined` gate requires `layout:check` to report required directories, workspace manifest presence, module-record/test-target markers, service contract stubs, generated-output ignore markers, forbidden secret-like files, package-boundary metadata, local-state markers, and docs-link contract files. JSON output must use the CLI result envelope and include per-check records with reason code, path, owning phase, and module id where available.

Focused fixture coverage must prove missing directory, unlisted module, missing contract, forbidden dependency, generated file committed, secret-like file committed, and stale docs link failure classes through validator fixtures or equivalent deterministic checks.

#### Schema And Docs Check Orchestration

The `schema_docs_check_orchestration_defined` gate keeps `schema:check`, `docs:check`, and `layout:check` as local and CI-runnable semantic records. `schema:check` validates `packages/schemas` and generated/projection metadata. `docs:check` validates Markdown links, headings, stale markers, restricted pricing/revenue assumptions, SDS/service/build-plan alignment, and crosswalk references. `layout:check` validates workspace shape and hygiene before later phases add runtime code.

#### Validation Artifact Schema

The `validation_artifacts_defined` gate defines the Phase 5 validation artifact names:

- `layout_check.passed`
- `layout_check.failed`
- `package_boundary_violation`
- `missing_service_contract`
- `missing_test_target`
- `generated_file_committed`
- `secret_file_committed`

Artifact records include reason code, path, owning phase, module id when available, and artifact ref. Secret-file artifacts must report only path and reason metadata, never raw secret values. These records are CI/build artifacts and are not Overwatch runtime events.

## Phase 6: Package Boundary Enforcement And Modular Control-Plane Shape

### Work Items

- **6.1 Define dependency direction groups.**
  - Design: Model allowed dependency groups for schemas, SDK, CLI, local tooling, integration tests, control-plane modules, node-agent modules, and docs/specs helpers.
  - Output: Package-boundary rules in the workspace manifest or dedicated boundary config.
  - Validation: Boundary tests reject runtime imports from integration tests, local-only modules, generated artifacts, docs, or future service folders.

- **6.2 Enforce shared-schema dependency paths.**
  - Design: Require service boundary objects to flow through packages/schemas and generated/validated bindings instead of private structs crossing services without contracts.
  - Output: Boundary check for contract imports and schema refs.
  - Validation: Tests reject service API payloads, event payloads, command envelopes, and fixture records that have no schema ref.

- **6.3 Preserve the modular control-plane process.**
  - Design: Keep Overgate, Overtenant, Overpass-lite, Overkey-lite, Overregistry, Overwatch, Overqueue, scheduler handoff, leases, and metering interfaces as separate crates/modules and contracts inside `services/control-plane` through Phase 3 by default.
  - Output: Control-plane module layout rule with future split conditions.
  - Validation: Review confirms the layout does not create premature deployable services for every control-plane domain.

- **6.4 Define split-review criteria.**
  - Design: Allow independent service boundaries only after documented API load, failure-isolation, security-boundary, operational, or grid-resident backbone pressure proves the need in Phase 4+ or Phase 7.
  - Output: Split-review checklist tied to SDS, service catalog, build-plan crosswalk, and docs/specs updates.
  - Validation: Review rejects service splits without measured rationale and updated contracts.

- **6.5 Enforce local/test-only separation.**
  - Design: Prevent runtime services from importing test harness, local stack internals, fixture generation internals, or local-only command helpers outside approved local/test entrypoints.
  - Output: Boundary rules for local-only and test-only modules.
  - Validation: Tests prove production-facing modules cannot depend on `infra/local`, test fixtures, integration artifact writers, or local simulator internals.

### Phase 6 Gate Outputs

#### Dependency Direction Groups

The `dependency_direction_groups_defined` gate records dependency groups for `contracts`, `sdk`, `cli`, `local_stack`, `integration_harness`, `admin_ui_shell`, `docs`, `local_infra`, `control_plane_modules`, `node_agent_modules`, and `docs_specs`. Runtime-facing modules may consume `contracts` and later explicitly approved runtime modules, but they must not depend on local/test-only groups as authority.

The workspace manifest must carry Phase 6 boundary metadata so `layout:check`, validators, and future CI can report package-boundary findings with deterministic reason codes instead of relying on ad hoc import review.

#### Shared-Schema Dependency Enforcement

The `shared_schema_dependency_paths_enforced` gate requires service API payloads, event payloads, command envelopes, fixture records, read models, and audit/error shapes to cite `packages/schemas`, `packages/schemas/overrid_contracts`, or `docs/specs` contract sources before implementation consumes them.

Rust crates that exchange boundary objects must depend on the `overrid-contracts` projection or a later generated/validated projection from the same canonical schemas. Private structs may exist inside one module, but they must not become cross-service or cross-package public contracts.

#### Modular Control-Plane Shape

The `modular_control_plane_shape_preserved` gate keeps `services/control-plane` as one modular Rust process through master Phase 3 by default. Overgate, Overtenant, Overpass-lite, Overkey-lite, Overregistry, Overwatch, Overqueue, scheduler handoff, lease handoff, and metering interface code may be separate crates/modules/contracts inside that boundary, but this phase must not create independently deployable service folders for each domain.

#### Split-Review Criteria

The `split_review_criteria_defined` gate allows a future service split only when phase docs, SDS files, service-catalog entries, build-plan crosswalk rows, docs/specs contracts, and validation evidence show measured API load, failure-isolation, security-boundary, operational, or grid-resident backbone pressure. Phase 4+ may justify security/operational splits, and Phase 7 may justify protected grid-resident backbone splits.

Split reviews that lack measured rationale produce `split_review_missing`; premature domain folders or deployable manifests produce `premature_service_split`.

#### Local/Test-Only Separation

The `local_test_only_separation_enforced` gate marks `local_stack`, `integration_harness`, `local_infra`, integration artifacts, fixture writers, and docs helpers as local/test-only unless an owning SDS later promotes a contract-backed adapter path. Runtime-facing modules must not import `infra/local`, integration artifact writers, local simulators, or docs files as executable configuration.

Phase 6 layout artifacts include `package_boundary_violation`, `schema_ref_missing`, `premature_service_split`, `split_review_missing`, and `local_test_boundary_violation`.

## Phase 7: Generated Artifacts, Secrets, Local State, And Index Hygiene

### Work Items

- **7.1 Define generated-output ignore rules.**
  - Design: Ignore `target/`, `node_modules/`, package caches, coverage, logs, generated SDK/types/docs, integration artifacts, generated fixture outputs, and temporary object chunks.
  - Output: `.gitignore`, `.docdexignore` when present, and workspace-manifest generated-output entries.
  - Validation: `layout:check` fails when generated outputs are unignored or committed in source-controlled areas.

- **7.2 Define local-state ignore rules.**
  - Design: Keep local state under `.overrid/`, `infra/local/state/`, or approved local-only state paths with explicit local/test markers and safe reset behavior.
  - Output: Local-state path and marker rules consumed by SDS #4 local stack work.
  - Validation: Reset and layout checks reject unmarked state paths, production-like state names, and local state committed to source.

- **7.3 Define secret-file rules.**
  - Design: Permit only example files in source, and reject secret-bearing environment files, `*.local.*`, `*.secret.*`, `*.key`, raw tokens, private keys, and fixture credentials outside local/test generated paths.
  - Output: Secret-file policy and scan patterns with false-positive handling for explicit negative-control documentation lines.
  - Validation: Security scans reject committed secret-like files and prove docs can still contain lines that explicitly prohibit secret handling mistakes.

- **7.4 Define Docdex indexing hygiene.**
  - Design: Keep docs, specs, SDS, build plans, service catalog files, source schemas, handwritten fixtures, and service contract stubs indexed while excluding large generated artifacts and local caches.
  - Output: Docdex/indexing rule in the workspace manifest and optional `.docdexignore`.
  - Validation: Docdex files/stats checks confirm the plan, SDS, service catalog, specs, and source schemas remain indexed after changes.

- **7.5 Define artifact redaction expectations.**
  - Design: Ensure layout validation artifacts, docs checks, and CI bundles redact secrets, keys, tokens, signatures, private payloads, encrypted content, and local fixture credentials.
  - Output: Redaction rule shared with SDS #3 and SDS #4 artifact work.
  - Validation: Redaction tests inject sentinel values and fail artifact export until forbidden values are removed.

### Phase 7 Gate Outputs

#### Generated-Output Ignore Rules

The `generated_output_ignore_rules_defined` gate records generated and cache paths in `.gitignore`, `.docdexignore`, and `overrid.workspace.toml`. The ignored paths include `target`, `node_modules`, package caches, coverage, logs, `docs/specs/generated`, generated SDK/type/doc output, `infra/local/artifacts`, `tests/integration/artifacts`, generated fixture outputs, and temporary object chunks.

Existing source-controlled generated projections are allowed only when they are explicitly listed as approved generated projection files and remain non-authoritative consumers of canonical schemas. Additional generated outputs produce `generated_file_committed` until ignored, regenerated from source contracts, or explicitly promoted through layout governance.

#### Local-State Ignore Rules

The `local_state_ignore_rules_defined` gate keeps local state under `.overrid`, `infra/local/state`, `infra/local/job-tables`, `infra/local/artifacts`, and `tests/integration/artifacts` with source-visible ignore markers. Local state paths are development/test-only, resettable, and must not become production configuration, hidden service discovery, or runtime storage authority.

Unmarked local state, production-like local state names, or source-controlled local runtime state produce `local_state_committed`.

#### Secret-File Rules

The `secret_file_rules_defined` gate allows example files such as `.env.example` while rejecting committed `.env`, `.env.*`, `*.local.*`, `*.secret.*`, `*.key`, `*.pem`, `*.p12`, `*.pfx`, `*.token`, `secrets.*`, and private-key path names such as `id_ed25519`. Secret findings must report path and reason metadata only.

Documentation may contain negative-control lines that explicitly prohibit unsafe handling, but no validation artifact may include raw secret values.

#### Docdex Indexing Hygiene

The `docdex_indexing_hygiene_defined` gate keeps hand-authored docs, specs, SDS files, build plans, service catalog files, source schemas, handwritten fixtures, and service contract stubs indexable while excluding generated artifacts, local state, dependency caches, logs, coverage, and runtime caches through `.docdexignore`.

Docdex validation should confirm `docs/build_plan`, `docs/sds`, `docs/service_catalog`, `docs/specs`, and source schema/fixture roots remain visible after changes. Misconfigured indexing rules produce `docdex_index_hygiene_violation`.

#### Artifact Redaction Expectations

The `artifact_redaction_expectations_defined` gate requires layout validation artifacts, docs checks, CI bundles, harness artifacts, and local-stack exports to redact secrets, keys, tokens, signatures, private payloads, encrypted content, and local fixture credentials. Artifact records may include reason code, path, owning phase, module id, and artifact ref, but not raw secret values or private payload contents.

Phase 7 layout artifacts include `generated_file_committed`, `secret_file_committed`, `local_state_committed`, `docdex_index_hygiene_violation`, and `artifact_redaction_violation`.

## Phase 8: Service Contract Templates And New-Module Checklist

### Work Items

- **8.1 Implement the service contract template.**
  - Design: Provide a repeatable contract stub for purpose, owned data, public API, events emitted, events consumed, security boundary, operational checks, test expectations, schema refs, phase refs, and downstream dependencies.
  - Output: Template under docs/specs or a documented equivalent plus usage notes.
  - Validation: Docs checks reject implemented services without the required contract sections.

- **8.2 Implement the new-service checklist.**
  - Design: Require new modules to update service catalog, SDS, build plan/crosswalk, docs/specs contracts, packages/schemas entries, test targets, local-stack participation where applicable, and module records.
  - Output: Checklist used by developers and future agent tasks before implementation starts.
  - Validation: Sample-service validation proves the checklist can add a module without inventing new top-level folders.

- **8.3 Define module addition workflow.**
  - Design: Move module changes through proposed, scaffolded, contracted, wired, validated, and accepted states with explicit evidence for each state.
  - Output: Workflow documentation and machine-readable status fields.
  - Validation: `layout:check` reports modules stuck in invalid states or accepted modules lacking wired validation.

- **8.4 Define deprecation and removal workflow.**
  - Design: Require deprecated and removed states to update docs, workspace manifest, tests, local stack, harness scenarios, generated outputs, and Docdex references before deletion.
  - Output: Deprecation/removal checklist.
  - Validation: Tests and docs checks reject deleted modules with stale references or missing replacement notes.

- **8.5 Define cross-document maintenance rules.**
  - Design: Keep SDS files, service catalog plans, build-plan phase docs, sub-build plans, service_catalog_alignment, and master_plan aligned whenever layout rules or service boundaries change.
  - Output: Maintenance rule referenced by this plan and future sub-build plans.
  - Validation: Link and alignment checks detect missing sub-build-plan refs, wrong first build phase, and stale master/crosswalk rows.

### Phase 8 Gate Outputs

#### Service Contract Template Usage

The `service_contract_template_implemented` gate keeps `docs/specs/service_contract_template.md` as the repeatable service/module contract stub. Contracts include purpose, owned data, public API, events emitted, events consumed, security boundary, operational checks, test expectations, schema refs, owning phase, and downstream dependencies before implemented service logic is accepted.

Equivalent SDS or service-catalog contract paths are allowed only when the module record names the path, documents a no-public-contract reason where appropriate, and docs checks verify every required section.

#### New-Service Checklist

The `new_service_checklist_defined` gate records the developer and future-agent checklist in `docs/specs/new_module_checklist.md`. A new module must update SDS, service catalog, build plan/crosswalk, docs/specs contracts, packages/schemas entries or no-public-contract reason, test targets, local-stack participation where applicable, generated-output rules, and workspace module records before acceptance.

Checklist validation uses existing service roots as sample-service evidence: `services/control-plane` and `services/node-agent` remain scaffolded service contract roots without inventing new top-level folders.

#### Module Addition Workflow

The `module_addition_workflow_defined` gate records lifecycle states in `overrid.workspace.toml`: `proposed`, `scaffolded`, `contracted`, `wired`, `validated`, and `accepted`. Module records carry machine-readable `lifecycle_state` values so `layout:check` and validators can reject invalid states and accepted modules without wired validation evidence.

#### Deprecation And Removal Workflow

The `deprecation_removal_workflow_defined` gate extends the lifecycle with `deprecated` and `removed`. Deprecated and removed modules require replacement or removal notes, manifest updates, docs updates, test-target updates, local-stack updates, harness-scenario updates, generated-output updates, and Docdex reference cleanup before deletion is accepted.

#### Cross-Document Maintenance Rules

The `cross_document_maintenance_rules_defined` gate keeps `docs/sds/foundation/repository_layout.md`, `docs/service_catalog/foundation/repository_layout.md`, this sub-build plan, `docs/build_plan/master_plan.md`, `docs/build_plan/service_catalog_alignment.md`, `docs/specs/service_contract_template.md`, `docs/specs/new_module_checklist.md`, `overrid.workspace.toml`, and phase planning/progress docs aligned whenever layout rules or service boundaries change.

Phase 8 layout artifacts include `missing_service_contract`, `missing_test_target`, `module_lifecycle_violation`, and `stale_layout_reference`.

## Phase 9: Foundation Integration With Local Stack, Harness, And CI

### Work Items

- **9.1 Expose layout metadata to the Local Development Stack.**
  - Design: Let SDS #4 local stack discover service definitions, profile paths, local state paths, ports, generated env paths, and safe reset markers from workspace layout metadata.
  - Output: Local-stack discovery fields in module records or related manifest records.
  - Validation: Local-stack tests prove it can reject missing service definitions, unsafe state paths, and unknown profile directories before startup.

- **9.2 Expose layout metadata to the Integration Test Harness.**
  - Design: Let SDS #3 harness discover scenario roots, fixture roots, artifact roots, schema refs, local stack commands, and test targets from the workspace manifest.
  - Output: Harness discovery fields and integration-test directory rules.
  - Validation: Harness smoke can list scenario manifests, fixture manifests, and artifact output paths without bespoke per-service scripts.

- **9.3 Define clean-checkout CI behavior.**
  - Design: Make Linux clean-checkout CI run `layout:check`, `schema:check`, `docs:check`, unit tests, local stack smoke, and harness smoke only through canonical root commands when those implementations exist.
  - Output: CI command sequence and blocked/fail/pass status model.
  - Validation: CI dry-run or local validation distinguishes missing test-runner configuration from documentation regressions.

- **9.4 Define docs and layout validation evidence.**
  - Design: Record link checks, phase-heading checks, work-item structure checks, stale-note scans, rejected-assumption scans, Docdex indexing, and queue/progress updates in the build-plan progress document.
  - Output: Progress evidence entries tied to SDS #5.
  - Validation: Progress review can trace the created sub-build plan to validation commands and known blockers.

- **9.5 Define validation artifact consumers.**
  - Design: Keep layout artifacts usable by CI, local developer commands, Docdex, and future agents without treating them as Overwatch runtime audit events.
  - Output: Artifact ownership and consumer list.
  - Validation: Review confirms validation artifacts remain build/CI evidence and do not imply runtime platform event behavior.

## Phase 10: Validation, Documentation Alignment, And Downstream Handoff

### Work Items

- **10.1 Validate sub-build-plan structure.**
  - Design: Check title prefix, attached SDS link, phase headings 1 through 10, work item structure, Design/Output/Validation fields, and exit gate.
  - Output: Focused validation result for `SUB BUILD PLAN #5`.
  - Validation: Scripted text checks pass for required headings, numbered phases, local links, and work-item sections.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit this plan and linked docs for Rust-first workspace ownership, language-neutral schema authority, Overrid-shaped local primitives, modular control-plane boundaries, and no conventional cloud product assumptions.
  - Output: Tech-stack alignment checklist for Repository Layout.
  - Validation: Scans find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions except lines that explicitly reject those assumptions.

- **10.3 Validate master-plan alignment.**
  - Design: Confirm the master Phase 0 through Phase 13 order remains unchanged and SDS #5 is represented as a Phase 0 foundation sub-build plan.
  - Output: Updated master-plan and crosswalk rows for SDS #5.
  - Validation: Review confirms only per-SDS sub-build indexing and Phase 0 wording cleanup changed; no phase-order dependency was altered.

- **10.4 Validate service-catalog and SDS alignment.**
  - Design: Ensure SDS #5 and the Repository Layout service plan link back to this sub-build plan and preserve the service's first build phase.
  - Output: Updated source-document and sub-build-plan references.
  - Validation: Local Markdown link validation across changed docs returns no missing local targets.

- **10.5 Prepare downstream phase handoff.**
  - Design: Document how later builders add control-plane, node-agent, execution, trust, accounting, product integration, grid-resident, data/storage/namespace, deployment, federation, native-app, mobile, and governance modules inside the established layout.
  - Output: Handoff rules for all master phases without new top-level sprawl.
  - Validation: Handoff review confirms later phases can add modules, contracts, tests, local profiles, and generated outputs without changing the master build order or weakening schema/boundary rules.

## Alignment Review

- The sub-build plan keeps Repository Layout first build work in master Phase 0, matching the SDS, service catalog entry, Phase 0 plan, and build-plan crosswalk.
- The plan treats Repository Layout as a governance/build-contract artifact, not a runtime service, service discovery database, or deployment controller.
- The plan carries forward SDS #5 resolved decisions: Rust-owned semantic root commands, one modular control-plane process through Phase 3, language-neutral schema authority, explicit generated/local ignore rules, and minimal root workspace manifest metadata.
- The plan aligns Phase 0 wording with the accepted tech stack by describing local durable state, durable job tables, and object/artifact stubs as Overrid-shaped local primitives rather than conventional database, queue, or object-store product boundaries.
- The plan explicitly depends on Shared Schema Package, Local Development Stack, Integration Test Harness, CLI/SDK, docs/specs, service catalog, SDS, and build-plan crosswalk without moving later services earlier.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.

## Exit Gate

SUB BUILD PLAN #5 is complete when a builder can implement Repository Layout as a Rust-first, manifest-backed, schema-governed workspace contract that creates the Phase 0 folder structure, records module ownership, enforces package boundaries, exposes stable root command semantics, excludes generated/local/secret artifacts, validates docs and contracts, supports Docdex indexing, and hands later phases a predictable way to add services without premature microservice sprawl or tech-stack drift.
