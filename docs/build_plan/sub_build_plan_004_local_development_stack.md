# SUB BUILD PLAN #4 - Local Development Stack

Attached SDS: [docs/sds/foundation/local_development_stack.md](../sds/foundation/local_development_stack.md)

## Purpose

This sub-build plan turns SDS #4 into an implementation sequence for the Local Development Stack. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

The Local Development Stack is the executable Phase 0 foundation. It must start a minimal Overrid environment from a clean checkout, keep every fixture and secret visibly local/test, provide deterministic reset and seed behavior, expose health and redacted diagnostics, and give the Integration Test Harness a stable lifecycle surface without becoming a production deployment system.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #4: Local Development Stack](../sds/foundation/local_development_stack.md) | Controls local stack purpose, topology, commands, data model, safety rules, validation, and resolved open-question decisions. |
| [Local Development Stack service plan](../service_catalog/foundation/local_development_stack.md) | Controls service-catalog objective, first build phase, dependencies, development order, contracts, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Controls the first implementation point for the local stack, deterministic fixtures, shared schemas, and smoke tests. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps Local Development Stack aligned to master Phase 0 as the first build phase. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires loopback-only local tooling with Rust-owned Overrid-shaped stubs or embedded engines, not conventional cloud product boundaries. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phase 0 | Attach SDS #4 to the build-plan layer and freeze local-stack boundaries before implementation. |
| 2 | Master Phase 0 | Define local stack schemas for profiles, services, ports, env, volumes, reset, seed, health, and local secrets. |
| 3 | Master Phase 0 | Build the Rust local-stack crate and command surface consumed by CLI, harness, and CI. |
| 4 | Master Phase 0 | Establish loopback topology, deterministic ports, generated env, and test-only secret guards. |
| 5 | Master Phase 0 with Phase 1 contract readiness | Build embedded/local Overbase-shaped state, Overqueue-shaped jobs, and Overstore-shaped artifacts behind Overrid abstractions. |
| 6 | Master Phase 0 | Implement lifecycle orchestration, readiness waits, service supervision, and health snapshots. |
| 7 | Master Phase 0 and Phase 1 | Implement deterministic reset, seed, fixtures, and signed smoke prerequisites for the first control-plane path. |
| 8 | Master Phase 0 with handoff to Phases 1, 2, and 3 | Add node-agent simulator and smoke execution hooks that later expand into real node and workload contracts. |
| 9 | Master Phase 0 with later use in all phases | Add redacted diagnostics, logs, artifact bundles, clean-checkout CI behavior, and flake evidence. |
| 10 | Master Phase 0 with handoff to Phases 1 through 13 | Validate local-stack safety, document extension rules, and prevent local shortcuts from leaking into later phases. |

## Tech Stack Guardrails

- The authoritative local-stack implementation belongs in the Rust Cargo workspace as local tooling plus `overrid dev ...` command handlers.
- Local stack records, fixture manifests, reset plans, seed manifests, health snapshots, and diagnostic bundles must use canonical JSON/JSON Schema contracts owned by the shared schema package.
- Local database, queue, and object-store capabilities must be Overrid-shaped embedded engines or local stubs behind Overbase, Overqueue, and Overstore-style abstractions.
- PostgreSQL, Redis, NATS, Kafka, S3, MinIO, Vault, blockchain, NFT, pricing, revenue, and customer-count assumptions are not part of Phase 0 local-stack scope.
- Services must bind to loopback by default and fail before startup on reserved-port collisions instead of silently shifting ports.
- Test secrets, fixture credentials, fixture tenants, fixture nodes, local ORU accounts, and local ledger refs must be marked `local_only` and `test_only`.
- Fixture credentials must fail against seed, staging, production-like, federation, public-provider, or non-local profiles.
- The stack may expose local diagnostics and test artifacts, but production Overwatch events must still be emitted by real Overrid services through normal APIs.

## Phase 1: SDS Attachment, Boundary, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #4.**
  - Design: Link this document from the numbered SDS, service catalog entry, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/foundation/local_development_stack.md`, `docs/service_catalog/foundation/local_development_stack.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #4 returns both the SDS and this sub-build plan.

- **1.2 Freeze the non-production stack boundary.**
  - Design: Record that the local stack is a development, CI, and harness surface, not a deployment orchestrator, production control plane, payment runner, public-provider testbed, or shortcut around service contracts.
  - Output: Boundary guardrails documented in this plan and referenced by every later phase.
  - Validation: Architecture review confirms no work item requires production endpoints, external cloud accounts, public-provider resources, real payment providers, or direct service-storage shortcuts.

- **1.3 Define first-build and expansion gates.**
  - Design: Treat master Phase 0 as the build point for local lifecycle, fixtures, smoke, and harness integration. Treat later phase capabilities as opt-in local simulators or service additions that preserve owning-service contracts.
  - Output: Phase-gate matrix covering Phase 0 foundation, Phase 1 control-plane smoke, Phase 2 node identity, Phase 3 execution loop, and later disabled/planned local services.
  - Validation: Review confirms this plan does not move Phase 1 through Phase 13 services earlier than the master build plan.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve decisions for native embedded/local state, filesystem-backed content-addressed Overstore stub, exactly one local Overcell-like node simulator, deterministic loopback port range, and reproducible Linux clean-checkout CI target.
  - Output: Implementation checklist tied to the SDS resolved decisions.
  - Validation: Review proves the plan does not re-open or contradict the SDS resolved decisions.

- **1.5 Define documentation-update rules.**
  - Design: When a local profile gains a new service, simulator, fixture family, or smoke scenario, update the owning service SDS/API first when applicable, then update this sub-build plan and harness scenarios.
  - Output: Cross-document maintenance rule for local-stack expansion.
  - Validation: Review checklist rejects local-stack additions that invent service behavior without an owning SDS, contract, or phase gate.

### Phase 1 Gate Outputs

#### Link Attachment Matrix

| Artifact | Required attachment | Phase 1 rule |
| --- | --- | --- |
| `docs/build_plan/sub_build_plan_004_local_development_stack.md` | This Phase 1 gate output plus the full ten-phase implementation sequence. | Remains the implementation-order document for SDS #4. |
| `docs/sds/foundation/local_development_stack.md` | [SDS #4: Local Development Stack](../sds/foundation/local_development_stack.md) | Owns local-stack purpose, safety boundary, data model, command surface, resolved decisions, and validation obligations. |
| `docs/service_catalog/foundation/local_development_stack.md` | [Local Development Stack service plan](../service_catalog/foundation/local_development_stack.md) | Owns service-catalog objective, first build phase, dependency summary, contracts, and handoff. |
| `docs/build_plan/master_plan.md` | [Master build plan](master_plan.md) | Keeps first build point in master Phase 0 and prevents local-stack work from moving later services earlier. |
| `docs/build_plan/service_catalog_alignment.md` | [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #4 aligned to Phase 0 with later simulator expansion gated by owning service phases. |
| `docs/overrid_tech_stack_choice.md` | [Tech stack decision](../overrid_tech_stack_choice.md) | Keeps the local stack Rust-first with Overrid-shaped local stubs rather than conventional cloud product boundaries. |
| `docs/planning/local_development_stack_phase_01_plan.md` | [Phase 1 implementation plan](../planning/local_development_stack_phase_01_plan.md) | Records the concrete Phase 1 work and validation plan. |
| `docs/planning/local_development_stack_phase_01_progress.md` | [Phase 1 progress trail](../planning/local_development_stack_phase_01_progress.md) | Records status, evidence, blockers, and validation commands. |

#### Frozen Local Stack Boundary

The Local Development Stack is a development, CI, and Integration Test Harness surface. It is not a deployment orchestrator, not a production control plane, not a payment runner, not a public-provider testbed, and not a shortcut around service contracts.

Direct storage, queue, object, event, or local file inspection is allowed only for reset, seed, health, and diagnostics. Runtime behavior must still flow through the owning Overrid contracts once those services exist. The local stack may launch embedded state, Overqueue-shaped durable jobs, an Overstore-shaped artifact stub, local diagnostic events, and exactly one local Overcell-like simulator, but it must not claim production authority for Overbase, Overqueue, Overstore, Overvault, Overwatch, Overcell, Overrun, Oversched, accounting, public-provider, federation, or governance behavior.

Rejected Phase 1 shortcuts:

- External production endpoints, public-provider resources, real payment providers, real cloud accounts, or model-provider credentials.
- PostgreSQL, Redis, NATS, Kafka, S3, MinIO, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions as local-stack product boundaries.
- Local-only fixture credentials accepted against seed, staging, production-like, federation, public-provider, or non-local profiles.
- Silent port shifting, wildcard binding, unmarked reset targets, raw committed secrets, or diagnostics that expose secret material.

#### First-Build And Expansion Gate Matrix

| Master phase | Local-stack gate state | Allowed local-stack scope | Required blocker when unavailable |
| --- | --- | --- | --- |
| 0 | `buildable_phase_0` | Local lifecycle, deterministic reset/seed, local/test fixtures, loopback ports, health snapshots, diagnostics, and Phase 0 smoke. | `dependency.phase0_contract_missing` |
| 1 | `local_smoke_prerequisite` | Fixtures and local lifecycle support for the first signed tenant-scoped control-plane smoke path. | `dependency.phase1_control_plane_unavailable` |
| 2 | `owning_service_required` | Exactly one local Overcell-like simulator may expose deterministic identity, heartbeat, capabilities, and health only after the Phase 2 owner contracts are ready. | `dependency.overcell_contract_missing` |
| 3 | `owning_service_required` | No-op execution handoff and pending-work smoke hooks only after Overpack, Oversched, Overlease, Overrun, and Overmeter contracts define the path. | `dependency.execution_loop_contract_missing` |
| 4 | `planned_disabled` | Trust, policy, challenge, verification, classifier, and Overmesh simulations remain disabled unless the owning SDS/API defines local-test behavior. | `dependency.trust_policy_contract_missing` |
| 5 | `planned_disabled` | ORU, Seal Ledger, Overmeter rollups, Overbill, grants, assets, payout, and disputes remain local/test fixtures only. | `dependency.accounting_contract_missing` |
| 6 | `planned_disabled` | Product/adaptor local services remain disabled until adapter, SDK, CLI, and admin/client contracts define local-test behavior. | `dependency.product_integration_contract_missing` |
| 7 | `planned_disabled` | Grid-resident backbone, failover, restore, package promotion, and migration drills stay outside Phase 1 local-stack authority. | `dependency.grid_resident_contract_missing` |
| 8 | `planned_disabled` | Native Overbase, Overstore, Overvault, namespace, Overmesh route, and entitlement behavior must come from owning services, not local-stack shortcuts. | `dependency.native_storage_contract_missing` |
| 9 | `planned_disabled` | Deployment graph, release strategy, and package validation simulations remain disabled without deployment-grid owner contracts. | `dependency.deployment_contract_missing` |
| 10 | `planned_disabled` | Trusted federation and public-interest pool workflows remain disabled without federation owner contracts. | `dependency.federation_contract_missing` |
| 11 | `planned_disabled` | Public-provider onboarding, public sandbox, fraud, reputation, and low-sensitivity pool behavior remain disabled. | `dependency.public_provider_contract_missing` |
| 12 | `planned_disabled` | Native app, assistant, mobile, and public utility local fixtures remain disabled until client/app owner contracts exist. | `dependency.native_app_contract_missing` |
| 13 | `not_local_stack_owned` | Governance, compliance, incident, reporting, migration, and threat-review workflows are not owned by the local stack. | `dependency.governance_contract_missing` |

#### Resolved SDS Decision Checklist

- Rust-owned embedded/local durable state is the only Phase 0 state boundary; candidate engines stay internal and must expose Overbase-shaped and Overqueue-shaped records.
- Filesystem-backed content-addressed Overstore stub is the only Phase 0 artifact boundary; it uses BLAKE3/content hashes, object manifests, local/test markers, deterministic reset, and redacted artifact bundles.
- Exactly one local Overcell-like node simulator is allowed before Phase 2; it provides deterministic identity, heartbeat, capability report, health endpoint, fixture workload acceptance, and no-op handoff only.
- Deterministic loopback port range is reserved as API `18080`, worker health/metrics `18081`, node-agent simulator `18082`, object/artifact stub `18083`, local event/audit query `18084`, and optional developer UI `18085`.
- Clean-checkout CI target is a reproducible Linux x86_64 local/test runner with repository-pinned Rust tooling, loopback networking, no external database/queue/object-store services, no ambient keychain, and no cloud credentials.

#### Documentation Update Rule

When a local profile adds a service, simulator, fixture family, smoke scenario, gate state, diagnostic event, command, or health field, update documents in this order:

1. Update the owning service SDS/API or mark the capability `planned_disabled` with a stable blocker.
2. Update the shared schema/package contract when the local-stack artifact is serialized or consumed by CLI, SDK, harness, CI, or docs-facing examples.
3. Update this sub-build plan and the local development stack service-catalog entry.
4. Update Integration Test Harness scenario manifests, fixtures, smoke paths, and validation evidence only after the owning contract exists.
5. Update the Phase 1 progress trail with validation commands and blockers.

Review rejects local-stack additions that invent service behavior without an owning SDS, API contract, phase gate, and focused validation evidence.

## Phase 2: Stack Schemas And Compatibility Contracts

### Work Items

- **2.1 Define stack profile schema.**
  - Design: Add `stack_profile` with profile id, environment class, enabled services, required phase gates, service dependencies, fixture set, feature flags, port policy, volume set, and local/test marker.
  - Output: Canonical schema plus valid and invalid examples for default local, CI smoke, harness, and later extended profiles.
  - Validation: Schema tests reject profiles missing local/test markers, using non-loopback endpoints by default, or enabling future services without phase gates.

- **2.2 Define service definition schema.**
  - Design: Add `service_definition` records for API, worker, embedded state, Overqueue-shaped job table, Overstore-shaped artifact stub, local event/audit query, and node-agent simulator.
  - Output: Schema for command, working directory, env refs, dependencies, health check, shutdown behavior, log target, and restart class.
  - Validation: Fixture tests reject services without dependency order, health checks, log targets, shutdown behavior, or explicit local/test scope.

- **2.3 Define port, env, and volume schemas.**
  - Design: Add `port_registry`, `local_env_manifest`, and `volume_registry` with deterministic defaults, loopback binding, collision policy, redaction rules, `.env.example` separation, and test-state markers.
  - Output: Canonical registry schemas for ports `18080` through `18085`, generated env names, local volumes, logs, and artifacts.
  - Validation: Tests prove reserved-port collisions fail before startup and reset refuses unmarked volumes.

- **2.4 Define reset, seed, and fixture schemas.**
  - Design: Add `reset_plan` and `seed_manifest` for deterministic tenants, actors, keys, nodes, manifests, packages, workloads, local ORU accounts, ledger refs, and policy contexts.
  - Output: Reset and seed schemas with fixture version, schema version, trace id, and deterministic id requirements.
  - Validation: Schema tests prove seed manifests require test-only fixture markers and reset plans require safe deletion guards.

- **2.5 Define health and diagnostic schemas.**
  - Design: Add `health_snapshot`, `local_secret_record`, and `local_diagnostic_event` with service readiness, schema versions, dependency status, last errors, redaction summary, and local-only event classes.
  - Output: Versioned health and diagnostic contracts shared by CLI, harness, and CI.
  - Validation: Tests prove health states distinguish not-started, starting, ready, degraded, failed, resetting, seeding, and running-tests paths.

## Phase 3: Rust Local-Stack Crate And Command Surface

### Work Items

- **3.1 Create the Rust local-stack crate boundary.**
  - Design: Add a Cargo workspace crate or module for stack manifests, lifecycle orchestration, health checks, reset/seed, fixture generation, diagnostics, and smoke helpers.
  - Output: Local-stack crate skeleton with stable public APIs for CLI, Integration Test Harness, and CI entrypoints.
  - Validation: `cargo check` for the workspace succeeds once implementation exists and no Node.js/TypeScript runtime becomes authoritative for stack orchestration.

- **3.2 Implement manifest loading and validation.**
  - Design: Load stack profiles and service definitions from canonical repo paths, validate with generated schemas, and reject unknown unsafe fields or incompatible schema versions.
  - Output: Manifest loader with human and JSON diagnostics.
  - Validation: Tests cover valid profile, invalid JSON, incompatible version, duplicate service id, missing dependency, and unsafe endpoint variants.

- **3.3 Implement `overrid dev` command handlers.**
  - Design: Expose `start`, `stop`, `restart`, `status`, `reset`, `seed`, `smoke`, `logs`, and `doctor` through Rust CLI handlers backed by the local-stack crate.
  - Output: CLI command surface with stable output envelopes, trace ids where applicable, reason codes, and exit classes.
  - Validation: CLI tests prove every command supports human output, stable `--json`, local profile checks, and clean failure when backing services are unavailable.

- **3.4 Implement phase-aware capability discovery.**
  - Design: Let the stack report which local services and simulators are available for the selected master phase without pretending later services exist.
  - Output: Capability response used by CLI help, harness scenario selection, and smoke gating.
  - Validation: Tests prove Phase 0 profiles expose only foundation services and later services return stable unavailable/blocked reasons until added.

- **3.5 Implement local command state tracking.**
  - Design: Track stack command lifecycle states such as planned, prerequisites_checked, starting, ready, resetting, seeding, smoking, collecting_artifacts, stopped, blocked, failed, and completed.
  - Output: Local command record used for diagnostics and test artifacts.
  - Validation: Transition tests reject impossible state changes and prove failed/blocked paths still produce redacted diagnostics.

## Phase 4: Loopback Topology, Env, Ports, And Secret Guards

### Work Items

- **4.1 Implement deterministic port registry checks.**
  - Design: Reserve the SDS-defined default range: API `18080`, worker health/metrics `18081`, node-agent simulator `18082`, object/artifact stub `18083`, local event/audit query `18084`, and optional developer UI `18085`.
  - Output: Preflight port checker with conflicting process reporting when available.
  - Validation: Tests prove startup fails before launching any service when a reserved port is occupied.

- **4.2 Implement loopback-only binding enforcement.**
  - Design: Force local services to bind to loopback by default and require explicit local profile overrides for any non-default address.
  - Output: Binding policy applied before service launch and captured in health snapshots.
  - Validation: Tests reject wildcard, public, seed, staging, or production-like endpoint bindings in default local and CI profiles.

- **4.3 Implement generated environment manifests.**
  - Design: Generate `.env.local` or equivalent local env values from schema-checked names while keeping `.env.example` committed with names only and no raw secrets.
  - Output: Env generator with redaction metadata and drift checks.
  - Validation: Tests prove generated env output redacts secrets and `.env.example` contains no raw secret values.

- **4.4 Implement test-only secret generation.**
  - Design: Generate local signing keys, API fixture tokens, service shared secrets, and simulator credentials with `local_only` and `test_only` metadata, never committing raw values.
  - Output: Local secret records and signer fixture references.
  - Validation: Tests prove fixture credentials are rejected outside local/test profiles and do not appear in logs or diagnostic bundles.

- **4.5 Implement prerequisite and doctor checks.**
  - Design: Check Rust toolchain, repo layout, schema generation outputs, port availability, file permissions, local volume markers, and CI runner compatibility before starting services.
  - Output: `overrid dev doctor` with stable reason codes and remediation hints.
  - Validation: Doctor tests cover missing runtime, wrong permissions, stale schemas, occupied ports, missing profile, and unsafe env values.

## Phase 5: Embedded State, Overqueue, And Overstore Stubs

### Work Items

- **5.1 Implement Overbase-shaped local state.**
  - Design: Provide a Rust-owned embedded/local durable state layer behind Overrid abstractions for tenants, identities, keys, manifests, events, fixture metadata, reset markers, and schema versions.
  - Output: Local state API and storage adapter hidden behind Overbase-shaped contracts.
  - Validation: Tests prove callers use Overrid state contracts and do not depend on PostgreSQL, MySQL, MongoDB, DynamoDB, or other database product semantics.

- **5.2 Implement Overqueue-shaped local job table.**
  - Design: Provide durable pending, retry, dead-letter, timeout, and terminal job records sufficient for Phase 0 smoke and Phase 1 synthetic pending-work scenarios.
  - Output: Local queue/job table module with idempotency, trace id, priority, retry metadata, and schema version fields.
  - Validation: Tests prove duplicate idempotency keys, pending state, retry metadata, and dead-letter state behave deterministically.

- **5.3 Implement Overstore-shaped artifact stub.**
  - Design: Use filesystem-backed content-addressed objects with BLAKE3/content hashes, object manifests, upload/download grant checks where applicable, local/test markers, and deterministic reset.
  - Output: Object/artifact stub module with manifest and hash verification.
  - Validation: Tests prove artifacts are content-addressed, reset-safe, and not modeled as S3 or MinIO buckets.

- **5.4 Implement local event and audit query support.**
  - Design: Store local diagnostic events separately from real service-emitted Overwatch events while allowing the harness to query Overwatch-style events emitted by local services through normal APIs.
  - Output: Local event/audit query endpoint or helper with trace, service, and time-window filters.
  - Validation: Tests prove local diagnostic events are not treated as authoritative production audit events.

- **5.5 Implement schema-version compatibility gates.**
  - Design: Block startup and smoke when local state, queue records, artifact manifests, fixtures, or service endpoints use incompatible schema versions.
  - Output: Compatibility check used by `start`, `seed`, `smoke`, and harness setup.
  - Validation: Tests prove stale schema versions block execution with stable reason codes and artifact evidence.

## Phase 6: Lifecycle Orchestration And Health Readiness

### Work Items

- **6.1 Implement dependency-ordered startup.**
  - Design: Start embedded state, local job table, object/artifact stub, API, worker, event/audit query, and node-agent simulator in dependency order.
  - Output: Lifecycle orchestrator with start graph, per-service timing, and rollback behavior on required-service failure.
  - Validation: Tests cover clean start, already-running services, required failure, optional degraded service, and partial-start rollback.

- **6.2 Implement graceful stop and restart.**
  - Design: Stop services in reverse dependency order, preserve state on stop, support restart without reset, and emit redacted diagnostic events.
  - Output: Stop/restart handlers with per-service shutdown reports.
  - Validation: Tests prove stop preserves marked state and restart reuses compatible local profile data.

- **6.3 Implement readiness and liveness checks.**
  - Design: Poll API `/healthz`, API `/readyz`, worker health/readiness, node-agent simulator health, object/artifact stub health, and event/audit query health when separate.
  - Output: Health snapshot table for CLI, harness, and CI.
  - Validation: Tests prove readiness distinguishes starting, ready, degraded, failed, timeout, and stale-schema conditions.

- **6.4 Implement bounded wait and timeout policy.**
  - Design: Use deterministic startup windows, per-service timeout classes, dependency wait diagnostics, and no unbounded sleeps.
  - Output: Bounded wait controller with reason-coded timeout output.
  - Validation: Tests prove health timeouts stop dependent services and export logs before returning failure.

- **6.5 Implement lifecycle event emission.**
  - Design: Emit local diagnostic events for start requested, service starting, service ready, reset started, seed started, seed completed, smoke started, smoke completed, and failure.
  - Output: Local diagnostic event stream and artifact refs.
  - Validation: Tests prove lifecycle events include stack profile, fixture version where applicable, schema version, trace id where applicable, and no raw secrets.

## Phase 7: Reset, Seed, Fixtures, And Test-Only Identity

### Work Items

- **7.1 Implement reset safety checks.**
  - Design: Require explicit local profile, local/test environment marker, and test-state volume or table markers before deleting any state.
  - Output: Reset executor with dry-run report, deletion plan, and abort-on-unsafe-state behavior.
  - Validation: Tests prove reset aborts without deleting state when markers are missing, stale, or inconsistent.

- **7.2 Implement deterministic seed.**
  - Design: Seed tenant, actor, key, node, manifest, package, workload, local ORU account, local Seal Ledger refs, and policy context fixtures in deterministic order.
  - Output: Seed manifest application with stable ids and fixture version metadata.
  - Validation: `reset && seed` produces identical fixture ids across repeated runs.

- **7.3 Implement fixture credential isolation.**
  - Design: Mark all fixture credentials, actors, tenants, nodes, package refs, workload refs, ORU accounts, and ledger refs as local/test and bind them to the selected stack profile.
  - Output: Fixture isolation checks used by CLI, SDK, local API, and harness.
  - Validation: Tests prove fixture credentials fail against seed, staging, production-like, federation, public-provider, and non-local profiles.

- **7.4 Implement Phase 1 control-plane seed prerequisites.**
  - Design: Prepare the minimal fixture set for the first signed tenant-scoped command path: tenant, actor, key, manifest, idempotency key, trace root, and pending-work target.
  - Output: Phase 1-ready seed pack gated behind Phase 1 service availability.
  - Validation: Tests prove Phase 0 can prepare the fixtures and Phase 1 consumes them without changing ids or bypassing signing.

- **7.5 Implement fixture drift detection.**
  - Design: Compare seed manifests, generated ids, schema versions, event refs, local accounts, and artifact hashes across runs.
  - Output: Drift report with stable diff fields and owning fixture references.
  - Validation: Repeated seed tests detect nondeterministic ids, stale schema versions, missing refs, and unexpected extra fixture records.

## Phase 8: Node Simulator, Smoke Path, And Harness Integration

### Work Items

- **8.1 Implement one local Overcell-like node simulator.**
  - Design: Provide deterministic identity, heartbeat, capability report, health endpoint, fixture workload acceptance, and no-op execution handoff sufficient for Phase 0 smoke and Phase 1 pending-work tests.
  - Output: Node-agent simulator service definition and simulator module.
  - Validation: Tests prove the simulator does not perform real hardware discovery, GPU runtime integration, benchmark publication, installer/update flow, remote shell behavior, or provider eligibility decisions.

- **8.2 Implement Phase 0 smoke command.**
  - Design: Run signed no-op command admission, valid audit event write/read, invalid schema denial, trace propagation, fixture state inspection, and redacted artifact export.
  - Output: `overrid dev smoke` scenario using stack profile, deterministic fixtures, and generated contracts.
  - Validation: Smoke tests prove signed command path, audit event, invalid schema denial, and trace id propagation.

- **8.3 Integrate with the Integration Test Harness.**
  - Design: Expose lifecycle hooks for harness start, status, reset, seed, smoke, logs, health snapshots, event export, and artifact collection.
  - Output: Harness-facing local-stack API and command contract.
  - Validation: SDS #3 harness scenarios can start, reset, seed, run Phase 0 smoke, and collect artifacts through documented commands.

- **8.4 Implement CLI and SDK smoke support.**
  - Design: Let CLI/SDK smoke tests exercise local stack commands and Overgate-shaped control-plane requests without calling private storage or simulator internals.
  - Output: CLI/SDK smoke fixtures and request examples.
  - Validation: Tests prove smoke paths route through generated contracts and public local APIs only.

- **8.5 Define later simulator expansion rules.**
  - Design: Add later service simulators only when they preserve production contract shape, stay local/test-marked, and do not claim provider, accounting, trust, or execution behavior before owning phases exist.
  - Output: Expansion checklist for Phase 2 node registration, Phase 3 execution, Phase 4 policy, Phase 5 accounting, and Phase 8 storage/namespace additions.
  - Validation: Review confirms simulator expansion cannot move later master-phase responsibilities into Phase 0.

## Phase 9: Diagnostics, Artifacts, CI, And Flake Evidence

### Work Items

- **9.1 Implement redacted log streaming and export.**
  - Design: Stream and bundle API, worker, local state, job table, artifact stub, event query, and node simulator logs with redaction for secrets, tokens, signatures, private payloads, and encrypted content.
  - Output: `overrid dev logs` and artifact bundle log sections.
  - Validation: Redaction tests inject sentinel values and prove bundles fail export until forbidden values are removed.

- **9.2 Implement diagnostic artifact bundles.**
  - Design: Collect health snapshots, stack profile, schema versions, fixture version, command lifecycle, trace ids, reason codes, local events, queue state, object refs, and reproduction command.
  - Output: Redacted diagnostic bundle with manifest and retention class.
  - Validation: Failed smoke runs leave enough evidence to reproduce stack profile, fixtures, scenario, trace id, and failure reason.

- **9.3 Implement clean-checkout CI entrypoint.**
  - Design: Support a reproducible Linux x86_64 local/test runner equivalent to Ubuntu 24.04 LTS with repository-pinned Rust toolchain, loopback networking, no cloud credentials, and no external database/queue/object-store services.
  - Output: CI command sequence for `dev:start`, `dev:reset`, `dev:seed`, `dev:smoke`, `schema:check`, `layout:check`, `docs:check`, and harness smoke.
  - Validation: CI fixture tests distinguish success, blocked, and failed outcomes with stable machine-readable output.

- **9.4 Implement flake detection fields.**
  - Design: Track repeated run count, startup timing variance, nondeterministic fixture ids, unstable event ordering, retry counts, health timeout classes, and tolerance-window use.
  - Output: Flake metadata in health snapshots, smoke reports, and artifact bundles.
  - Validation: Repeated smoke runs detect changed fixture ids, unstable timing assertions, and nondeterministic trace output.

- **9.5 Implement local cleanup and retention policy.**
  - Design: Keep successful smoke summaries compact, retain failure bundles longer, and provide explicit prune behavior that respects test-state markers.
  - Output: Artifact retention classes and cleanup command integration.
  - Validation: Tests prove pruning cannot delete unmarked user directories, production-like state, or non-local artifacts.

## Phase 10: Validation, Documentation, And Downstream Handoff

### Work Items

- **10.1 Validate local-stack self-consistency.**
  - Design: Test schemas, manifest loading, port checks, env generation, secret records, lifecycle, reset, seed, smoke, health, logs, diagnostics, and artifacts as one coherent local stack.
  - Output: Local-stack self-test suite tied to SDS #4 validation items.
  - Validation: `docdexd run-tests` or the repo's canonical test command passes once test-runner configuration and implementation exist.

- **10.2 Validate tech-stack alignment.**
  - Design: Audit implementation and docs for Rust-first local tooling, JSON/JSON Schema contracts, Overrid-shaped embedded/local state, Overqueue-shaped jobs, Overstore-shaped artifact stub, and no conventional cloud product boundaries.
  - Output: Tech-stack alignment checklist for local development.
  - Validation: Scans and review find no PostgreSQL, Redis, S3, MinIO, NATS, Kafka, Vault, blockchain, NFT, pricing, revenue, or customer-count assumptions in local-stack scope except lines that explicitly reject them.

- **10.3 Validate security and environment separation.**
  - Design: Exercise loopback binding, profile class checks, reset markers, fixture credential isolation, secret redaction, diagnostic bundle redaction, and non-local profile denial.
  - Output: Local-stack security test report tied to SDS policy/security requirements.
  - Validation: Tests prove no default local path can reach seed, staging, production-like, federation, public-provider, payment, or provider-payout behavior.

- **10.4 Validate documentation links and queue evidence.**
  - Design: Keep SDS #4, service catalog entry, master plan, crosswalk, build-plan progress, queue state, and Docdex memory aligned with the created sub-build plan.
  - Output: Updated documentation references and recorded validation evidence.
  - Validation: Markdown links pass, queue state marks `004-build-plan` complete, and Docdex search returns this plan for SDS #4 queries.

- **10.5 Prepare downstream phase handoff.**
  - Design: Document how Phase 1 through Phase 13 builders add local services, simulators, fixtures, smoke scenarios, diagnostics, and artifact expectations without bypassing owning-service contracts.
  - Output: Handoff rules for control-plane, private swarm, execution, policy, accounting, product integration, grid-resident backbone, data/storage/namespace, deployment, federation, native apps, and governance work.
  - Validation: Handoff review confirms the local stack can expand with later services without changing master Phase 0 through Phase 13 order or weakening local/test safety.

## Alignment Review

- The sub-build plan keeps Local Development Stack first build work in master Phase 0, matching the SDS, service catalog entry, Phase 0 plan, and build-plan crosswalk.
- The plan treats local database, queue, object store, node-agent, events, fixtures, and secrets as Overrid-shaped local/test surfaces, not external cloud products or production substitutes.
- The plan carries forward SDS #4 resolved decisions: Rust-owned embedded/local state, filesystem-backed content-addressed Overstore stub, one local Overcell-like simulator, deterministic loopback port range, and reproducible Linux clean-checkout CI target.
- The plan explicitly depends on Repository Layout, Shared Schema Package, Integration Test Harness, CLI/SDK, Overgate, Overwatch, Overqueue, and later owning services without moving those later services earlier.
- The plan does not require any change to the master Phase 0 through Phase 13 dependency order.
- The plan adds only a more detailed per-SDS implementation layer under `docs/build_plan`.

## Exit Gate

SUB BUILD PLAN #4 is complete when a builder can implement the Local Development Stack as a Rust-first, loopback-only, deterministic local/test environment that starts, stops, resets, seeds, smokes, reports health, exports redacted diagnostics, and feeds the Integration Test Harness without violating tech-stack boundaries, fixture safety, service ownership, auditability, or the canonical master build order.
