# SUB BUILD PLAN #46 - Deployment Planner

Attached SDS: [docs/sds/deployment_grid/deployment_planner.md](../sds/deployment_grid/deployment_planner.md)

## Purpose

This sub-build plan turns SDS #46 into an implementation sequence for Deployment Planner. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Deployment Planner is the Phase 9 bridge between signed Overpack application intent and bounded deployment execution. It owns plan records, plan revisions, `deployment_graph_v0`, preflight aggregation, rollback graph generation, execution cursors, downstream command envelopes, deployment timelines, and replayable evidence without owning package validation, release strategy policy, workload execution, storage, vault, routing, accounting, ledger, or primary state truth.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #46: Deployment Planner](../sds/deployment_grid/deployment_planner.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved design decisions. |
| [Deployment Planner service plan](../service_catalog/deployment_grid/deployment_planner.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, deterministic fixtures, canonical JSON plus JSON Schema, signed command envelopes, idempotency keys, trace ids, stable reason codes, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate ingress, Overpass identity, Overtenant scope, Overkey signatures, Overregistry refs, Overwatch audit, and Overqueue state prerequisites. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack workload contracts, Oversched placement, Overlease reservations, Overrun/Overcell execution handoffs, and Overmeter raw usage facts. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard decisions, Policy Dry-Run previews, workload/data class facts, route and egress policy gates, and replayable denial reasons. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies ORU budget prechecks, Overbill hooks, Seal Ledger evidence refs, and metering/accounting visibility without pricing, revenue, or mutable balance assumptions. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service runtime expectations, failover/restore readiness, grid-resident command safety, and founder-hardware migration constraints. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase, Overstore, Overvault, Universal Namespace Service, and Overmesh provisioning boundaries. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Controls the first build point: application-intent planning, package validation binding, provisioning, route activation, release strategy handoff, rollback, health checks, and one signed manifest deploying without manual infrastructure edits. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies security review, incident response, audit export, migration, compliance, governance, and scale-hardening expectations. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #46 first build work aligned to master Phase 9, with earlier prerequisite phases and later Phase 13 governance/reliability hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 9, and 13 | Attach SDS #46, freeze authority boundaries, preserve Phase 9 as the first build point, and record prerequisite and hardening gates. |
| 2 | Master Phases 0, 1, 4, 5, 8, and 9 | Define Rust contracts, canonical schemas, `deployment_graph_v0`, rollback/evidence models, reason codes, and deterministic fixtures. |
| 3 | Master Phases 0, 1, 3, 4, and 9 | Implement manifest intake, validation-report binding, actor/tenant authorization, read-only graph generation, and plan revision rules. |
| 4 | Master Phases 4, 5, 8, and 9 | Implement preflight aggregation across policy, validation freshness, budget, resource, route, storage, vault, restore, and health-readiness facts. |
| 5 | Master Phases 4, 7, 8, 9, and 13 | Implement rollback graphs, side-effect classes, compensation rules, downstream command envelopes, approval gates, and restore-point requirements. |
| 6 | Master Phases 1, 3, 5, 8, and 9 | Implement idempotent execution, cursor persistence, dependency scheduling, retry handling, pause/resume/block flows, and evidence spooling. |
| 7 | Master Phases 7, 8, and 9 | Implement provisioning handoffs for runtime, data, storage, vault, namespace, routes, health gates, planner-only activation, and Release Strategy boundaries. |
| 8 | Master Phases 1, 4, 9, and 13 | Implement timeline queries, replay mode, dry-run diffs, operator diagnostics, tenant-safe views, and redaction. |
| 9 | Master Phases 5, 6, 9, 12, and 13 | Implement metering/billing confirmations, CLI/SDK/admin views, Release Strategy, Package Validator, Overwatch, incident, and native/AI consumer handoffs. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Deployment Planner core is a Rust service/module using shared contract crates, Tokio for bounded workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Deployment plans, graphs, steps, rollback plans, cursors, command envelopes, timelines, evidence bundles, API objects, event payloads, fixtures, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed actor or service-account envelopes, tenant/system scope, trace id, idempotency key, schema version, policy refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for manifest refs, package refs, graph snapshots, command payload refs, evidence bundles, replay inputs, and deterministic fixture comparison.
- Planner records may point to Overbase, Overstore, Overvault, Universal Namespace Service, Overmesh, Overmeter, ORU, Overbill, Seal Ledger, Overwatch, Oversched, Overlease, and Overrun refs, but Deployment Planner must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, revenue projections, customer-count assumptions, raw secret storage, direct payment execution, release strategy ownership, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Planner Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #46.**
  - Design: Link this document from the Deployment Planner SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/deployment_grid/deployment_planner.md`, `docs/service_catalog/deployment_grid/deployment_planner.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #46 returns both the Deployment Planner SDS and this sub-build plan.

- **1.2 Freeze Deployment Planner authority boundaries.**
  - Design: Record that Deployment Planner owns plan records, plan revisions, graph generation, execution cursors, retry windows, preflight aggregation, command envelopes, rollback graph generation, timelines, and evidence refs.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the planner does not validate package internals, decide canary/rolling/blue-green strategy, execute workloads, own data/storage/vault/namespace/route/billing/ledger truth, mutate Seal Ledger history, or bypass Overguard, Overmeter, ORU, Overbill, Overwatch, or downstream owner APIs.

- **1.3 Preserve master Phase 9 as the first build point.**
  - Design: Keep first implementation in Phase 9 after shared contracts, identity, policy, private execution, accounting, grid-resident backbone, and native data/storage/namespace primitives are available.
  - Output: Phase-gate note that Phase 9 builds the application-intent deployment planner, while earlier phases supply prerequisites and Phase 13 hardens governance, compliance, security review, and scale.
  - Validation: Review proves this plan does not move application deployment into Phase 3 workload execution, data ownership into Phase 8 services, release strategy into the planner, or governance hardening before the Phase 9 deployment path is proven.

- **1.4 Carry forward resolved SDS #46 decisions.**
  - Design: Preserve `deployment_graph_v0` as canonical JSON plus JSON Schema, fresh preflight requirements, 24-hour draft readability, five-minute or TTL reservation freshness, planner-only route activation limits, Release Strategy ownership of traffic-shift changes, auto-compensation boundaries, and restore-point requirements for stateful migrations.
  - Output: Resolved-decision checklist tied to contract, preflight, execution, route, rollback, and migration reviews.
  - Validation: Review rejects UI-only graph formats, stale execution from old drafts, automatic destructive compensation, planner-owned canary/blue-green/rolling strategy, production traffic shifts without Release Strategy, and destructive migration readiness without verified restore refs.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Overpack, Package Validator, Overguard, Policy Dry-Run API, Oversched, Overlease, Overrun, Overcell, Overbase, Overstore, Overvault, Universal Namespace Service, Overmesh, Overmeter, ORU Account Service, Overbill, Seal Ledger, Backup and Restore Service, Release Strategy Service, Overwatch, SDK, CLI, Admin UI, and incident response.
  - Output: Boundary matrix listing owner, input refs, output refs, command contract, freshness rule, policy gate, idempotency behavior, rollback behavior, audit refs, and downstream consumer.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, and Overwatch events rather than privileged shared tables or direct primary-state reads.

## Phase 2: Rust Contracts, Canonical Graph Schema, And Fixtures

### Work Items

- **2.1 Create the Deployment Planner Rust contract module.**
  - Design: Add contract types for deployment plans, plan revisions, graph nodes, graph edges, steps, cursors, rollback plans, provisioning request refs, evidence bundles, timelines, API errors, events, redaction profiles, and audit exports.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, side-effect-class enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overpack, Package Validator, Release Strategy, Overbase, Overstore, Overvault, Overmesh, Overmeter, Overbill, and Seal Ledger internals.

- **2.2 Define canonical deployment record schemas.**
  - Design: Model `deployment_plan`, `deployment_step`, `deployment_cursor`, `provisioning_request_ref`, and `deployment_evidence_bundle` with stable ids, tenant/app refs, manifest refs and hashes, validation refs, policy refs, budget refs, strategy refs, lifecycle state, command refs, evidence refs, trace ids, idempotency keys, and audit refs.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, and docs-facing examples.
  - Validation: Schema tests reject missing tenant scope, app id, manifest hash, validation refs, policy refs, idempotency key, trace id, schema version, lifecycle state, stable reason codes, or audit refs.

- **2.3 Define `deployment_graph_v0`.**
  - Design: Model the canonical graph as a versioned DAG with typed node and edge records for plan/step ids, dependencies, blockers, service targets, command types, resource scopes, side-effect classes, rollback links, parallelization groups, reason codes, manifest refs, policy refs, and Overwatch evidence refs.
  - Output: `deployment_graph_v0` JSON Schema, canonical examples, invalid examples, graph sorting rules, deterministic hashing rules, UI read-model generation rules, and replay fixture snapshots.
  - Validation: Tests prove identical inputs produce identical graph hashes and reject cycles, dangling step refs, missing rollback links where required, unsupported side-effect classes, non-deterministic ordering, and UI-only graph fields that are not derived from canonical records.

- **2.4 Define rollback and compensation schemas.**
  - Design: Model `rollback_plan`, rollback steps, safe-to-auto-execute flags, required restore points, route reversion steps, data migration reversion refs, operator approval requirements, compensation policies, and approval reason codes.
  - Output: Rollback graph schema, compensation policy schema, operator approval schema, restore-point fixture refs, and failure-mode examples.
  - Validation: Tests reject destructive rollback without restore refs, secret/vault grant changes without approval, ledger/accounting correction as direct mutation, route cutover rollback without Release Strategy ownership, and auto-compensation for externally visible or non-idempotent effects.

- **2.5 Create deterministic planner fixtures.**
  - Design: Build fixtures for one valid app deployment, policy denial, stale validation report, route ownership failure, budget reservation failure, capacity conflict, missing restore point, downstream timeout, route activation health failure, safe rollback, unsafe rollback requiring approval, replay, and dry-run diff.
  - Output: Fixture directory, canonical manifests, validation report refs, policy refs, graph outputs, cursor states, command refs, evidence bundles, reason codes, hashes, and redacted views.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, signatures, graph hashes, state transitions, denial reasons, audit refs, and idempotency outcomes across repeated runs.

## Phase 3: Manifest Intake, Validation Binding, And Read-Only Plan Generation

### Work Items

- **3.1 Implement signed manifest intake.**
  - Design: Add `POST /deployment-plans` for signed Overpack manifest refs, validation report refs, optional release strategy refs, budget scope, actor/service account, tenant scope, trace id, schema version, and idempotency key.
  - Output: Intake handler, request contract, idempotency store, manifest-ref fetcher, initial plan record, stable denial reasons, and `deployment_planner.plan_submitted` events.
  - Validation: Tests prove unsigned requests, wrong tenant scope, missing idempotency key, missing manifest refs, hash mismatch, stale schema versions, and duplicate requests with different payloads are rejected before plan generation.

- **3.2 Bind Package Validator reports without owning validation.**
  - Design: Resolve Package Validator refs, artifact hashes, warning/error codes, schema/ruleset versions, signer authority, revocation state, and compatibility previews as inputs to the plan without revalidating package internals.
  - Output: Validation binding model, freshness checker, report mismatch reason codes, and read-only validation evidence refs.
  - Validation: Tests prove missing, stale, hash-mismatched, revoked, wrong-signer, and incompatible validation reports fail deterministically before side effects.

- **3.3 Authorize actor, tenant, app, route, and budget context.**
  - Design: Use Overgate, Overpass, Overtenant, Overkey, Overregistry, Overguard, ORU/Overbill, Universal Namespace Service, and Overmesh refs to establish authority facts before building executable steps.
  - Output: Authorization fact bundle, owner refs, route ownership refs, budget scope refs, tenant/app refs, and denial reason catalog.
  - Validation: Tests prove wrong actor, suspended tenant, missing app ownership, route-not-owned, missing budget scope, revoked key, and missing policy refs fail before plan enters `ready`.

- **3.4 Generate read-only dependency graphs.**
  - Design: Expand manifest intent into runtime, state, storage, vault, namespace, route, policy, metering, billing, health, and release steps without issuing downstream commands.
  - Output: Read-only graph generator, deterministic sort, dependency edge rules, blocker rules, parallelization groups, side-effect classifications, and graph hash.
  - Validation: Tests prove graph generation has no side effects, preserves manifest intent, delegates placement to Oversched/Overlease, delegates provisioning to owner services, and records stable reason codes for omitted or blocked steps.

- **3.5 Implement plan revisions and stale draft handling.**
  - Design: Allow draft plans to remain readable for 24 hours while requiring fresh preflight for execution; submitted plan revisions are append-only and correction creates a linked new revision.
  - Output: Revision model, stale draft reason code, draft TTL check, superseded-revision links, and plan history query.
  - Validation: Tests prove stale drafts cannot execute without fresh preflight, submitted history is never overwritten, revision links preserve evidence, and duplicate plan creation with the same idempotency key returns the existing result.

## Phase 4: Preflight Aggregation And Admission Readiness

### Work Items

- **4.1 Implement validation and manifest freshness preflight.**
  - Design: Recheck manifest hash, package validation report refs, ruleset versions, signer authority, revocation state, and schema compatibility at submit and execute time.
  - Output: Preflight validator, freshness window rules, stale report reason codes, and evidence refs.
  - Validation: Tests prove reports older than allowed freshness windows, mismatched hashes, revoked signers, and incompatible schema versions block execution without provisioning side effects.

- **4.2 Implement Overguard and Policy Dry-Run aggregation.**
  - Design: Ask Overguard for admission using manifest, validation, workload class, data class, tenant, budget, route, secret/vault, egress, provider eligibility, and restore facts; expose dry-run compatible denials.
  - Output: Policy fact bundle, policy decision refs, denial reason mapping, missing-prerequisite refs, and preflight events.
  - Validation: Tests prove policy denial, missing facts, unsafe data class, invalid egress, public-provider ineligibility, and missing secret grants stop before side effects and produce replayable decision refs.

- **4.3 Implement budget and resource reservation prechecks.**
  - Design: Request ORU/Overbill budget prechecks, Overmark resource-card facts, Oversched feasibility, and Overlease capacity reservation previews with five-minute or lease-TTL freshness, whichever is shorter.
  - Output: Budget precheck adapter, reservation preview adapter, reservation freshness model, blocked-step reason codes, and evidence refs.
  - Validation: Tests prove insufficient budget, stale reservation, capacity conflict, expired lease TTL, and mismatched resource dimensions block execution without creating duplicate reservations.

- **4.4 Implement route, storage, vault, namespace, and restore readiness preflight.**
  - Design: Check namespace ownership, Overmesh route authority, Overbase readiness, Overstore artifact/object scope readiness, Overvault grant refs, and Backup and Restore Service restore-point refs for destructive stateful steps.
  - Output: Readiness adapter set, owner-service fact bundle, restore-point freshness checks, route authority refs, vault grant refs, and denial reasons.
  - Validation: Tests prove missing route ownership, unavailable storage/vault readiness, missing namespace authority, revoked vault grants, and stale/missing restore points block affected steps before side effects.

- **4.5 Implement preflight state transitions and evidence.**
  - Design: Move plans through `submitted`, `preflighting`, `ready`, `blocked`, `failed`, or `stale_plan_revision` using stable reason codes and trace-linked evidence.
  - Output: Preflight state machine, `deployment_planner.preflight_started`, `preflight_passed`, and `preflight_failed` events, evidence refs, and plan query projection.
  - Validation: Tests prove every preflight terminal state includes reason codes, policy refs, audit refs, redacted summaries, and no private topology or secret details in tenant-facing responses.

## Phase 5: Rollback Graphs, Command Envelopes, And Compensation Policy

### Work Items

- **5.1 Generate rollback graphs before forward execution.**
  - Design: Build a rollback graph alongside the forward graph with dependency order, restore requirements, route reversion steps, data migration reversion refs, and operator approval flags.
  - Output: Rollback graph generator, forward-to-rollback mapping, restore-point refs, route reversion refs, approval refs, and rollback plan state.
  - Validation: Tests prove no forward plan can enter `ready` when required rollback or restore metadata is missing for risky or destructive steps.

- **5.2 Classify side effects and compensation safety.**
  - Design: Assign side-effect classes for provisional invisible steps, queued commands, staged artifacts, namespace drafts, unactivated route drafts, unfinalized metering/billing hook drafts, traffic-visible changes, destructive migrations, secret/vault changes, ledger/accounting corrections, and system-service route cutovers.
  - Output: Side-effect classifier, compensation policy table, safe-to-auto-execute flags, operator approval reasons, and review checklist.
  - Validation: Tests prove automatic compensation is limited to provisional, idempotent, non-visible effects and operator approval is required for destructive state migrations, secret/vault changes, ledger/accounting corrections, production/system route cutovers, break-glass overrides, data deletion, and downstream `operator_approval_required` results.

- **5.3 Define downstream command envelope contracts.**
  - Design: Shape signed command envelopes for Oversched/Overlease, Overrun/Overcell, Overbase, Overstore, Overvault, Universal Namespace Service, Overmesh, Overmeter, Overbill, Overwatch, Release Strategy Service, and Backup and Restore Service.
  - Output: Command schemas, idempotency-key derivation, expected result schemas, timeout policies, retry policies, compensation policies, and downstream state query contracts.
  - Validation: Contract tests prove every mutating command includes tenant/system scope, trace id, idempotency key, schema version, policy refs, stable reason codes, and expected evidence refs.

- **5.4 Implement safe pause, cancel, rollback, and approval gates.**
  - Design: Add pause/cancel/rollback contracts that stop at safe boundaries, preserve downstream command state, and require signed operator action where rollback is not automatically safe.
  - Output: `POST /pause`, `POST /cancel`, `POST /rollback`, approval request model, approval event model, and operator reason codes.
  - Validation: Tests prove active commands are not interrupted unsafely, unstarted plans cancel cleanly, partially visible deployments require rollback policy, unsafe rollback requests become approval-required, and all outcomes preserve evidence.

- **5.5 Enforce stateful migration restore-point contracts.**
  - Design: Require restore-point refs from Backup and Restore Service or recognized service-native restore contracts before destructive migration steps can enter `ready`.
  - Output: Restore-point validator, migration checkpoint ref model, consistency window checks, Overvault grant checks where secrets are involved, rollback path checks, and readiness reason codes.
  - Validation: Tests prove destructive migration steps block without current verified restore refs for the declared RPO/RTO class, policy approval, destination trust requirements, rollback path, expected verification checks, and Overwatch evidence.

## Phase 6: Idempotent Execution Engine And Cursor Persistence

### Work Items

- **6.1 Implement execution cursor persistence.**
  - Design: Add durable `deployment_cursor` records with current step ids, last successful step ids, blocked step ids, resume token, retry count, last error code, and evidence refs.
  - Output: Cursor storage abstraction, cursor query, cursor update state machine, resume-token model, and migration/backfill hooks.
  - Validation: Tests prove duplicate `POST /execute` calls with the same idempotency key return the same cursor outcome and interrupted execution resumes without skipping or repeating completed side effects unsafely.

- **6.2 Implement dependency-order step scheduling.**
  - Design: Select ready steps from `deployment_graph_v0` using dependency edges, blockers, parallelization groups, side-effect class constraints, resource scope locks, and downstream availability.
  - Output: Scheduler loop, ready-step selector, parallel group executor, resource lock model, and blocked-step reporting.
  - Validation: Tests prove dependent steps run in order, independent safe steps can run in parallel, resource-scope conflicts serialize, and cycles or missing dependencies are rejected before execution.

- **6.3 Implement downstream timeout and retry handling.**
  - Design: Preserve idempotency keys and query downstream owner state before retrying timed-out commands; use bounded retry windows and reason codes for retryable, blocked, failed, and approval-required results.
  - Output: Retry policy engine, downstream state pollers, timeout reason codes, retry-wait state, and failure evidence.
  - Validation: Tests prove timeouts do not duplicate provisioning, downstream success discovered after timeout advances the cursor, retry exhaustion blocks or fails with evidence, and non-idempotent commands are not retried blindly.

- **6.4 Implement pause, resume, blocked, and cancellation execution behavior.**
  - Design: Respect safe step boundaries for pause/cancel, use blocked states for dependency/policy/budget/operator/retry conditions, and resume only after fresh preflight where freshness rules require it.
  - Output: Execution handlers, blocked-state projection, pause/cancel guards, fresh-preflight hook, and operator-facing reason codes.
  - Validation: Tests prove paused plans resume from the stored cursor, blocked plans keep evidence and reason codes, cancellation cannot occur after irreversible handoff without compensation policy, and stale plans require new preflight.

- **6.5 Implement evidence write spooling.**
  - Design: Do not mark deployment complete until Overwatch events, validation refs, policy decisions, budget checks, route bindings, lease proofs, command results, health checks, and billing hooks are durably written or recovered from a local spool.
  - Output: Evidence spool, Overwatch writer, recovery worker, evidence-bundle builder, and failure reason codes.
  - Validation: Tests prove evidence write failure leaves the plan blocked or retrying, spool recovery can complete evidence writes, and tenant-facing responses include refs and safe summaries only.

## Phase 7: Provisioning, Health Gates, Route Activation, And Release Boundaries

### Work Items

- **7.1 Implement runtime allocation handoffs.**
  - Design: Coordinate runtime allocation through Oversched and Overlease and submit workload/service commands to Overrun/Overcell only through signed owner-service command envelopes.
  - Output: Runtime provisioning adapter, lease proof refs, workload command refs, scheduler reason refs, and readiness evidence.
  - Validation: Integration tests prove the planner does not hard-code placement, does not run workloads directly, preserves lease proofs, and blocks when scheduling or lease evidence is missing or stale.

- **7.2 Implement data, storage, vault, namespace, and route provisioning handoffs.**
  - Design: Submit provisioning commands to Overbase, Overstore, Overvault, Universal Namespace Service, and Overmesh using resource scopes, route refs, grant refs, expected result schemas, idempotency keys, and rollback hints.
  - Output: Provisioning adapter set, command payload schemas, expected-result schemas, route draft refs, namespace draft refs, vault grant refs, storage/object refs, and evidence refs.
  - Validation: Integration tests prove owner services create their own truth, planner stores refs only, raw secrets are never stored, namespace/route ownership is enforced, and failed provisioning can pause or roll back according to compensation policy.

- **7.3 Implement health and readiness gates.**
  - Design: Gate route activation and terminal success on Overwatch health evidence, service readiness checks, metering hook readiness, billing hook readiness, and release-strategy gates where a strategy is attached.
  - Output: Health gate adapter, readiness result model, route activation blocker rules, deployment completion criteria, and `deployment_planner.plan_completed` events.
  - Validation: Tests prove routes cannot activate before health/readiness passes, plan completion waits for metering and billing evidence, and failed health gates pause, retry, block, or request rollback according to strategy and compensation policy.

- **7.4 Enforce planner-only activation limits.**
  - Design: Permit planner-only route activation only for first-time route bindings or internal/private test routes with no existing live traffic, no version-pin change, no route-weight progression, and all policy, ownership, health, metering, billing, and evidence gates passing.
  - Output: Planner-only activation validator, route activation reason codes, policy facts, and failure examples.
  - Validation: Tests prove production/user-facing/system-service/public routes, existing-live-traffic changes, route-weight progression, version-pin changes, route transfers, freeze-window changes, rollback triggers, and break-glass activation require Release Strategy Service approval.

- **7.5 Hand off rollout strategy to Release Strategy Service.**
  - Design: Treat canary, rolling, blue-green, production channel, route-weight, version-pin, live traffic preservation, rollback trigger, freeze-window, and break-glass decisions as Release Strategy ownership.
  - Output: Strategy-ref contract, strategy gate adapter, rollout variant request, route activation handoff, and strategy evidence refs.
  - Validation: Integration tests prove Deployment Planner can request and consume strategy decisions but cannot edit release strategy policy or progress live traffic shifts without strategy evidence.

## Phase 8: Timeline, Replay, Dry-Run Diff, And Operator Diagnostics

### Work Items

- **8.1 Implement deployment timeline queries.**
  - Design: Expose trace-linked plan, preflight, command, step, rollback, route, health, metering, billing, and evidence events through `GET /deployment-plans/{plan_id}/timeline`.
  - Output: Timeline projection, event ordering rules, redaction policy, pagination, filters, and operator/tenant view models.
  - Validation: Tests prove timeline entries are ordered, trace-linked, redacted by data class, and include refs rather than private content, internal topology, raw secret refs, or hidden provider details in tenant views.

- **8.2 Implement replay mode.**
  - Design: Recompute a plan graph from stored inputs, manifest refs, validation refs, policy versions, budget facts, route facts, restore facts, and dependency refs without issuing side effects.
  - Output: Replay API, replay bundle schema, graph comparison report, policy version report, and deterministic hash output.
  - Validation: Tests prove replay reconstructs key decisions from stored inputs, produces the expected graph hash, and marks differences when policy versions, validation reports, or dependency facts changed.

- **8.3 Implement dry-run diff between manifest versions.**
  - Design: Compare two manifests or plan revisions to show added, removed, changed, blocked, newly risky, newly irreversible, newly policy-denied, and newly restore-required steps.
  - Output: Dry-run diff API, change classification schema, human-readable summary, machine-readable diff, and UI read-model examples.
  - Validation: Tests prove diffs are deterministic, side-effect-free, redacted, and derived from canonical `deployment_graph_v0` rather than a separate UI-only graph.

- **8.4 Implement operator diagnostics for graph and cursor state.**
  - Design: Provide authorized operators with graph, step state, downstream command refs, cursor, retry windows, blocked reasons, approval requirements, rollback safety, and evidence bundle refs.
  - Output: Operator projection, search/filter fields, approval prompts, incident linkage, and strict role/policy/audit gates.
  - Validation: Tests prove operator diagnostics require signed authority, least-privilege scope, Overwatch audit, and incident refs for break-glass views.

- **8.5 Implement tenant-safe and client-safe deployment status views.**
  - Design: Expose safe deployment status for SDK, CLI, Admin UI, developer UI, native apps, and AI-generated deployment tools without leaking private provider, route, topology, secret, policy internals, or cross-tenant facts.
  - Output: Tenant-safe view schema, client-safe step summaries, reason-code mapping, remediation hints, and redaction fixtures.
  - Validation: Tests prove tenant/client views include stable state, safe reason codes, evidence refs, and correction paths while hiding protected internals.

## Phase 9: Metering, Billing, Interfaces, And Downstream Handoffs

### Work Items

- **9.1 Emit planner usage and operator-attention events.**
  - Design: Emit planning CPU/time, preflight, retry, blocked-step, rollback, replay, and operator-attention events to Overmeter where material enough to track without encoding pricing or revenue assumptions.
  - Output: Usage event schema, Overmeter handoff refs, resource dimensions, trace ids, and audit refs.
  - Validation: Tests prove planner usage creates usage facts where Phase 5 integration exists and never creates, mutates, or settles balances directly.

- **9.2 Confirm billing hooks before final activation and completion.**
  - Design: Require ORU budget reservation refs and Overbill hook confirmation before provisioning billable resources and before final route activation for billable app workloads.
  - Output: Budget reservation adapter, billing hook adapter, pre-activation gate, completion blocker rules, and billing evidence refs.
  - Validation: Tests prove missing budget reservation, stale reservation, missing Overbill hook, and failed billing confirmation block activation or completion with stable reason codes.

- **9.3 Implement CLI, SDK, and Admin UI command surfaces.**
  - Design: Add generated CLI/SDK/admin operations for plan creation, preflight, read, steps, execute, pause, cancel, rollback, timeline, replay, dry-run diff, and approval workflows.
  - Output: CLI command contracts, SDK bindings, Admin UI view contracts, stable JSON output, error examples, idempotency behavior, and trace propagation.
  - Validation: Contract tests prove client commands pass signed envelopes, trace ids, idempotency keys, schema versions, policy refs, and stable reason codes through generated contracts.

- **9.4 Feed Package Validator, Release Strategy, Overwatch, and incident owners.**
  - Design: Provide graph refs, validation report refs, warning/error refs, strategy refs, route activation refs, rollback refs, restore refs, health refs, timeline refs, and incident refs to owner services.
  - Output: Owner-service event contracts, handoff APIs, strategy handoff fixtures, incident evidence bundles, and Overwatch timeline links.
  - Validation: Integration tests prove owner services consume refs through APIs, not planner-owned private records, and can reject or block planner progress with explicit reason codes.

- **9.5 Prepare native app, mobile, and AI deployment consumers.**
  - Design: Shape read-only redacted views and command boundaries for future native apps, mobile services, personal AI assistant, developer UI, and AI-generated deployment assistants.
  - Output: Consumer view contracts, allowed command list, denied command list, redaction profiles, and follow-up backlog entries.
  - Validation: Review confirms clients and AI tools call Deployment Planner APIs through documented surfaces and cannot write deployment records, provisioning refs, or owner-service truth directly.

## Phase 10: Validation, Security Review, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, planner-owned-release-strategy, planner-owned-storage, planner-owned-billing, planner-owned-ledger, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools or local stubs into product boundaries.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated backlinks and index rows for SDS #46.
  - Validation: Local link checks pass and Docdex search returns the SDS, service plan, crosswalk row, and this sub-build plan in aligned results.

- **10.4 Validate queue state and progress evidence.**
  - Design: Mark `046-build-plan` complete in the Codex55 queue, update `.codex55_sds_queue/progress.md`, append validation evidence to `docs/build_plan/progress.md`, and preserve the next incomplete build-plan task as SDS #47.
  - Output: Updated queue JSON, queue progress summary, and build-plan progress notes.
  - Validation: JSON validation passes; queue validation confirms `046-build-plan` is complete, no task is running, and `047-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders contract, intake, preflight, rollback, execution, provisioning, route/release, replay, interface, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.
