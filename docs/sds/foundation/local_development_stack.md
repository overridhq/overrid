SDS #4

# Local Development Stack SDS

## Purpose

Let developers run a complete minimal Overrid environment locally without external cloud services.

The local development stack is the executable foundation for Phase 0 and the test bed for later phases. It must provide repeatable process orchestration, deterministic reset/seed behavior, local-only fixtures, health checks, logs, and a safe way to exercise Overrid contracts before founder seed hardware or grid-resident services are required.

## Source Documents

| Source | Path |
| --- | --- |
| Sub-build plan | [sub_build_plan_004_local_development_stack.md](../../build_plan/sub_build_plan_004_local_development_stack.md) |
| Service implementation plan | [local_development_stack.md](../../service_catalog/foundation/local_development_stack.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 0: Foundation](../../build_plan/phase_00_foundation.md) |

## Service Family

- Family: Foundation and developer tooling.
- Owning layer: Local execution and developer environment.
- Primary data scope: local process definitions, port registry, environment variables, fixture seed data, local volumes, health checks, logs, and reset plans.
- First build phase from service plan: [Phase 0: Foundation](../../build_plan/phase_00_foundation.md).

## Problem Statement

Overrid cannot be built safely if each developer starts a different pile of databases, queues, mock nodes, and fixtures. Phase 0 requires a clean checkout to start an API process, worker process, queue/durable job table, database, object/artifact store stub, one node-agent simulator, and test identity/tenant fixtures. Without a deterministic local stack, integration tests will be flaky and service contracts will drift.

## Goals

- Start the minimal Overrid stack from a clean checkout.
- Run without external cloud services or paid SaaS dependencies.
- Provide deterministic reset and reseed for integration tests.
- Expose health/readiness checks for every local process.
- Keep local credentials, fixtures, and bypasses visibly test-only.
- Provide logs, event output, and job-state inspection good enough for Phase 0/1 debugging.
- Mirror production contract shapes without pretending local stubs are production services.

## Non-Goals

- Do not become a production deployment system.
- Do not require founder hardware, public providers, external cloud accounts, real payment providers, or real model providers.
- Do not use local-only shortcuts in production paths.
- Do not store real secrets in committed files.
- Do not make service-specific mocks that bypass shared schemas.
- Do not split the first system into more processes than needed to prove the contracts.

## Primary Actors And Clients

- Developers implementing Phase 0 through Phase 6.
- Integration Test Harness controlling stack lifecycle.
- CLI and SDK smoke tests.
- Service implementers inspecting API, worker, queue, event, and node-agent simulator behavior.
- CI jobs running bounded local smoke suites.

## Dependencies

- Repository Layout for folder structure and root commands.
- Shared Schema Package for request, response, event, manifest, and error validation.
- Integration Test Harness for scenario execution and artifact capture.
- CLI for developer-facing stack commands once available.
- Initial control-plane API and worker modules.
- Local database, queue/durable job table, object/artifact storage stub, and node-agent simulator.

The default stack should use open-source/self-hosted components where concrete technology is needed. The SDS defines required capabilities; implementation may choose the simplest local component that preserves the contract.

## Owned Responsibilities

Local Development Stack is responsible for:

- Defining local service topology and startup order.
- Defining environment profiles, ports, volumes, and generated test secrets.
- Providing `start`, `stop`, `status`, `reset`, `seed`, `logs`, and `smoke` behavior.
- Ensuring reset removes local state and seed recreates deterministic fixtures.
- Providing readiness checks for API, worker, database, queue/job table, object store stub, event log, and node-agent simulator.
- Keeping local fixture credentials from being accepted by non-local environments.
- Producing operator-readable local diagnostics.

It does not own production deployment, package release, provider onboarding, or grid scheduling.

## Data Model

The first implementation should define:

- `stack_profile`: named local profile with enabled services, ports, volumes, fixture set, feature flags, and environment marker.
- `service_definition`: local process/container definition with command, working directory, env refs, health check, log target, dependencies, and shutdown behavior.
- `port_registry`: reserved local ports and collision handling for API, worker metrics, database, queue, object store stub, and node-agent simulator.
- `local_env_manifest`: generated environment variables, redaction rules, and `.env.local`/`.env.example` separation.
- `volume_registry`: named local state volumes/directories for database, queue, object store, event log, logs, and artifacts.
- `reset_plan`: ordered cleanup operations with test-state markers to avoid deleting non-test data.
- `seed_manifest`: deterministic tenant, actor, key, node, manifest, package, workload, and ledger fixture references.
- `health_snapshot`: current readiness, liveness, version, schema version, dependency status, and last error by service.
- `local_secret_record`: generated secret metadata with `local_only` and `test_only` markers; raw secret content must stay out of docs and logs.

Every local stack record that appears in test artifacts should include stack profile, trace id where applicable, schema version, and fixture version.

## API Surface

The stack surface is command and local lifecycle behavior:

- `overrid dev start`: start the configured local stack.
- `overrid dev stop`: stop local services without deleting state.
- `overrid dev restart`: restart services in dependency order.
- `overrid dev status`: print health and readiness state.
- `overrid dev reset`: delete marked local test state.
- `overrid dev seed`: recreate deterministic fixtures.
- `overrid dev smoke`: run the Phase 0 smoke path.
- `overrid dev logs`: stream or export redacted local logs.
- `overrid dev doctor`: check ports, dependencies, versions, file permissions, and fixture markers.

Required local service endpoints:

- API health: `GET /healthz`
- API readiness: `GET /readyz`
- Worker health/readiness equivalent.
- Local event/audit query endpoint for tests.
- Node-agent simulator health endpoint.
- Object/artifact stub health endpoint if implemented as a separate process.

API requirements:

- Local commands must fail when the selected profile is not marked local/test.
- Reset must require explicit local profile and test-state markers.
- Seed must be idempotent after reset.
- Health checks must distinguish not-started, starting, ready, degraded, and failed.
- Logs and diagnostics must redact local secrets and fixture key material.

## Event Surface

Local stack events are local diagnostics and test artifacts. They should not be treated as production Overwatch events unless emitted by the running Overrid services through normal APIs.

Local diagnostic events:

- `local_stack.start_requested`
- `local_stack.service_starting`
- `local_stack.service_ready`
- `local_stack.reset_started`
- `local_stack.seed_started`
- `local_stack.seed_completed`
- `local_stack.smoke_started`
- `local_stack.smoke_completed`
- `local_stack.failed`

The stack must also allow the Integration Test Harness to inspect real Overwatch events emitted by local Overrid services during scenarios.

## Core Workflow

1. Developer or harness selects local stack profile.
2. Stack command checks prerequisites, ports, runtime availability, and test-only markers.
3. Stack starts database, queue/job table, object/artifact stub, API, worker, and node-agent simulator in dependency order.
4. Readiness checks wait until all required services are ready.
5. Reset command clears marked local volumes/tables and preserves configuration.
6. Seed command creates deterministic tenant, actor, key, node, manifest, workload, and package fixtures.
7. Smoke command submits a signed no-op command, writes/reads audit event, and rejects an invalid schema payload.
8. Logs and health snapshots are available for diagnostics and integration test artifacts.

## State Machine

Stack lifecycle:

1. `uninitialized`: local config or runtime prerequisites are missing.
2. `configured`: profile, ports, env, and service definitions are valid.
3. `starting`: services are being launched in dependency order.
4. `ready`: all required health/readiness checks pass.
5. `degraded`: one optional service failed or is unhealthy.
6. `resetting`: marked local state is being removed.
7. `seeding`: deterministic fixtures are being recreated.
8. `running_tests`: integration or smoke tests are using the stack.
9. `stopping`: services are shutting down.
10. `stopped`: services are not running but config remains.
11. `failed`: required service failed or health checks timed out.

Fixture lifecycle:

1. `declared`
2. `seed_pending`
3. `seeded`
4. `verified`
5. `reset`

## Policy And Security

- Bind local services to loopback by default.
- Generate test-only secrets at local setup time; never commit raw secret values.
- Mark fixture tenants, identities, keys, nodes, and ledger accounts as local/test.
- Prevent local fixture credentials from being used against seed, staging, production-like, federation, or public-provider endpoints.
- Keep local bypasses behind explicit local profile checks.
- Use `.env.example` for names and `.env.local` or generated secret stores for values.
- Redact secrets, tokens, signatures, private payloads, and encrypted content from logs.
- Fail reset unless the target volume/database carries a test-state marker.
- Keep object/artifact storage stubs isolated from user personal directories.

## Metering And Accounting

The local stack should model usage without creating real billing behavior:

- Generate test usage events through Overmeter once the phase supports it.
- Use local/test ORU and Seal Ledger accounts only.
- Mark all local usage as non-settling test usage.
- Ensure no external payment, provider payout, or marketplace operation can run in default local mode.
- Provide fixture limits for budget exhaustion and quota denial tests.

## Observability And Operations

The stack should expose:

- Service health and readiness table.
- Port and process/container status.
- Schema/API compatibility versions.
- Fixture manifest version.
- Last reset and last seed timestamps.
- Local event/audit query output.
- Queue/job-table depth and worker status.
- Log streaming and redacted log bundle export.
- Startup timing and dependency wait diagnostics.

## Failure Modes And Recovery

- Port collision: fail before partial startup and show conflicting port/process.
- Runtime missing: show prerequisite and setup command.
- Service health timeout: stop dependent services and export logs.
- Reset safety check fails: abort without deleting state.
- Seed partial failure: show fixture id, owning service, trace id, and rollback/reset guidance.
- Schema version mismatch: block smoke tests until schemas and services align.
- Local secret missing: regenerate only local/test secrets, never guess production values.
- Node-agent simulator unavailable: mark execution scenarios blocked but keep control-plane smoke available.

## Validation Plan

Service-plan validation:

- A clean checkout can start the local stack.
- Reset removes local state and reseed recreates deterministic fixtures.
- Smoke test creates a tenant, actor, command, and audit event.

Additional SDS-level validation:

- `overrid dev start` reaches ready state from a clean checkout.
- `overrid dev status` reports every required service and schema version.
- `overrid dev reset && overrid dev seed` produces identical fixture ids across runs.
- `overrid dev smoke` proves signed command path, valid audit event, invalid schema denial, and trace id propagation.
- Reset refuses to delete state without test-state markers.
- Fixture credentials fail against non-local profiles.
- Logs and artifact bundles contain no raw secrets.
- Integration Test Harness can start, reset, seed, and run Phase 0 smoke through documented commands.

## Build Breakdown

1. Define local stack profile, service definition, port registry, local env manifest, volume registry, reset plan, seed manifest, and health snapshot schemas.
2. Add root commands or CLI shims for start, stop, status, reset, seed, logs, doctor, and smoke.
3. Start minimal database, queue/job table, object/artifact stub, API process, worker process, and node-agent simulator.
4. Add generated local/test secrets and fixture keys with environment guards.
5. Add deterministic fixtures for tenant, actor, key, node, manifest, workload, and package.
6. Add health/readiness checks and log export.
7. Add Phase 0 smoke test integration.
8. Add CI-friendly bounded startup and artifact export.

## Phase-Gate Boundary Decisions

Phase 1 of [SUB BUILD PLAN #4 - Local Development Stack](../../build_plan/sub_build_plan_004_local_development_stack.md) freezes the local-stack gate semantics before runtime stack implementation begins.

Gate states:

- `buildable_phase_0`: Phase 0 local lifecycle, deterministic reset/seed, local/test fixtures, loopback health, diagnostics, and smoke are the first build point.
- `local_smoke_prerequisite`: Phase 1 may consume local fixtures and lifecycle support for the first signed tenant-scoped control-plane smoke path.
- `owning_service_required`: Phase 2 node identity and Phase 3 execution-loop hooks require owning Overcell, Overpack, Oversched, Overlease, Overrun, and Overmeter contracts before local simulators can expand.
- `planned_disabled`: Later service families remain documented but disabled in the local stack until their owning SDS/API contracts define local-test behavior.
- `not_local_stack_owned`: Governance, compliance, migration, incident, and stewardship workflows are not local-stack-owned behavior.

Boundary rules:

- The local stack is not a deployment orchestrator, not a production control plane, not a payment runner, not a public-provider testbed, and not a shortcut around service contracts.
- Direct storage, queue, object, event, or local file inspection is allowed only for reset, seed, health, and diagnostics.
- PostgreSQL, Redis, NATS, Kafka, S3, MinIO, Vault, blockchain, NFT, pricing, revenue, and customer-count assumptions are rejected as local-stack product boundaries.
- New local-stack behavior must update the owning service SDS/API first, then shared schemas when serialized, then the sub-build plan, service catalog, harness scenarios, and validation evidence.

Resolved decisions carried into Phase 1:

- Rust-owned embedded/local durable state behind Overbase-shaped and Overqueue-shaped contracts.
- Filesystem-backed content-addressed Overstore stub with BLAKE3/content hashes, object manifests, local/test markers, deterministic reset, and redacted artifact bundles.
- Exactly one local Overcell-like node simulator before Phase 2, limited to deterministic identity, heartbeat, capability report, health endpoint, fixture workload acceptance, and no-op handoff.
- Deterministic loopback ports `18080`, `18081`, `18082`, `18083`, `18084`, and optional developer UI `18085`.
- Reproducible Linux x86_64 clean-checkout CI target with repository-pinned Rust tooling, loopback networking, no external database/queue/object-store services, no ambient keychain, and no cloud credentials.

## Handoff And Downstream Use

Phase 1 consumes the local stack to build the first signed control-plane path. The Integration Test Harness consumes it as the lifecycle manager for scenario tests.

Later phases should add local services or simulators only when they preserve production contract shape and remain clearly marked as local/test.

## Open Design Questions

Resolved decisions:

- The first local database and queue/job-table implementation should be a Rust-owned embedded/local durable state layer behind Overrid abstractions. It should store Overbase-shaped records, Overqueue-style durable job records, event refs, schema versions, fixture markers, and reset metadata without depending on PostgreSQL, Redis, NATS, Kafka, or similar products as the development boundary. Candidate embedded engines may be benchmarked internally, but Phase 0 code should expose only Overrid local-state and Overqueue contracts.
- Object storage should start as a filesystem-backed, content-addressed Overstore stub, not an S3-compatible local service. The stub should use BLAKE3/content hashes, object manifests, local/test markers, deterministic reset, upload/download grant checks where applicable, and redacted artifact bundles. S3-compatible tools may be useful later for import/export or benchmark adapters, but they should not define Phase 0 object semantics.
- Before Phase 2 hardware registration, the node-agent simulator should implement exactly one local Overcell-like node with deterministic identity, heartbeat, capability report, health endpoint, fixture workload acceptance, and no-op execution handoff sufficient for Phase 0 smoke and Phase 1 pending-work tests. It should not perform real hardware discovery, GPU runtime integration, benchmark publication, installer/update flows, remote shell behavior, or provider eligibility decisions until the Phase 2 Overcell, Hardware Discovery, Benchmark Runner, and Node Installer contracts exist.
- The default local port registry should bind loopback only and reserve a deterministic Phase 0 range: API `18080`, worker health/metrics `18081`, node-agent simulator `18082`, object/artifact stub `18083`, local event/audit query `18084`, and optional developer UI `18085`. Embedded state and job-table implementations should not open network ports by default. Startup must check all reserved ports before launching any service, fail before partial startup on collision, print the conflicting port/process when known, and require explicit local profile overrides rather than silently shifting ports.
- The reference clean-checkout CI target should be a reproducible Linux x86_64 local/test runner image, equivalent to Ubuntu 24.04 LTS with the repository-pinned Rust toolchain, loopback networking, no external database/queue/object-store services, no ambient persistent keychain, and no cloud credentials. macOS and other developer hosts remain supported targets, but the clean-checkout gate should pass in this Linux runner using only `dev:start`, `dev:reset`, `dev:seed`, `dev:smoke`, `schema:check`, `layout:check`, `docs:check`, and the integration harness.
