# SUB BUILD PLAN #3 - Integration Test Harness

Attached SDS: [docs/sds/foundation/integration_test_harness.md](../sds/foundation/integration_test_harness.md)

## Purpose

This sub-build plan turns SDS #3 into an implementation sequence for the Integration Test Harness. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

The harness is a development and release gate, not a production service. It must orchestrate the local stack, seed deterministic fixtures, execute black-box scenario workflows, assert shared schemas and golden traces, and emit redacted artifacts that make cross-service regressions reproducible.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #3: Integration Test Harness](../sds/foundation/integration_test_harness.md) | Controls harness purpose, fixture/scenario/run/golden-trace data models, helper API surface, security, validation, and resolved open-question decisions. |
| [Integration Test Harness service plan](../service_catalog/foundation/integration_test_harness.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Controls the first implementation point for local stack smoke tests, deterministic fixtures, and integration harness foundation. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps Integration Test Harness aligned to master Phase 0 as the first build phase. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires a Rust-first harness in the Cargo workspace with JSON/JSON Schema contracts and Overrid-shaped local stubs instead of conventional cloud product boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 0 | Attach SDS #3 to the build-plan layer and freeze harness boundaries before implementation. |
| 2 | Master Phase 0 | Define scenario, fixture, run-record, assertion, artifact, and golden-trace schemas through the shared schema package. |
| 3 | Master Phase 0 | Build the Rust harness crate, runner API, manifest loader, and deterministic execution shell. |
| 4 | Master Phase 0 | Integrate local development stack lifecycle, reset, seed, health, logs, and clean-checkout smoke behavior. |
| 5 | Master Phase 0 and Phase 1 | Build deterministic fixtures for Phase 0 smoke and Phase 1 control-plane spine scenarios. |
| 6 | Master Phase 0 and Phase 1 | Build scenario execution, CLI/SDK/API step runners, stable exit codes, and fail-closed dependency checks. |
| 7 | Master Phases 0, 1, 3, 4, and 5 | Implement exact and DAG-based golden-trace assertions for protocol, queue, execution, policy, usage, ledger, and dispute events as phases mature. |
| 8 | Master Phase 0 with later use in all phases | Build redacted artifact bundles, reproduction commands, diagnostics, flake detection, and retention classes. |
| 9 | Master Phases 0 through 13 | Add phase-gate selection so each master phase inherits earlier gates and adds only the minimal new contract-spine scenarios. |
| 10 | Master Phase 0 with handoff to Phases 1 through 13 | Validate the harness itself and document downstream service-handoff rules. |

## Tech Stack Guardrails

- The authoritative harness runner belongs in the Rust Cargo workspace as shared test utilities plus `overrid test ...` CLI commands.
- Scenario manifests, fixture manifests, golden traces, assertion records, and artifact bundle schemas must remain language-neutral JSON/JSON Schema contracts owned by the shared schema package.
- TypeScript, client checks, browser checks, or product-specific scripts may run as harness steps later, but they must not become the authoritative phase-gate runner.
- The harness must use SDK, CLI, Overgate, and local test hooks for behavior assertions; direct storage inspection is limited to reset and diagnostics.
- Local "database", "queue", and "object store" substitutes must be Overrid-shaped local stubs or embedded engines behind Overrid-owned abstractions.
- The harness must not make PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions.
- Fixture credentials must be visibly `test_only` and rejected outside local/test profiles.

## Phase 1: SDS Attachment, Harness Boundary, And Gate Semantics

### Work Items

- **1.1 Attach the build plan to SDS #3.**
  - Design: Link this document from the numbered SDS, service catalog entry, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/foundation/integration_test_harness.md`, `docs/service_catalog/foundation/integration_test_harness.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #3 returns both the SDS and this sub-build plan.

- **1.2 Freeze the non-production harness boundary.**
  - Design: Record that the harness is a development/release gate and test orchestrator, not a runtime service, policy engine, production event writer, payment actor, or provider-side execution service.
  - Output: Boundary guardrails documented in this plan and referenced by later phase work.
  - Validation: Architecture review confirms no work item requires production endpoints, real payment providers, public-provider resources, raw secrets, or weakened platform rules.

- **1.3 Define gate classes by master phase.**
  - Design: Split scenario suites into `smoke`, `contract_spine`, `regression`, `extended`, and `release_candidate`, with mandatory gates limited to the current phase's spine plus all earlier phase gates.
  - Output: Gate-class matrix for master Phases 0 through 13.
  - Validation: Phase 0 and Phase 1 gate definitions match SDS #3 resolved decisions and do not promote later flows early.

- **1.4 Define dependency fail-closed behavior.**
  - Design: Required services, schemas, local stack profiles, and scenario tags must be checked before execution; missing requirements produce `blocked` test runs rather than partial false passes.
  - Output: Dependency contract for stack, schemas, CLI/SDK, Overgate, Overwatch, Overqueue, and later owning services.
  - Validation: Tests prove absent local stack, missing schema, unavailable service, and wrong phase tag all produce stable blocked outcomes.

- **1.5 Define documentation-update rules.**
  - Design: When a service adds a public contract, it must add or update scenario manifests and golden traces before its service plan can claim integration readiness.
  - Output: Cross-document maintenance rule for service SDS files, service catalog plans, and this sub-build plan.
  - Validation: Review checklist rejects service implementation plans that add public APIs or events without harness scenario coverage.

## Phase 2: Harness Schemas, Fixtures, And Compatibility Contracts

### Work Items

- **2.1 Define fixture manifest schemas.**
  - Design: Add `fixture_manifest`, `fixture_identity`, `fixture_key`, resource card, workload id, package id, local ledger account, and policy-context refs with deterministic seeds and explicit schema versions.
  - Output: Canonical fixture schemas plus valid and invalid fixture examples.
  - Validation: Schema tests prove missing seed, missing tenant, missing actor, non-test key, and incompatible fixture version fail closed.

- **2.2 Define scenario manifest schemas.**
  - Design: Add `scenario_manifest` and `scenario_step` with phase, tags, required services, setup fixtures, action kind, input refs, timeout, retry expectations, expected result class, and cleanup rules.
  - Output: Scenario schema for CLI, SDK, API, local helper, and assertion steps.
  - Validation: Fixture tests cover valid smoke, blocked dependency, invalid action kind, unsupported phase, missing assertion, and unsafe timeout variants.

- **2.3 Define run and assertion record schemas.**
  - Design: Add `test_run_record` and `assertion_result` with run id, scenario ids, stack profile, workspace fingerprint when available, status, timing, reason code, field path, expected value, actual value, and artifact refs.
  - Output: Stable run and assertion records for CI, local CLI output, diagnostics, and progress evidence.
  - Validation: Schema tests prove every terminal run has a status, started/ended metadata, reason class, and artifact policy.

- **2.4 Define golden trace schemas.**
  - Design: Support exact ordered traces for Phase 0/1 protocol spine flows and partially ordered DAG traces for later concurrent flows, with required nodes, required causal edges, forbidden transitions, schema versions, and stable reason codes.
  - Output: `golden_trace` schema with exact and DAG assertion modes.
  - Validation: Golden tests prove missing events, extra state-changing events, reordered exact events, missing causal edges, and forbidden transitions are detected.

- **2.5 Define artifact bundle schemas.**
  - Design: Add `artifact_bundle` with redacted logs, Overwatch exports, CLI output, API payload envelopes, stack health, fixture versions, reproduction command, retention class, and redaction summary.
  - Output: Artifact schema with explicit retention metadata and privacy classifications.
  - Validation: Redaction tests reject raw secrets, private keys, tokens, signatures, encrypted RAG content, private payloads, and fixture key material.

## Phase 3: Rust Harness Crate, Runner API, And Execution Shell

### Work Items

- **3.1 Create the Rust harness crate boundary.**
  - Design: Add a Cargo workspace test-utility crate and runner module that depend on shared contracts, SDK/CLI helpers, local-stack hooks, and artifact utilities through stable internal boundaries.
  - Output: Harness crate skeleton with runner, manifests, fixtures, assertions, artifacts, and phase-gate modules.
  - Validation: `cargo check` for the harness crate succeeds once implementation exists and no Node.js/TypeScript runtime dependency becomes authoritative.

- **3.2 Implement manifest loading and validation.**
  - Design: Load scenario and fixture manifests from canonical repo paths, validate with generated schemas, report compatibility errors, and reject unknown unsafe fields.
  - Output: Manifest loader with schema-version checks and human/JSON diagnostics.
  - Validation: Tests cover valid manifest, invalid JSON, incompatible version, unknown sensitive fields, missing fixture, and duplicate scenario id.

- **3.3 Implement deterministic run context.**
  - Design: Generate run ids, fixture seeds, logical clock controls, trace id roots, artifact paths, and workspace fingerprints deterministically where possible.
  - Output: `harness_run_context` used by every scenario and step.
  - Validation: Two reset/reseed runs produce identical fixture ids and compatible traces for Phase 0 smoke scenarios.

- **3.4 Implement stable runner lifecycle.**
  - Design: Model test runs as `planned`, `stack_starting`, `stack_ready`, `resetting`, `seeding`, `running`, `asserting`, `collecting_artifacts`, `passed`, `failed`, or `blocked`.
  - Output: Lifecycle recorder and status transitions tied to `test_run_record`.
  - Validation: Transition tests reject impossible transitions and prove blocked/failed paths still collect diagnostic artifacts where possible.

- **3.5 Implement CLI command surface.**
  - Design: Expose `overrid test integration`, `overrid test scenario <name>`, `overrid test list`, `overrid test reset`, and `overrid test artifacts <run-id>` through the Rust CLI.
  - Output: Initial CLI command handlers calling the harness crate.
  - Validation: CLI tests prove stable `--json` output, phase filtering, scenario listing, blocked dependency reporting, and artifact lookup.

## Phase 4: Local Stack Lifecycle, Reset, Seed, And Health Integration

### Work Items

- **4.1 Integrate local stack startup.**
  - Design: Call Local Development Stack lifecycle hooks to start or verify API, worker, native Overqueue-like durable state, event log, object/artifact stub, and node-agent simulator in dependency order.
  - Output: `harness.start_stack(profile)` with readiness wait and service health snapshot.
  - Validation: Tests cover clean start, already-running start, health timeout, port conflict, and degraded optional service state.

- **4.2 Integrate safe reset.**
  - Design: Reset only local/test-marked state using explicit stack profile and test-state markers; abort before seeding when cleanup is incomplete.
  - Output: `harness.reset_stack(profile)` with cleanup report and safety checks.
  - Validation: Tests prove reset cannot target non-local profiles, unmarked state, or production-like endpoints.

- **4.3 Integrate deterministic seed.**
  - Design: Seed tenants, actors, test keys, node simulators, manifests, workload refs, package refs, local ORU accounts, and policy contexts through public APIs where possible.
  - Output: `harness.seed(fixtures)` with seed result refs and fixture version metadata.
  - Validation: Reset/reseed produces identical ids and emits expected local/test markers.

- **4.4 Integrate logs, health, and event export.**
  - Design: Collect local logs, service readiness, Overwatch test-event exports, CLI outputs, API envelopes, and artifact refs by time window and trace id.
  - Output: Diagnostic collection helpers usable by assertions and artifact bundles.
  - Validation: Tests prove diagnostics include trace ids and reason codes but redact secrets and private payloads.

- **4.5 Build Phase 0 smoke orchestration.**
  - Design: Execute clean-checkout start, reset/reseed, fixture tenant and actor creation, test key use, signed no-op command, audit event write/read, invalid schema denial, and redacted artifact export.
  - Output: `phase0_smoke` scenario set and default integration suite.
  - Validation: Phase 0 smoke passes from a clean local profile and fails deterministically when schema or signature behavior regresses.

## Phase 5: Deterministic Fixture Library And Test-Only Identity Controls

### Work Items

- **5.1 Build core identity and tenant fixtures.**
  - Design: Provide typed fixtures for people, organizations, service accounts, system services, tenants, role bindings, quotas, suspension states, and test-only actors.
  - Output: Fixture builders with deterministic ids and schema-checked outputs.
  - Validation: Tests prove role, tenant, actor, and suspension variants trigger expected accept/deny behavior.

- **5.2 Build test key and signing fixtures.**
  - Design: Provide test signing key metadata, rotation, revocation, wrong-key, expired-key, and mismatched-tenant variants without storing raw private keys in artifacts.
  - Output: `fixture_key` builders and signer helpers scoped to local/test profiles.
  - Validation: Invalid signature, revoked key, wrong tenant, expired key, and fixture-key-outside-local scenarios fail closed.

- **5.3 Build workload, package, and resource fixtures.**
  - Design: Provide no-op command, synthetic workload, package, resource card, node simulator, queue item, lease placeholder, and runner-state fixtures that can expand from Phase 1 to Phase 3.
  - Output: Workload fixture library with phase tags and required-service declarations.
  - Validation: Tests prove Phase 1 synthetic work stops at pending queue state and does not pretend real execution exists.

- **5.4 Build accounting and policy fixtures.**
  - Design: Provide local/test ORU accounts, usage dimensions, receipt refs, budget-exhaustion variants, policy context facts, and denied-class fixtures without real payment or payout side effects.
  - Output: Phase 4/5-ready fixture modules gated behind owning-service availability.
  - Validation: Tests prove accounting fixtures are marked test usage and cannot reach external payment providers or provider payout flows.

- **5.5 Build fixture drift detection.**
  - Design: Compare fixture schemas, expected ids, generated refs, and golden trace refs across runs to catch nondeterministic fixture behavior.
  - Output: Fixture drift report with stable diff fields.
  - Validation: Repeated smoke runs detect changed fixture ids, missing refs, stale schema versions, and unexpected extra fixture records.

## Phase 6: Scenario Step Runners, Black-Box Paths, And Fail-Closed Results

### Work Items

- **6.1 Implement CLI step runner.**
  - Design: Execute Rust CLI commands with profile, environment, trace id, timeout, `--json`, expected exit class, and redacted stdout/stderr capture.
  - Output: `scenario_step` runner for CLI actions.
  - Validation: Tests cover successful command, schema denial, invalid signature, duplicate idempotency, timeout, missing tenant, and command not available in phase.

- **6.2 Implement SDK and API step runners.**
  - Design: Execute SDK calls and Overgate API requests through generated contracts, signed command envelopes, idempotency keys, trace ids, and stable error decoding.
  - Output: SDK/API step runner modules.
  - Validation: Tests prove direct service URLs, private storage reads, unsigned requests, and invalid response schemas are rejected.

- **6.3 Implement local helper step runner.**
  - Design: Support local-only helper actions for stack lifecycle, seed verification, artifact lookup, local event export, and diagnostic checks without using helper paths for service behavior assertions.
  - Output: Helper step runner with strict allowed action list.
  - Validation: Tests prove helper steps cannot mutate production-like state or bypass service APIs for normal acceptance assertions.

- **6.4 Implement result and reason-code assertions.**
  - Design: Assert API response envelopes, CLI output envelopes, event refs, reason codes, retry classes, idempotency behavior, and terminal states with schema-checked expected values.
  - Output: Assertion library shared across scenario runners.
  - Validation: Golden tests prove wrong reason code, missing audit ref, missing schema version, and wrong terminal state fail with actionable output.

- **6.5 Implement blocked versus failed classification.**
  - Design: Distinguish missing dependency, unsupported phase, unavailable service, and configuration errors from actual service regressions.
  - Output: Stable run status and exit classification for `blocked` and `failed` paths.
  - Validation: CI tests prove blocked runs do not count as false passes and failed runs retain failure artifacts.

## Phase 7: Golden Trace Assertions And Cross-Service Contract Coverage

### Work Items

- **7.1 Implement exact Phase 0/1 protocol traces.**
  - Design: Require exact ordered event sequences for signed command admission, schema denial, tenant/identity/key creation, audit write/read, idempotency, and pending queue transitions.
  - Output: Phase 0/1 golden trace set.
  - Validation: Tests catch missing audit events, reordered exact events, duplicate state transitions, and illegal pending-state shortcuts.

- **7.2 Implement partially ordered DAG trace checks.**
  - Design: Support later concurrent flows with required nodes, required causal edges, forbidden transitions, stable reason codes, schema versions, and allowed diagnostic extras.
  - Output: DAG trace assertion engine for Phase 3+ execution and later phase flows.
  - Validation: Tests catch missing causal edges, missing required nodes, illegal state transitions, and false assumptions about concurrent event ordering.

- **7.3 Add execution-loop trace coverage.**
  - Design: As Phase 3 services mature, assert queue, scheduler, lease, runner, result, retry, cancellation, timeout, dead-letter, usage, and audit paths.
  - Output: Execution-loop scenario and trace templates.
  - Validation: Successful, retryable failed, final failed, cancelled, timed-out, and dead-lettered workloads have distinct traces.

- **7.4 Add policy, verification, and dispute trace coverage.**
  - Design: As Phase 4 services mature, assert Overguard decisions, policy dry-run refs, Oververify evidence, Overclaim holds, challenge windows, and correction refs.
  - Output: Trust/policy scenario and trace templates.
  - Validation: Tests cover denied egress, insufficient trust, quota exhaustion, package trust failure, challenged provider, and disputed settlement paths.

- **7.5 Add usage, ledger, and receipt trace coverage.**
  - Design: As Phase 5 services mature, assert Overmeter rollups, ORU state transitions, Seal Ledger refs, holds, refunds/corrections, receipts, and local/test payment boundaries.
  - Output: Accounting scenario and trace templates.
  - Validation: Tests prove usage creates test-account transitions only and cannot reach external payment or payout side effects by default.

## Phase 8: Artifact Bundles, Redaction, Reproduction, And Flake Detection

### Work Items

- **8.1 Implement artifact collection.**
  - Design: Collect per-run logs, Overwatch exports, CLI outputs, API payload envelopes, service health snapshots, fixture versions, schema versions, and assertion diffs.
  - Output: Redacted artifact bundle directory with manifest and refs.
  - Validation: Failed runs leave enough evidence to reproduce stack, fixtures, scenario, trace id, and reason code.

- **8.2 Implement redaction scanner.**
  - Design: Scan artifacts for raw secrets, tokens, private keys, signatures, private payloads, decrypted RAG content, fixture key material, private file contents, and unsafe local paths.
  - Output: Redaction report attached to each artifact bundle.
  - Validation: Tests inject forbidden sentinel values and prove bundles fail export until redacted.

- **8.3 Implement reproduction commands.**
  - Design: Generate minimal commands for rerunning one scenario with the same profile, fixture manifest, scenario id, trace root when applicable, and artifact output path.
  - Output: Reproduction command in every failure bundle.
  - Validation: Reproduction tests rerun a failed scenario and preserve the original reason-code classification.

- **8.4 Implement flake detection fields.**
  - Design: Track repeated run count, timing variance, nondeterministic assertion markers, unstable event ordering, and tolerance-window use without marking nondeterminism as success.
  - Output: Flake metadata in run records and artifacts.
  - Validation: Repeated run tests detect nondeterministic ids, timing-sensitive assertions without tolerance rules, and unstable trace ordering.

- **8.5 Implement retention classes.**
  - Design: Apply compact retention for successful smoke runs, longer retention for failures and release candidates, and explicit retention metadata for phase-gate evidence.
  - Output: Retention policy fields and pruning hooks for local/CI artifacts.
  - Validation: Tests prove successful summaries, failure bundles, and release-candidate evidence receive distinct retention classes.

## Phase 9: Phase Gates, CI Selection, And Later-Phase Expansion

### Work Items

- **9.1 Implement phase and tag selection.**
  - Design: Let builders select scenario sets by master phase, service, tag, changed path, required dependency, gate class, or explicit scenario name.
  - Output: Scenario discovery and selection engine.
  - Validation: Tests prove Phase 0 selects only foundation scenarios, Phase 1 includes earlier gates plus control-plane spine, and later unavailable scenarios remain extended/planned.

- **9.2 Implement CI smoke suite.**
  - Design: Run a bounded smoke suite with stable exit codes, deterministic reset/reseed, artifact export, and failure classification suitable for clean-checkout CI.
  - Output: CI entrypoint for Phase 0 and early Phase 1 gates.
  - Validation: CI fixture tests prove success, blocked, and failed outcomes are distinguishable and machine-readable.

- **9.3 Implement service contract coverage reporting.**
  - Design: Report which SDS/service catalog APIs, events, schemas, and reason-code families have scenario and golden-trace coverage.
  - Output: Coverage report by service, phase, scenario, schema module, and event type.
  - Validation: Review gates reject a service marked integrated when required public contract scenarios are missing.

- **9.4 Implement product integration gates.**
  - Design: Add Phase 6 scenarios for Docdex, Mcoda, Codali, CLI, SDK, and admin UI workloads once their owning adapters and product contracts exist.
  - Output: Product scenario packs that exercise submit, status, result, cancellation, usage, receipt, and failure paths through public contracts.
  - Validation: Product tests prove jobs can be inspected and billed through Overrid rails without calling internal product APIs directly.

- **9.5 Implement governance and scale hardening gates.**
  - Design: Add later gate scenarios for grid-resident backbone, storage/namespace, deployment, federation, public low-sensitivity pool, native apps, governance, incident response, and compliance only when owning phase contracts exist.
  - Output: Planned gate map through master Phase 13.
  - Validation: Later-phase gate tests require earlier phase gates and do not weaken private workload or secret-bearing boundaries for convenience.

## Phase 10: Harness Validation, Documentation, And Downstream Handoff

### Work Items

- **10.1 Validate harness self-consistency.**
  - Design: Test manifest loading, fixture generation, local stack lifecycle, scenario execution, assertion records, golden traces, artifact export, redaction, and phase selection as one coherent harness.
  - Output: Harness self-test suite tied to SDS #3 validation items.
  - Validation: `docdexd run-tests` or the repo's canonical test command passes once test-runner configuration and implementation exist.

- **10.2 Validate alignment with tech-stack choices.**
  - Design: Audit implementation and docs for Rust-first harness ownership, JSON/JSON Schema manifest contracts, Overrid-shaped local substitutes, and no conventional cloud product boundaries.
  - Output: Tech-stack alignment checklist for the harness.
  - Validation: Scans and review find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions in harness scope.

- **10.3 Validate security and redaction.**
  - Design: Exercise fixture credentials, local/test profile checks, artifact redaction, environment separation, no-production-defaults, and direct-storage restrictions.
  - Output: Security test report tied to SDS #3 policy/security requirements.
  - Validation: Tests prove production endpoints are blocked by default and redacted bundles contain no raw secrets or private payloads.

- **10.4 Validate documentation links and progress evidence.**
  - Design: Keep SDS #3, service catalog entry, master plan, crosswalk, build-plan progress, queue state, and Docdex memory aligned with the created sub-build plan.
  - Output: Updated documentation references and recorded validation evidence.
  - Validation: Markdown links pass, queue state marks `003-build-plan` complete, and Docdex search returns this plan for SDS #3 queries.

- **10.5 Prepare downstream service handoff.**
  - Design: Document how future service builders add scenario manifests, fixtures, golden traces, artifact expectations, and phase-gate coverage before claiming service integration.
  - Output: Handoff rules for Phases 1 through 13 and service SDS authors.
  - Validation: Handoff review confirms the harness can grow with later services without changing master Phase 0 through Phase 13 order.

## Alignment Review

- The sub-build plan keeps Integration Test Harness first build work in master Phase 0, matching the SDS, service catalog entry, Phase 0 plan, and build-plan crosswalk.
- The plan treats the harness as a Rust-first development/release gate with language-neutral JSON/JSON Schema scenario and fixture contracts.
- The plan explicitly depends on Local Development Stack, Repository Layout, Shared Schema Package, CLI/SDK, Overgate, Overwatch, Overqueue, and later owning services without moving those later services earlier.
- The plan carries forward SDS #3 resolved decisions: Rust-owned orchestration, hybrid exact/DAG golden traces, Overrid-shaped local stubs, artifact retention classes, and mandatory phase-gate boundaries.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.
- The plan adds only a more detailed per-SDS implementation layer under `docs/build_plan`.

## Exit Gate

SUB BUILD PLAN #3 is complete when a builder can implement the Integration Test Harness as a Rust-first, deterministic, phase-gated local validation system that starts/resets/seeds the local stack, runs black-box scenario workflows, asserts schemas and golden traces, emits redacted reproduction artifacts, and expands through later master phases without violating tech-stack boundaries, test-only safety, auditability, or the canonical master build order.
