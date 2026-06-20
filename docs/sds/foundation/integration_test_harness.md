SDS #3

# Integration Test Harness SDS

## Purpose

Prove Overrid behavior across service boundaries instead of relying only on unit tests.

The integration test harness is a development and release gate, not a production service. It must start/reset the local stack, seed deterministic fixtures, run scenario workflows, capture golden event traces, and produce failure artifacts that make cross-service regressions diagnosable.

## Source Documents

| Source | Path |
| --- | --- |
| Sub-build plan | [sub_build_plan_003_integration_test_harness.md](../../build_plan/sub_build_plan_003_integration_test_harness.md) |
| Service implementation plan | [integration_test_harness.md](../../service_catalog/foundation/integration_test_harness.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 0: Foundation](../../build_plan/phase_00_foundation.md) |

## Service Family

- Family: Foundation and developer tooling.
- Owning layer: Quality, contract, and integration validation.
- Primary data scope: fixture manifests, scenario definitions, test run records, golden traces, failure artifacts, local stack lifecycle refs, and validation reports.
- First build phase from service plan: [Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Problem Statement

Overrid depends on signed commands, tenant boundaries, idempotency, append-only events, schema validation, queue state, execution state, metering, policy, and accounting. Those guarantees only matter if they survive service-to-service behavior. A unit-test-only strategy would miss broken trace propagation, stale fixture state, invalid audit chains, non-deterministic retries, and services that bypass shared schemas.

## Goals

- Provide deterministic end-to-end tests from a clean checkout.
- Start, reset, seed, and inspect the local stack without external cloud services.
- Build reusable fixtures for tenants, identities, keys, nodes, manifests, workloads, packages, ledger accounts, and policy contexts.
- Capture golden event traces for signed requests, denials, state transitions, retries, cancellations, timeouts, stale leases, usage, receipts, and disputes as phases mature.
- Produce failure reports with logs, events, traces, seed data ids, reason codes, and reproduction commands.
- Make every service implementation plan prove its contract through this harness.

## Non-Goals

- Do not run against production or public-provider resources by default.
- Do not create real external payment, billing, marketplace, or provider-payout side effects.
- Do not rely on nondeterministic fixture ids, wall-clock-only assertions, or hidden order dependencies.
- Do not store real secrets, raw private keys, or private user data in fixtures.
- Do not become a substitute for unit tests, static checks, schema tests, or security reviews.
- Do not weaken platform rules to make tests easier.

## Primary Actors And Clients

- Service implementers validating a phase locally.
- CI jobs running deterministic cross-service smoke and regression suites.
- Release operators checking whether a phase can advance.
- Future package validators that need executable contract examples.
- Incident responders reproducing a failure path from captured artifacts.

## Dependencies

- Local Development Stack for process lifecycle, reset, seed, and health checks.
- Repository Layout for test folder conventions and root test commands.
- Shared Schema Package for request/response/event/error validation.
- CLI and SDK for black-box command-path testing.
- Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, and Overqueue for Phase 1 scenarios.
- Later phase services for execution, metering, policy, accounting, adapters, and native app scenarios.

The harness should depend on public contracts and local test hooks only. Direct database inspection is allowed only for fixture reset/diagnostics and must not become a service behavior assertion unless the owning service SDS explicitly defines storage semantics.

## Owned Responsibilities

Integration Test Harness is responsible for:

- Defining fixture manifests and deterministic id generation.
- Controlling local stack lifecycle during tests.
- Providing scenario definitions and step runners.
- Validating API responses, events, command output, and audit chains against shared schemas.
- Capturing logs, traces, events, artifacts, seed ids, and timing data.
- Producing reproducible failure reports.
- Enforcing phase gates and preventing services from bypassing shared schemas.

The harness does not own production data and does not emit production Overwatch events except by exercising platform APIs in a local/test environment.

## Data Model

The first implementation should define:

- `fixture_manifest`: named fixture set with deterministic seed, tenant ids, actor ids, key refs, node ids, package ids, workload ids, resource cards, and expected schema versions.
- `fixture_identity`: typed person, organization, node, app, service account, native service, or system-service identity used in tests.
- `fixture_key`: deterministic test signing key metadata with clear `test_only` marker and rotation/revocation fixtures.
- `scenario_manifest`: ordered scenario with name, phase, dependencies, setup fixtures, steps, assertions, expected events, and cleanup rules.
- `scenario_step`: one CLI/API/helper action with input refs, timeout, retry expectations, and expected result class.
- `test_run_record`: run id, scenario ids, git/workspace fingerprint when available, stack profile, started/ended timestamps, status, and artifact refs.
- `golden_trace`: expected event sequence with event type, schema version, subject refs, state transition, reason code, and redaction expectations.
- `assertion_result`: pass/fail record with field path, expected value, actual value, reason code, and linked artifact.
- `artifact_bundle`: redacted logs, Overwatch exports, CLI output, API payloads, stack health, and reproduction command.

Fixture ids should be deterministic across clean resets. Time-sensitive assertions should use bounded logical clocks or explicit tolerance windows.

## API Surface

The harness surface is a test runner and helper API:

- `overrid test integration`: run the default integration suite for the current phase.
- `overrid test scenario <name>`: run one scenario by name.
- `overrid test list`: list available scenarios, tags, required services, and phase alignment.
- `overrid test reset`: reset local test state and reseed fixtures.
- `overrid test artifacts <run-id>`: show or export a run's failure artifacts.
- `harness.start_stack(profile)`: start local services and wait for readiness.
- `harness.reset_stack(profile)`: clear database, queue, object store, event log, and local artifacts.
- `harness.seed(fixtures)`: create deterministic fixture records through public APIs where possible.
- `harness.run_scenario(manifest)`: execute steps and assertions.
- `harness.export_trace(run_id)`: export Overwatch/audit trace plus CLI/API outputs.

API requirements:

- Scenario manifests must be schema checked before running.
- Test runs must fail closed when required services are missing.
- Fixture setup must be idempotent after reset.
- Artifacts must redact secrets and private payload content.
- Test helpers should not know private implementation details unless clearly marked as reset/diagnostic-only.

## Phase-Gate Boundary Decisions

The harness gate classes are `smoke`, `contract_spine`, `regression`, `extended`, and `release_candidate`. Phase 0 and Phase 1 mandatory gates stay limited to the current phase contract spine plus all earlier mandatory gates; later execution, policy, accounting, federation, native app, and governance flows remain planned or extended until their owning service phases exist.

Missing local stack profiles, missing schemas, unavailable services, wrong phase tags, missing scenario or fixture manifests, unsafe non-local profiles, non-test fixture credentials, and not-yet-implemented later-phase contracts must produce `blocked` test runs with stable reason codes such as `dependency.local_stack_unavailable`, `dependency.schema_missing`, `dependency.service_unavailable`, `dependency.phase_tag_unsupported`, `dependency.manifest_missing`, `safety.non_local_profile`, `safety.fixture_not_test_only`, and `dependency.phase_contract_not_ready`.

Every service SDS or service catalog plan that adds a public API, event, signed command envelope, schema, reason-code family, or externally visible state transition must add or update Integration Test Harness scenario coverage before claiming integration readiness.

## Event Surface

The harness observes and asserts platform events; it should not write platform events directly except through normal API calls.

Harness-local event/report types:

- `test_run.started`
- `test_run.stack_ready`
- `test_run.fixtures_seeded`
- `test_run.scenario_started`
- `test_run.step_passed`
- `test_run.step_failed`
- `test_run.golden_trace_matched`
- `test_run.artifacts_collected`
- `test_run.completed`

Platform event assertions start with Phase 0/1 events such as request accepted/denied, identity created, tenant created, key created/revoked, manifest accepted, queue item created, and state transition applied. Later phases add lease, execution, policy, usage, ledger, dispute, and adapter events.

## Core Workflow

1. Select scenario set by phase, tags, changed services, or explicit command.
2. Start or verify the local stack.
3. Reset state and seed deterministic fixtures.
4. Run scenario steps through CLI, SDK, or public APIs.
5. Capture response envelopes, CLI outputs, audit events, logs, health snapshots, and object refs.
6. Validate schemas, state transitions, reason codes, idempotency behavior, and golden traces.
7. Export redacted artifacts and reproduction commands for failures.
8. Return stable exit code and summary.

## State Machine

Test run lifecycle:

1. `planned`: scenario set and required services are selected.
2. `stack_starting`: local stack is starting or being verified.
3. `stack_ready`: health checks passed.
4. `resetting`: old local test state is being cleared.
5. `seeding`: deterministic fixtures are being created.
6. `running`: scenario steps are executing.
7. `asserting`: schema, state, event, and artifact assertions are running.
8. `collecting_artifacts`: logs, traces, payloads, and diagnostics are being exported.
9. `passed`: all assertions passed.
10. `failed`: one or more assertions failed with artifacts.
11. `blocked`: required service or stack dependency is unavailable.

Fixture lifecycle:

1. `defined`
2. `seeded`
3. `used`
4. `verified`
5. `reset`

## Policy And Security

- Mark all fixture credentials as `test_only` and prevent them from being accepted outside local/test profiles.
- Block production endpoints unless a separate, explicit, guarded smoke mode is designed later.
- Redact private payloads, signatures, tokens, raw keys, encrypted RAG content, and provider secrets from artifacts.
- Keep network egress disabled by default for local integration tests unless a scenario explicitly requires a local-only stub.
- Separate test-only bypasses from production code paths with visible compile/runtime guards.
- Fail if a service accepts invalid schema payloads, missing tenant context, invalid signatures, or unauthorized fixture actors.
- Ensure cleanup does not delete non-test state by requiring stack profile and test-state markers.

## Metering And Accounting

The harness should verify accounting behavior without creating real charges:

- Use local/test ORU accounts and ledger fixtures.
- Mark all generated usage as test usage.
- Assert that Phase 5+ scenarios create usage rollups, ledger entries, holds, corrections, or receipts through test accounts only.
- Verify budget exhaustion and quota denial paths with fixture limits.
- Ensure no external payment provider or real provider payout call is reachable from default integration tests.

## Observability And Operations

The harness should expose:

- Test run id, scenario ids, fixture manifest version, and stack profile.
- Per-step duration, retry count, and timeout classification.
- Service health snapshot before and after run.
- Overwatch trace export for each scenario.
- CLI/API output capture with redaction.
- Log bundle by service and time window.
- Artifact directory path and reproduction command.
- Flake detector fields: repeated run count, nondeterministic assertion markers, and timing variance.

## Failure Modes And Recovery

- Stack cannot start: mark run blocked and export service startup logs.
- Reset incomplete: abort before seeding to avoid contaminated assertions.
- Fixture seeding fails: show fixture id, owning service, trace id, and reason code.
- Golden trace mismatch: show missing, extra, or reordered events with schema versions.
- Timeout: classify as startup, API, queue, execution, or assertion timeout.
- Idempotency regression: show original and retried command ids/outcomes.
- Artifact collection failure: still return primary failure with partial artifact refs.
- Flaky assertion: mark as failure unless a deterministic tolerance rule exists in the scenario manifest.

## Validation Plan

Service-plan validation:

- Tests are deterministic.
- A failed integration test leaves enough logs and event records for diagnosis.
- The harness can test success, retry, cancellation, timeout, policy denial, and stale lease behavior.

Additional SDS-level validation:

- Running from a clean checkout starts the local stack and passes the Phase 0 smoke scenario.
- Reset/reseed produces identical fixture ids across runs.
- Invalid schema, invalid signature, duplicate idempotency key, and missing tenant scenarios fail with expected reason codes.
- Golden trace matching catches missing audit events and illegal state transitions.
- Artifact bundles contain trace ids and reason codes but no raw secrets.
- Scenario manifests can be filtered by phase and required service.
- CI can run a bounded smoke suite and return stable exit codes.

## Build Breakdown

1. Define fixture manifest, scenario manifest, run record, golden trace, assertion result, and artifact bundle schemas.
2. Build local-stack lifecycle helpers for start, health, reset, seed, and teardown.
3. Build Phase 0 smoke scenario: start stack, create fixture tenant, create fixture actor, submit signed no-op command, write/read audit event, reject invalid schema.
4. Add CLI command-path tests for signing, idempotency, trace id propagation, and `--json` output.
5. Add Phase 1 synthetic workload scenario through tenant, identity, key, manifest, Overgate, Overwatch, and Overqueue.
6. Add failure-path scenarios for invalid signature, duplicate idempotency, missing tenant, policy denial, timeout, cancellation, stale lease, and retry as backing services mature.
7. Add artifact export and reproduction command generation.
8. Add phase-gate tags and CI selection.

## Handoff And Downstream Use

Every later service SDS should define validation in terms the integration harness can execute or assert. The harness becomes the practical gate for moving from one build phase to the next.

When a new service contract is added, update its scenario manifests and golden traces before declaring the service integrated.

## Open Design Questions

Resolved decisions:

- The Rust integration harness should own orchestration even if later client or UI code uses TypeScript or other languages. The canonical runner lives in the Rust Cargo workspace as shared test utilities plus `overrid test ...` CLI commands. Scenario manifests, fixture manifests, golden traces, and artifact bundle schemas remain language-neutral JSON/JSON Schema contracts from the shared schema package. Non-Rust checks can run as harness steps or adapters, but they should not become the authoritative phase-gate runner.
- Golden traces should use a hybrid assertion model. Phase 0/1 protocol-spine traces must be exact ordered sequences for signed command admission, schema denial, identity or tenant creation, audit write/read, idempotency, and pending queue transitions. Later concurrent flows should use partially ordered DAG assertions with required nodes, required causal edges, stable reason codes, schema versions, and forbidden illegal transitions, while allowing extra diagnostic events that do not change the platform state contract.
- The first local substitutes should be Overrid-shaped Rust local stubs or embedded engines behind Overrid-owned abstractions, not PostgreSQL, Redis, S3, MinIO, NATS, Kafka, or other external product boundaries. Phase 0 may use an embedded/local durable state layer for Overqueue-style job records and event refs, and a content-addressed filesystem-backed Overstore stub with BLAKE3 hashes, manifests, local/test markers, and deterministic reset. These substitutes must preserve final Overqueue/Overstore contract shapes and remain replaceable as native services mature.
- CI should retain artifacts by class. Successful smoke runs keep compact summaries, schema versions, trace ids, and fixture versions for short inspection windows. Failures keep redacted logs, payload envelopes, event traces, health snapshots, and reproduction commands long enough for debugging. Phase-gate and release-candidate runs keep redacted evidence bundles longer than ordinary branch runs. Artifact pruning must be automated, retention metadata should live on the run record, and no raw secrets, private payloads, signatures, encrypted RAG content, or fixture key material may be uploaded.
- Mandatory phase gates are the current phase's contract-spine scenarios plus all earlier phase gates. Phase 0 gates are clean checkout start, reset/reseed determinism, fixture tenant and actor creation, test key use, signed no-op command, audit event write/read, invalid schema denial, stable exit codes, and redacted artifact export. Phase 1 adds tenant, identity, key, manifest, Overgate, Overwatch, and Overqueue pending-work scenarios, including idempotency, invalid signature, duplicate command, and missing tenant denials. Later phases add only the minimal gate scenarios needed to prove their spine, while cancellation, timeout, stale lease, retry, policy denial, metering, ledger, product-integration, concurrency, failover, federation, public-provider, and native-app flows can start as extended regression suites until their owning phase makes them gate requirements.
