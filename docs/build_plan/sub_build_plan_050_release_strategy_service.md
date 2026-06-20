# SUB BUILD PLAN #50 - Release Strategy Service

Attached SDS: [docs/sds/deployment_grid/release_strategy_service.md](../sds/deployment_grid/release_strategy_service.md)

## Purpose

This sub-build plan turns SDS #50 into an implementation sequence for Release Strategy Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Release Strategy Service is the rollout-intent authority for app and system-service deployments. It owns release plans, strategy templates, release channels, traffic steps, health gates, promotion rules, freeze records, version pins, rollback triggers, approvals, and release evidence refs. It does not build packages, validate package internals, generate deployment-plan graphs, execute workloads, store backups, perform live failover, manipulate routes outside Overmesh command contracts, or mutate ORU, Seal Ledger, Overmeter, or Overbill state.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #50: Release Strategy Service](../sds/deployment_grid/release_strategy_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflow, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Release Strategy Service plan](../service_catalog/deployment_grid/release_strategy_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed envelopes, trace ids, idempotency, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate ingress, Overpass identities, Overtenant scope, Overkey signing refs, Overregistry package/version facts, Overwatch audit, and Overqueue state prerequisites. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies workload execution, Overpack workload manifests, Overrun states, Overmeter raw usage facts, and package-validation report consumers that later app releases must not bypass. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy decisions, Workload Classifier facts, Policy Dry-Run previews, Oververify trust refs, Overclaim dispute holds, and Overmesh private routing foundations. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies metering, ORU budget prechecks, billing-hook confirmation, accounting evidence refs, and near-cost visibility without direct balance, invoice, payout, or ledger mutation by Release Strategy Service. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies system-service workload class, grid-resident package contracts, backup/restore evidence, failover/recovery handoffs, and protected operator approval rules for backbone releases. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase, Overstore, Overvault, Universal Namespace Service, Overmesh route resolution, and Overasset refs that release plans may require but do not own. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Controls the first build point: app release plans, route/health/release gates, canary or blue-green first proof, rolling/manual strategies, rollback handoff, version pins, and no-manual-infrastructure deployment proof. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies incident response, threat modeling, release audit export, compliance retention, security review, break-glass governance, and stewardship reporting hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #50 first build work aligned to master Phase 9, with earlier phases as prerequisites and Phase 13 as hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 9, and 13 | Attach SDS #50, freeze rollout authority boundaries, preserve Phase 9 as first build, and identify prerequisite owner-service gates. |
| 2 | Master Phases 0, 1, 7, 8, 9, and 13 | Define Rust contracts, release schemas, strategy templates, channel records, health gates, version pins, rollback triggers, evidence bundles, and deterministic fixtures. |
| 3 | Master Phases 1, 3, 4, 5, 7, 8, and 9 | Implement release plan intake, immutable input snapshots, compatibility checks, append-only revisions, and read/timeline projections. |
| 4 | Master Phases 3, 4, 5, 7, 8, and 9 | Implement preflight aggregation for validation reports, deployment-plan readiness, route ownership, policy/freeze rules, restore refs, and metering/billing hooks. |
| 5 | Master Phases 7, 8, 9, and 13 | Implement strategy templates, release channels, traffic-step state, canary, blue-green, rolling, route-weight, freeze-window, approval, and version-pin semantics. |
| 6 | Master Phases 4, 5, 7, 8, and 9 | Implement rollout supervision, Overmesh route-step command integration, Overwatch health-gate evaluation, promotion criteria, and terminal release states. |
| 7 | Master Phases 7, 9, and 13 | Implement pause, resume, rollback, freeze, emergency rollback, package-revocation, and failover-escalation flows without taking over owner-service domains. |
| 8 | Master Phases 6, 9, 12, and 13 | Add SDK, CLI, admin/developer UI, and AI-deployment assistant surfaces with redacted user-visible and operator-only projections. |
| 9 | Master Phases 9 and 13 | Add operations, evidence bundles, simulation, diagnostics, retention, audit export, incident, compliance, and stewardship hooks. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Release Strategy Service core is a Rust service/module using shared contract crates, Tokio for bounded supervision workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Release plans, release revisions, strategy templates, release channels, traffic steps, health gates, rollback triggers, version pins, approvals, freeze records, evidence bundles, API objects, event payloads, fixtures, simulation inputs, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed actor or service-account envelopes, tenant/system scope, trace id, idempotency key, release channel authority, policy decision refs, validation report refs when package versions are involved, stable reason codes, and append-only Overwatch evidence refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for package/version refs, release plan snapshots, strategy template refs, evidence bundles, replay fixtures, release diffs, and deterministic fixture comparison.
- Release Strategy Service may point to Deployment Planner, Package Validator, Grid-Resident Service Packager, Overpack, Overregistry, Overmesh, Overwatch, Overguard, Backup and Restore Service, Failover and Recovery Coordinator, Overmeter, ORU Account Service, Overbill, SDK, CLI, Admin UI, native apps, AI-generated deployment tools, Incident Response, Compliance Boundary, and Stewardship Reporting, but it must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, revenue projections, customer-count assumptions, raw secret storage, package validation, deployment planning, workload execution, backup storage, emergency failover, billing mutation, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Release Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #50.**
  - Design: Link this document from the Release Strategy Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/deployment_grid/release_strategy_service.md`, `docs/service_catalog/deployment_grid/release_strategy_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #50 returns both the Release Strategy Service SDS and this sub-build plan.

- **1.2 Freeze rollout-intent authority boundaries.**
  - Design: Record that Release Strategy Service owns release plans, strategy templates, release channels, traffic steps, health gates, promotion rules, freeze records, version pins, rollback triggers, approvals, and release evidence refs.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the service does not build packages, validate package internals, generate deployment-plan graphs, execute workloads, store backups, perform live failover, mutate routes outside Overmesh commands, or mutate accounting state.

- **1.3 Preserve Phase 9 as the first build point.**
  - Design: Keep first implementation in master Phase 9 because Release Strategy Service depends on signed Overpack application intent, Package Validator reports, Deployment Planner plan refs, Overmesh route refs, Overguard policy refs, Overwatch health evidence, and accounting-hook readiness.
  - Output: Phase-gate note that earlier phases provide prerequisites, Phase 9 builds release plans and rollout supervision, and Phase 13 hardens governance, security, retention, and incident workflows.
  - Validation: Review proves this plan does not move release-plan implementation into Phases 0 through 8 or defer essential Phase 9 release controls to Phase 13.

- **1.4 Carry forward resolved SDS #50 decisions.**
  - Design: Preserve the resolved decisions that the first Phase 9 app proof may use manually approved canary or blue-green rollout, health gates are versioned `health_gate_profile` records, freeze windows block normal forward changes, user-visible pins are redacted by scope, and early automation stays conservative.
  - Output: Resolved-decision checklist tied to first proof, health gates, freeze overrides, version-pin visibility, and automation limits.
  - Validation: Review rejects manual-only first deployment, hardcoded threshold constants, freeze bypass for unvalidated forward fixes, hidden user-impacting pins without stable reason codes, and automatic rollback for stateful/system-service/production scopes before drills are current.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Deployment Planner, Package Validator, Grid-Resident Service Packager, Overpack, Overregistry, Overmesh, Overwatch, Overguard, Backup and Restore Service, Failover and Recovery Coordinator, Overmeter, ORU Account Service, Overbill, SDK, CLI, Admin UI, native apps, AI deployment tools, Incident Response, Compliance Boundary, and Stewardship Reporting.
  - Output: Boundary matrix listing owner, input refs, output refs, freshness rules, policy refs, audit refs, redaction class, downstream consumer, and rejection authority.
  - Validation: Review confirms every handoff uses explicit APIs, immutable refs, signed evidence, stable reason codes, trace ids, idempotency keys, policy refs, and Overwatch events rather than privileged shared records or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Strategy Records, And Fixtures

### Work Items

- **2.1 Create the Release Strategy Rust contract module.**
  - Design: Add contract types for release plans, release revisions, strategy templates, release channels, traffic steps, health gates, rollback triggers, version pins, approvals, freeze records, evidence bundles, lifecycle states, errors, events, redaction profiles, and simulation fixtures.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, strategy enums, channel enums, lifecycle enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Deployment Planner, Package Validator, Overmesh, Overwatch, Overguard, Backup and Restore, Failover Coordinator, Overmeter, ORU, Overbill, and Seal Ledger internals.

- **2.2 Define release plan and release revision schemas.**
  - Design: Model `release_plan` and `release_revision` with tenant scope, app/service scope, environment, channel, strategy type, source/target version refs, deployment plan ref, validation report refs, state, policy refs, route refs, approval refs, audit refs, and append-only revision metadata.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, docs-facing examples, and release snapshot hash fixtures.
  - Validation: Schema tests reject missing tenant/system scope, app/service scope, channel, strategy type, version refs, deployment plan ref when required, validation refs, state, policy refs, trace id, idempotency key, audit refs, or schema version.

- **2.3 Define strategy template, channel, and traffic-step schemas.**
  - Design: Model `release_strategy_template`, `release_channel`, and `traffic_shift_step` with strategy type, traffic steps, promotion gates, rollback triggers, approval requirements, max duration, route behavior, supported workload classes, allowed actors, freeze windows, version-pin rules, route refs, weights, hold durations, and step lifecycle state.
  - Output: Strategy template schema, channel schema, traffic-step schema, compatibility matrix, strategy fixtures, and negative fixtures.
  - Validation: Tests prove strategy/channel changes create versioned refs, unsupported workload classes are rejected, invalid weights or hold durations fail, and route-changing steps require Overmesh-ready route refs.

- **2.4 Define health gate, rollback trigger, version pin, and evidence schemas.**
  - Design: Model `health_gate`, `rollback_trigger`, `version_pin`, and `release_evidence_bundle` with metric refs, thresholds, sample windows, failure budget, incident-link behavior, trigger reason, rollback plan ref, pin scope, expiry, policy refs, approval refs, and immutable evidence refs.
  - Output: Health gate profile schema, rollback trigger schema, version-pin schema, evidence bundle schema, redaction profile, and replay fixtures.
  - Validation: Tests prove thresholds are data in versioned profiles, pins are auditable and reversible, rollback triggers cannot point to unvalidated packages, and evidence bundles can be replayed without exposing private topology or secret-bearing refs.

- **2.5 Create deterministic release fixtures.**
  - Design: Build fixtures for valid canary, canary health failure, blue-green cutover, rolling update step failure, manual release, route-weight change, version-pin conflict, active freeze, missing validation report, stale deployment plan, missing restore ref, unavailable health telemetry, billing hook missing, package revocation, and emergency rollback.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected reason codes, evidence bundle hashes, timeline outputs, and redacted views.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, release states, traffic-step states, reason codes, evidence bundle refs, audit refs, and redacted projections across repeated runs.

## Phase 3: Release Plan Intake, Snapshots, And Read Models

### Work Items

- **3.1 Implement release plan creation and read APIs.**
  - Design: Add `POST /release-plans`, `GET /release-plans/{release_id}`, and list/search projections for app, service, channel, environment, strategy, state, actor, route, version, and policy filters.
  - Output: API handlers, request/response schemas, idempotency behavior, pagination/filter rules, auth/scope checks, stable errors, and event emissions.
  - Validation: API tests cover valid release creation, invalid schema, duplicate idempotency key, missing version ref, unsupported strategy, wrong tenant, unauthorized actor, release lookup, and filtered release search.

- **3.2 Assemble immutable release input snapshots.**
  - Design: Fetch and freeze package/version facts, validation report refs, deployment plan state, route refs, health gate profile refs, policy/freeze facts, restore-point refs, metering/billing hook refs, version-pin state, and trace refs before preflight or rollout side effects start.
  - Output: Snapshot builder, snapshot hash, input availability checks, redacted snapshot projection, fetch-failure reasons, and `release_strategy.plan_created` events.
  - Validation: Tests prove changed package refs, changed deployment plan refs, stale validation reports, missing route refs, missing policy facts, and mismatched snapshot hashes create new revisions or blocked states rather than mutating completed release evidence.

- **3.3 Implement strategy and channel compatibility checks.**
  - Design: Compare strategy type, release channel, environment, app/service scope, workload class, data class, route type, operator approval state, freeze windows, and supported deployment-plan behavior before the plan becomes ready.
  - Output: Compatibility checker, channel policy adapter, strategy eligibility result, missing-prerequisite refs, stable reason codes, and compatibility fixtures.
  - Validation: Tests prove strategy-not-allowed, channel authority failure, production approval missing, system-service class mismatch, data-class restriction, route type mismatch, and freeze conflicts produce replayable blocked states.

- **3.4 Implement append-only release revision semantics.**
  - Design: Allow draft changes before side effects, then require release revisions, pause, rollback, new release plans, or version-pin changes after rollout starts.
  - Output: Revision state machine, immutable history writer, side-effect boundary checks, revision diff output, and conflict errors.
  - Validation: Tests prove started releases cannot silently mutate source/target version refs, strategy templates, route refs, policy refs, or approval refs, and every change after side effects appears in the release timeline.

- **3.5 Implement release timeline and read projections.**
  - Design: Build read models for release state, strategy, channel, version pins, traffic steps, gate states, route refs, approval refs, rollback availability, evidence refs, and redacted timeline events.
  - Output: Timeline API, projection builders, redaction rules, CLI/SDK/Admin UI view contracts, and projection fixtures.
  - Validation: Contract tests prove clients can present current state, next action, blockers, route weights, gate samples, policy decisions, validation refs, and rollback availability without parsing private package payloads, raw policy traces, protected topology, or secret material.

## Phase 4: Preflight And Policy Gate Aggregation

### Work Items

- **4.1 Validate package, deployment-plan, and route readiness.**
  - Design: Require Package Validator report refs, package/version facts, Deployment Planner readiness, rollback plan refs where needed, route ownership, route command availability, and package revocation checks before rollout starts.
  - Output: Preflight checker, readiness fact bundle, stale/missing ref errors, route ownership checks, and preflight fixtures.
  - Validation: Tests prove missing validation reports, stale report refs, deployment plan not ready, rollback plan missing, route ownership failure, route command unavailable, and revoked package refs block start or promotion.

- **4.2 Validate freeze windows and version-pin conflicts.**
  - Design: Check release freeze records, channel freezes, system-service holds, active version pins, conflicting target pins, rollback target pins, expiry, policy refs, and redacted reason visibility.
  - Output: Freeze/pin checker, conflict result schema, pin reason projection, operator-only pin handling, and stable error codes.
  - Validation: Tests prove freeze windows block normal starts/promotions/route-weight increases/forward version changes, pin conflicts stop before route changes, and user-visible blockers include stable reason codes plus redacted evidence refs.

- **4.3 Integrate Overguard policy and approval gates.**
  - Design: Request Overguard decisions for strategy, actor/channel authority, workload/data class, production release, system-service release, freeze, rollback, break-glass, and approval requirements.
  - Output: Policy adapter, approval requirement model, signed operator approval schema, break-glass approval schema, missing-approval state, and policy event refs.
  - Validation: Tests prove policy denial, missing approval, stale policy refs, unauthorized break-glass, and risky rollback without signed approval produce replayable `awaiting_approval`, `blocked`, or `failed` states without route side effects.

- **4.4 Validate restore-point and recovery readiness.**
  - Design: Require Backup and Restore Service refs, restore drill refs, migration safety refs, known-good rollback version refs, Failover and Recovery Coordinator handoff refs, and incident-link behavior for stateful or system-service releases.
  - Output: Recovery-readiness checker, restore/migration/failover fact bundle, stateful release blockers, emergency rollback constraints, and drill freshness rules.
  - Validation: Tests prove stateful and system-service releases cannot start without required restore/rollback evidence, emergency rollback cannot introduce unvalidated packages, and incident-grade recovery escalates instead of being handled locally by Release Strategy Service.

- **4.5 Validate metering and billing hook readiness.**
  - Design: Confirm Overmeter release usage dimensions, deployment-plan metering hooks, ORU budget precheck refs, Overbill billing hook refs, and billable-traffic activation readiness without creating accounting mutations.
  - Output: Metering/billing readiness result, usage dimension mapping, budget precheck ref, billing hook compatibility refs, and owner-service handoff notes.
  - Validation: Tests prove material release orchestration, route-shift, observation, and rollback usage facts are emitted to Overmeter where needed, while Release Strategy Service never creates pricing, revenue, balance transitions, invoices, payouts, or Seal Ledger entries directly.

## Phase 5: Strategy Templates, Channels, Traffic Steps, And Version Pins

### Work Items

- **5.1 Implement the canary strategy template.**
  - Design: Support route-weight-capable services with staged target weights, hold durations, health gate profiles, minimum observation windows or sample counts, pause points, rollback triggers, and final promotion behavior.
  - Output: Canary template implementation, route-step builder, gate schedule builder, promotion criteria, rollback behavior, and canary fixtures.
  - Validation: Tests prove canary can start, observe health, promote, pause on gate failure, roll back to known-good refs, and preserve previous route/version state until final promotion.

- **5.2 Implement the blue-green strategy template.**
  - Design: Support parallel target deployments, readiness gates, cutover gates, previous-environment preservation, rollback target refs, and atomic route cutover constraints.
  - Output: Blue-green template implementation, parallel-target read model, cutover command plan, previous-route preservation behavior, and blue-green fixtures.
  - Validation: Tests prove blue-green preserves old route state until final cutover, blocks cutover on failed readiness, supports rollback to the previous target, and records all route/version refs in evidence bundles.

- **5.3 Implement rolling, manual, and route-weight strategies.**
  - Design: Support rolling update steps, manual promote/hold controls, route-weight-only changes, bounded step sizes, approval gates, health gates, and rollback triggers without assuming per-replica maturity before evidence exists.
  - Output: Rolling strategy template, manual strategy template, route-weight strategy template, step-size constraints, manual approval state, and fixtures.
  - Validation: Tests prove rolling updates advance only when prior steps pass readiness and health gates, manual releases cannot auto-promote without approval, and route-weight changes remain auditable and reversible.

- **5.4 Implement release channels and freeze-window behavior.**
  - Design: Model `dev`, `test`, `private`, `production`, and `system` channels with allowed actors, required approvals, freeze windows, version-pin rules, data/workload class constraints, and emergency release-channel behavior.
  - Output: Channel registry, freeze-window evaluator, emergency channel policy, channel-specific defaults, and channel fixtures.
  - Validation: Tests prove channel rules affect strategy eligibility, approval requirements, freeze behavior, version-pin visibility, health gate defaults, and emergency forward fixes without hardcoding threshold values in service code.

- **5.5 Implement version-pin lifecycle management.**
  - Design: Create, activate, conflict, expire, supersede, and remove pins for app, route, channel, package version, rollback target, system-service, quarantine, incident, or compliance scopes with redacted reason/evidence refs.
  - Output: `POST /version-pins`, pin read/search APIs, lifecycle transitions, conflict detector, expiry scheduler contract, redacted projections, and pin events.
  - Validation: Tests prove pins are auditable, conflict-detected, reversible, scoped by actor/channel authority, visible to affected users where allowed, and redacted when operator-only pins protect sensitive topology or security details.

## Phase 6: Rollout Supervision, Health Gates, And Promotion

### Work Items

- **6.1 Implement rollout supervision start.**
  - Design: Add `POST /release-plans/{release_id}/start` to transition ready plans into rollout supervision only after preflight, policy, approval, route, validation, restore, and metering checks are fresh.
  - Output: Start API, readiness gate, stale-ref refresh rules, supervisor worker, state transition writer, and `release_strategy.rollout_started` events.
  - Validation: API tests prove start is idempotent, rejects stale or failed preflight, blocks missing approvals, and emits no route commands until the release reaches a valid rollout state.

- **6.2 Integrate Overmesh traffic-step commands.**
  - Design: Send route-weight, route-binding, and version-progression requests through Overmesh command contracts with trace ids, idempotency keys, route refs, expected current state, rollback refs, and evidence refs.
  - Output: Overmesh adapter, traffic-step command schema, expected-state guard, route evidence collector, retry behavior, and route rejection reason codes.
  - Validation: Integration tests prove route shifts preserve current state on rejection, detect stale route state, record route evidence, and never let Release Strategy Service mutate route truth outside Overmesh APIs.

- **6.3 Implement versioned health gate evaluation.**
  - Design: Consume Overwatch readiness, health, trace, error, route endpoint, metering/billing hook, incident, and policy evidence through versioned `health_gate_profile` records rather than hardcoded thresholds.
  - Output: Health gate evaluator, profile adapter, sample-window evaluator, failure-budget evaluator, health event refs, and `release_strategy.health_gate_passed/failed` events.
  - Validation: Tests prove low-risk private apps can use simple gates, traffic-bearing canaries require observation windows or sample counts, system-service releases require stricter restore/health/route/lease/queue evidence, and profile changes are versioned policy data.

- **6.4 Implement promotion decisions.**
  - Design: Add `POST /release-plans/{release_id}/promote` and automatic low-risk promotion hooks for allowed private canaries when all gates pass, policy allows automation, and the release is not stateful/system-service/production/public/user-facing.
  - Output: Promotion API, promotion worker, manual approval path, low-risk automation guard, step advancement, final cutover behavior, and `release_strategy.promoted` events.
  - Validation: Tests prove promotion blocks on failed gates, active freezes, missing approvals, stale evidence, version-pin conflicts, route command failures, package revocation, or automation scope restrictions.

- **6.5 Finalize release terminal states.**
  - Design: Mark releases as completed, failed, cancelled, or rolled back only when route state, deployment state, health state, policy refs, validation refs, metering refs, and Overwatch evidence are durable enough for replay.
  - Output: Terminal-state transition logic, final evidence bundle writer, release summary projection, final version-pin updates, and terminal events.
  - Validation: Tests prove completed releases are replayable, failed releases preserve blocker reasons, cancelled releases do not hide side effects, rolled-back releases preserve rollback evidence, and degraded evidence writes prevent false completion.

## Phase 7: Pause, Resume, Rollback, Freeze, And Emergency Paths

### Work Items

- **7.1 Implement pause and resume flows.**
  - Design: Add `POST /release-plans/{release_id}/pause` and `POST /release-plans/{release_id}/resume` with policy refresh, gate freshness checks, current route/version preservation, actor authority, reason codes, and evidence refs.
  - Output: Pause API, resume API, state transitions, stale-gate refresh behavior, timeline events, and redacted pause reasons.
  - Validation: Tests prove pause is idempotent, resume revalidates policy/freeze/gate freshness, and paused releases cannot progress route weights or version pins until explicitly resumed.

- **7.2 Implement rollback through Deployment Planner.**
  - Design: Add `POST /release-plans/{release_id}/rollback` to request rollback using Deployment Planner rollback plan refs, known-good previous version refs, route state, approval refs, and release evidence.
  - Output: Rollback API, Deployment Planner rollback adapter, rollback request schema, rollback availability checker, and `release_strategy.rollback_requested` events.
  - Validation: Tests prove rollback requires a valid rollback plan or known-good version, never introduces an unvalidated package through rollback, and records planner responses without becoming deployment-plan execution owner.

- **7.3 Escalate incident-grade recovery to Failover and Recovery Coordinator.**
  - Design: Detect failed rollback, outage-class health failure, system-service failure, writer/fence failure, backup/restore need, or incident ref and hand off to Failover and Recovery Coordinator plus Incident Response.
  - Output: Escalation adapter, incident-link behavior, evidence handoff bundle, recovery request schema, and escalation reason codes.
  - Validation: Tests prove Release Strategy Service escalates incident-grade recovery instead of performing live failover by itself and preserves release timeline evidence for incident review.

- **7.4 Implement release freezes and emergency exceptions.**
  - Design: Add `POST /release-plans/{release_id}/freeze` and freeze-scope commands for channels, routes, apps, services, package versions, or release plans, with policy/approval refs and emergency rollback exception rules.
  - Output: Freeze API, freeze state machine, freeze override schema, emergency rollback guard, freeze events, and freeze projection.
  - Validation: Tests prove freezes block normal starts/promotions/route-weight increases/forward version changes, urgent security rollback can use only validated known-good refs with signed policy approval, and emergency forward fixes still require validation, planner, Overguard, route, health, and approval gates.

- **7.5 Handle package revocation, policy revocation, and release cancellation.**
  - Design: React to package revocation, signer revocation, policy revocation, route revocation, billing hook failure, operator cancellation, and incident pins by pausing, freezing, rolling back, or failing according to strategy and policy.
  - Output: Revocation event consumer, cancellation API, policy-revocation behavior, package-revocation behavior, billing-hook failure behavior, and timeline output.
  - Validation: Tests prove revocations cannot be ignored mid-release, cancellation preserves side-effect history, and billing/policy/package failures produce stable reason codes and owner-service handoffs.

## Phase 8: Client, SDK, CLI, Admin UI, And AI Deployment Surfaces

### Work Items

- **8.1 Generate SDK and API client contracts.**
  - Design: Add generated client operations for create plan, preflight, start, promote, pause, resume, rollback, freeze, create version pin, read release, list releases, read timeline, read pins, and simulate release.
  - Output: SDK operation contracts, generated Rust SDK hooks, TypeScript/web bindings from the same contracts where needed, stable JSON examples, and error examples.
  - Validation: Contract tests prove clients pass signed envelopes, trace ids, idempotency keys, schema versions, release channel refs, policy refs, validation refs, stable reason codes, and redaction rules through generated contracts.

- **8.2 Implement CLI release commands.**
  - Design: Add CLI command contracts for release create, preflight, start, promote, pause, resume, rollback, freeze, pin, status, timeline, and simulate with stable JSON output and safe exit codes.
  - Output: CLI command specs, input schemas, output schemas, exit-code rules, trace propagation, and examples.
  - Validation: CLI contract tests prove failed preflight, approval required, freeze active, health gate failed, version pin conflict, rollback unavailable, and deployment plan not ready are machine-readable without raw private details.

- **8.3 Implement admin/developer UI release views.**
  - Design: Provide quiet operational views for active releases, paused releases, frozen channels, route weights, current version pins, health gate samples, policy decisions, validation refs, deployment-plan refs, rollback availability, and release timeline.
  - Output: Admin UI view contract, read projections, action enablement rules, redacted tenant/user projections, operator-only projections, and audit refs for privileged views.
  - Validation: UI contract tests prove disabled actions reflect policy/freeze/gate state, user-visible pins are redacted correctly, operator-only pins require privileged scope, and every break-glass or diagnostic view emits Overwatch audit refs.

- **8.4 Integrate AI-generated deployment assistant constraints.**
  - Design: Let AI deployment tools propose release plans only after Package Validator, Deployment Planner, Overguard, route, health, approval, and budget facts are present; require human approval for risky channels and show risk-grouped diffs.
  - Output: AI tool read/write contract, proposed-release schema, risk summary, required human approval flags, safety reason codes, and redacted evidence refs.
  - Validation: Tests prove AI-generated releases cannot bypass package validation, policy preview, human approval rules, route ownership, billing hooks, freeze windows, or rollback constraints.

- **8.5 Implement user-visible and operator-only version-pin projections.**
  - Design: Expose pins affecting a user's app, tenant, route, channel, or release decision while hiding system-service, security quarantine, internal route, topology-sensitive, compliance, and incident pins behind stable reason codes and redacted refs.
  - Output: Pin projection API, redaction profile, user-visible pin view, operator-only pin view, stable blocker codes, and audit events.
  - Validation: Tests prove affected users see actionable pin status and redacted reason refs, while sensitive topology, incident, security, signer, and internal route details remain operator-only.

## Phase 9: Operations, Evidence, Simulation, And Governance Hooks

### Work Items

- **9.1 Emit release event stream and evidence bundles.**
  - Design: Emit `plan_created`, `preflight_passed`, `preflight_failed`, `rollout_started`, `traffic_step_started`, `health_gate_passed`, `health_gate_failed`, `promoted`, `paused`, `rollback_requested`, `version_pinned`, `frozen`, `completed`, and `failed` events with redacted refs.
  - Output: Event schemas, Overwatch event writer, evidence bundle writer, retry/idempotency behavior, and event fixtures.
  - Validation: Tests prove every state transition emits replayable evidence and tenant/public projections hide private package, route, provider, policy, and topology details.

- **9.2 Implement operational monitoring and diagnostics.**
  - Design: Track active releases, paused releases, frozen channels, failed gates, rollback requests, route-shift lag, promotion latency, version-pin conflicts, strategy success/failure rates, common rollback triggers, stuck releases, and degraded evidence writes.
  - Output: Health endpoint, metrics/events, operator diagnostics, stuck-release query, failure-reason dashboard contract, and Overwatch refs.
  - Validation: Tests prove diagnostic views require authorized operator scope, tenant-facing views are redacted, and stuck release recovery emits reason-coded evidence.

- **9.3 Implement release simulation.**
  - Design: Simulate canary, blue-green, rolling, manual, freeze, pin conflict, failed health gate, missing restore, stale validation report, and rollback flows from stored health, route, deployment, package, policy, and fixture inputs without issuing route commands.
  - Output: Simulation API, simulation input schema, simulated timeline output, expected gate states, missing prerequisite summaries, and fixture packs.
  - Validation: Tests prove simulation is side-effect-free, replayable, redacted, and useful for production rollout review without mutating release, route, deployment, package, policy, billing, or accounting truth.

- **9.4 Implement retention, redaction, and audit export.**
  - Design: Retain release plans, traffic-step refs, approvals, freeze records, version pins, rollback requests, route evidence, policy refs, validation refs, and health gate refs according to channel, system-service scope, incident refs, and compliance boundary rules.
  - Output: Retention policy schema, redaction classifier, audit export schema, incident/compliance pins, expiry scheduler contract, and release evidence export.
  - Validation: Tests prove production, system-service, rollback, incident, compliance, and public/user-facing releases preserve required evidence while raw private data, secret refs beyond allowed scope, protected topology, and private package contents are not exposed in reports.

- **9.5 Add incident, threat-model, and stewardship hooks.**
  - Design: Hand release evidence to Incident Response, Threat Modeling and Security Review Tracker, Compliance Boundary, Stewardship Reporting, and PIP governance for high-risk releases, break-glass actions, repeated rollback triggers, and security-significant package/version changes.
  - Output: Governance handoff schema, incident trigger refs, threat-model checklist refs, compliance export refs, stewardship summary refs, and PIP evidence refs.
  - Validation: Review proves release governance hooks are evidence refs and workflow handoffs, not centralization of incident response, compliance authority, stewardship publication, or PIP decisions inside Release Strategy Service.

## Phase 10: Validation, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack and authority guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, release-owned-deployment-planning, release-owned-package-validation, release-owned-backup-storage, release-owned-failover, release-owned-accounting, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, owner-service refs, or downstream APIs into Release Strategy Service's product boundary.

- **10.3 Validate SDS #50 build-breakdown coverage.**
  - Design: Map every SDS #50 build-breakdown item to this plan: schemas, creation/read APIs, preflight checks, Overmesh traffic steps, health gate evaluation, pause/resume/promote/rollback/freeze/version-pin APIs, rollback/failover handoff, and canary or blue-green first proof.
  - Output: Coverage matrix, API checklist, workflow checklist, fixture checklist, and integration-test targets.
  - Validation: Review proves no SDS #50 build-breakdown item is missing, first proof is not manual-only, and risky automation is limited until owner-service evidence and drills are current.

- **10.4 Validate SDS, service catalog, master plan, crosswalk, and queue alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, build-plan crosswalk, Phase 9 wording, queue state, queue progress, and build-plan progress.
  - Output: Updated backlinks and index rows for SDS #50, queue state update, queue progress update, and build-plan progress evidence.
  - Validation: JSON validation passes; local link checks pass; queue validation confirms `050-build-plan` is complete, no materialized task is running, and `051-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders source alignment, contracts, schemas, intake, preflight, strategy templates, traffic steps, health gates, promotion, rollback/freeze, client surfaces, evidence, simulation, operations, governance hooks, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.

## Alignment Review

- The sub-build plan keeps Release Strategy Service first build work in master Phase 9 because rollout intent depends on signed application-intent manifests, package validation reports, deployment-plan refs, route refs, policy decisions, health evidence, restore refs, and metering/billing hook readiness.
- The plan treats Deployment Planner as the owner of ordered deployment graphs, execution cursors, rollback plan refs, and deployment timelines; Release Strategy Service consumes refs and supervises rollout policy.
- The plan treats Package Validator as the owner of validation reports and package safety evidence; Release Strategy Service requires report refs and package/version facts but does not validate internals.
- The plan treats Overmesh as the owner of route weights, route bindings, endpoint health, and traffic-shift command truth; Release Strategy Service requests route steps and records evidence.
- The plan treats Overwatch as the owner of health, audit, trace, evidence durability, and release event storage; Release Strategy Service consumes health gate evidence and emits events.
- The plan treats Overguard as the policy owner for strategy, channel, actor, freeze, rollback, break-glass, workload class, data class, and approval decisions.
- The plan treats Backup and Restore Service and Failover and Recovery Coordinator as recovery owners; Release Strategy Service checks restore refs, requests rollback, and escalates incident-grade recovery instead of performing restore or failover itself.
- The plan treats Overmeter, ORU Account Service, Overbill, and Seal Ledger as accounting owners; Release Strategy Service emits release orchestration usage and confirms hooks without pricing, revenue, balance, payout, invoice, or ledger mutation.
- The plan preserves master Phase 0 through Phase 13 ordering and uses earlier phases as prerequisites plus Phase 13 as governance/security/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first core service/contracts, native Overrid boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, and no conventional cloud, blockchain, NFT, pricing, revenue, or customer-count assumptions.
