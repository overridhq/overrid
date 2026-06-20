# SUB BUILD PLAN #78 - Migration Tooling

Attached SDS: [SDS #78 - Migration Tooling](../sds/governance_ops/migration_tooling.md)

## Purpose

This sub-build plan turns SDS #78 into an implementation sequence for Migration Tooling. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Migration Tooling is the planned, resumable, audited migration coordinator for services, tenants, data, routes, workloads, accounting refs, event streams, and backbone responsibilities. It coordinates migration plans, inventories, preflight reports, dependency graphs, step cursors, checkpoints, cutover windows, integrity reports, rollback records, and replay bundles. It does not own deployment planning, live failover, raw data storage, route truth, ledger truth, backup content, release strategy, or direct mutation of owner-service state.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #78: Migration Tooling](../sds/governance_ops/migration_tooling.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, states, policy/security, metering, operations, failure modes, validation, build breakdown, downstream handoffs, and resolved open-question decisions. |
| [Migration Tooling plan](../service_catalog/governance_ops/migration_tooling.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, canonical JSON plus JSON Schema, deterministic fixtures, signed command envelopes, trace ids, idempotency keys, stable errors, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overpass identities, Overtenant scopes, Overkey signing refs, Overgate ingress, Overregistry service records, Overwatch audit refs, Overqueue primitives, and service account boundaries. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard decisions, Workload Classifier facts, Policy Dry-Run previews, Oververify evidence, Overclaim refs, Challenge Task refs, reason codes, and deny-by-default eligibility checks. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Overmeter usage refs, ORU/Seal Ledger/Overbill accounting truth, Overgrant grant refs, Provider Payout refs, Overclaim dispute refs, and the rule that Migration Tooling verifies accounting continuity without mutating ledger truth. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies the first grid-migration slice for non-critical system-service moves, founder-hardware migration evidence, restore/failover/rollback drill prerequisites, and emergency fallback gates. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies Overbase records, Overstore artifact refs, Overvault grant refs, Universal Namespace refs, Overmesh route refs, native storage/vault boundaries, and data-copy/replay substrates. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies Deployment Planner, Release Strategy, Package Validator, Overpack package refs, rollout/rollback refs, health gates, route gates, and package compatibility evidence. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Supplies trusted federation templates, participant boundaries, public-interest pool refs, purpose tags, cross-tenant grant refs, and migration scope for known participant pools. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Supplies public-provider onboarding, public sandbox limits, public-provider fraud/reputation/challenge refs, payout holds, and public low-sensitivity migration constraints. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Supplies native app, wallet, assistant, search, messaging, social, maps, workspace, mobile, and stewardship-interface consumers of migration status, notices, cutovers, and redacted reports. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Controls the full first build point for Migration Tooling, including mature migration planning, approval, execution, rollback, reporting, security review, reliability drills, and scale hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #78 aligned to Phase 13 as the full first build point while preserving Phase 7 grid-migration tooling as an earlier limited slice. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services and contracts, Tokio where bounded workers are needed, Axum/Tower/Hyper-style HTTP where a service boundary exists, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, TypeScript for client surfaces only, native Overrid service boundaries, and no conventional database/object-store/vault/queue/compliance SaaS product boundary, Kubernetes-first architecture, blockchain, NFT, hardcoded pricing, revenue, or customer-count assumptions. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 7, 8, 9, 10, 11, 12, and 13 | Attach SDS #78, preserve Phase 13 as the full first build point, record Phase 7 early slice, and freeze Migration Tooling authority. |
| 2 | Master Phases 0, 1, 4, 5, 7, 8, 9, and 13 | Define Rust contracts, schemas, state machines, stable errors, fixtures, and replay inputs. |
| 3 | Master Phases 1, 4, 7, 8, 9, and 13 | Implement plan creation, inventory snapshots, preflight aggregation, dependency graphs, and approval readiness. |
| 4 | Master Phase 7 with prerequisites from Phases 0, 1, 4, 5, 8, and 9 | Build the limited non-critical system-service migration slice for founder-hardware-to-grid proof. |
| 5 | Master Phases 1, 4, 5, 7, 8, 9, and 13 | Implement idempotent step execution and owner-service command envelopes for copy, replay, drain, verify, route prebind, and checkpoint work. |
| 6 | Master Phases 4, 5, 7, 8, 9, 12, and 13 | Implement cutover windows, go/no-go review, route rebinding coordination, monitoring, and status projections. |
| 7 | Master Phases 5, 7, 8, 9, and 13 | Implement pause, resume, cancel, rollback, fallback, checkpoint conflict handling, and rollback evidence. |
| 8 | Master Phases 8, 10, 11, 12, and 13 | Expand migration classes for tenant, data, route, namespace, federation, public-provider, native-app, and AI-routing moves with redacted status. |
| 9 | Master Phase 13 with evidence from Phases 0 through 12 | Implement metering, observability, reporting exports, Incident Response handoffs, threat/security review, reliability drills, and scale hardening. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Migration Tooling uses Rust-first shared contracts and service-facing APIs for migration plans, inventories, preflight reports, graphs, steps, checkpoints, cutover windows, integrity reports, rollback records, and replay bundles. TypeScript is acceptable only for generated client/admin/operator surfaces and must call Overrid APIs without becoming a migration authority.
- Contracts, migration records, lifecycle states, state-machine transitions, stable errors, replay bundles, integrity reports, rollback manifests, and deterministic fixtures use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf may be generated later only through the shared contract layer.
- Mutating APIs require signed actor/service envelopes, tenant/system scope, trace id, idempotency key, operator/steward approval refs where required, policy refs, evidence refs, schema version, stable errors, and Overwatch audit refs.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for inventories, package refs, checkpoint manifests, copy/replay evidence, integrity reports, rollback records, replay bundles, fixture inputs, and deterministic comparison artifacts. BLAKE3 must not be described as encryption.
- Structured migration state, evidence refs, data refs, secret grant refs, route refs, queue refs, ledger refs, package refs, usage refs, and replay must use native Overrid boundaries such as Deployment Planner, Grid-Resident Service Packager, Package Validator, Backup and Restore Service, Failover and Recovery Coordinator, Release Strategy Service, Overmesh, Overbase, Overstore, Overvault, Overqueue, Overregistry, Overwatch, Overguard, Compliance Boundary Service, Seal Ledger, ORU Account Service, Overbill, Overmeter, Provider Payout Service, Overgrant, Incident Response Service, Stewardship Reporting Service, SDK, CLI, and Admin/Developer UI.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Elasticsearch, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, a migration SaaS, Kubernetes-first orchestration, blockchain, NFTs, external payment custody, hardcoded pricing, revenue forecasts, customer-count assumptions, raw private-data exports, raw secrets, direct data-store mutation, direct route truth mutation, ledger mutation, queue mutation, vault mutation, deployment-plan ownership, live failover ownership, or release-strategy ownership the Migration Tooling authority.

## Phase 1: SDS Attachment, Phase Boundary, And Migration Authority

### Work Items

- **1.1 Attach the build plan to SDS #78.**
  - Design: Link this document from the Migration Tooling SDS, service plan, master build plan, Phase 7 plan, Phase 13 plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/governance_ops/migration_tooling.md`, `docs/service_catalog/governance_ops/migration_tooling.md`, `docs/build_plan/master_plan.md`, `docs/build_plan/phase_07_grid_resident_backbone.md`, `docs/build_plan/phase_13_governance_compliance_scale_hardening.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #78 returns both the Migration Tooling SDS and this sub-build plan.

- **1.2 Preserve Phase 13 as the full first build point.**
  - Design: Keep mature migration coordination in Phase 13 because migration plans depend on identity, tenancy, policy, accounting refs, recovery tooling, package validation, storage/vault refs, federation/public controls, native-app status, reporting, and compliance prerequisites.
  - Output: Phase-gate note that Phase 7 only starts a limited grid-migration slice for non-critical system-service proof, while Phase 13 builds full migration planning, approval, rollback, evidence, and governance hardening.
  - Validation: Review proves the plan does not move full tenant, data, regulated, accounting-impacting, public-provider, native-app, or founder-hardware removal authority into Phase 7.

- **1.3 Freeze the Migration Tooling ownership boundary.**
  - Design: Record that Migration Tooling owns migration plan records, inventories, preflight aggregation, migration graphs, session state, step cursors, checkpoints, cutover windows, integrity reports, rollback records, and replay bundles.
  - Output: Ownership checklist for architecture, APIs, implementation, operations, governance, and review gates.
  - Validation: Review confirms the service does not own Deployment Planner plans, live failover decisions, route truth, primary stores, backup artifacts, ledger truth, queue truth, vault contents, package artifacts, release strategy, or owner-service finality.

- **1.4 Carry forward resolved SDS #78 decisions.**
  - Design: Preserve the selected Phase 7 non-critical candidate, source-authoritative snapshot-plus-append-only replay model, automation approval split, cutover-window classes, and evidence-gated founder fallback rules as explicit constraints.
  - Output: Resolved-decision checklist covering non-primary Overwatch/internal-observability replica eligibility, single-writer source authority until cutover, service-account safe operations, operator approval gates, native-app/AI/public-provider windows, and founder-hardware exit drills.
  - Validation: Review rejects active/active multi-writer assumptions for early data migration, high-risk automation without approval, public-provider cutovers without conservative windows, and founder-hardware exit without required drill evidence.

- **1.5 Define upstream and downstream service boundaries.**
  - Design: Record how Deployment Planner, Grid-Resident Service Packager, Package Validator, Backup and Restore, Failover and Recovery, Release Strategy, Overmesh, Overbase, Overstore, Overvault, Overqueue, Overregistry, Overwatch, Overguard, Compliance Boundary, accounting services, Incident Response, Stewardship Reporting, SDK, CLI, and operator UI interact through refs and command envelopes.
  - Output: Boundary matrix naming allowed reads, owned writes, required refs, denied direct mutations, policy refs, evidence refs, rollback refs, usage refs, audit refs, redaction classes, and owner-service finality.
  - Validation: Review confirms Migration Tooling exchanges signed refs/events/commands and never reaches into private storage, route tables, ledgers, queues, vaults, package stores, or live failover controls directly.

## Phase 2: Contracts, Schemas, State Machines, Stable Errors, And Fixtures

### Work Items

- **2.1 Create the Migration Tooling Rust contract module.**
  - Design: Add contract types for migration plans, inventories, preflight reports, graphs, steps, checkpoints, cutover windows, integrity reports, rollback records, replay bundles, stable errors, lifecycle states, and owner-service command envelopes.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, migration-type enums, lifecycle enums, side-effect classes, redaction classes, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from direct deployment planning, failover execution, primary storage, routing truth, ledger truth, backup content, and release strategy.

- **2.2 Define plan, inventory, preflight, and graph schemas.**
  - Design: Model `migration_plan`, `source_destination_inventory`, `migration_preflight_report`, and `migration_graph` with source/destination refs, scope, risk class, freshness, owner refs, dependency edges, side-effect classes, required freezes, rollback links, and verification requirements.
  - Output: JSON Schema files, valid examples, invalid examples, stable error mapping, and fixtures for service, tenant, workload, data, route, namespace, queue, accounting, and backbone migrations.
  - Validation: Schema tests reject missing source/destination refs, missing owner service, missing tenant/system scope, missing risk class, stale inventory, unversioned schemas, unbounded side effects, missing rollback requirements, and missing audit refs.

- **2.3 Define step, checkpoint, cutover, integrity, rollback, and replay schemas.**
  - Design: Model `migration_step`, `migration_checkpoint`, `cutover_window`, `integrity_check_report`, `rollback_record`, and `migration_replay_bundle` with idempotency keys, cursors, expected outputs, timeout/retry policy, owner-service refs, verification refs, rollback deadlines, and replay manifests.
  - Output: Schema set, lifecycle state enums, HTTP/API mapping, audience-classed projection schemas, deterministic examples, and golden replay fixtures.
  - Validation: Tests prove outputs are deterministic from stored refs and cannot include raw secrets, private data, payment credentials, raw vault contents, private topology, exploit details, or security-sensitive migration internals for unauthorized audiences.

- **2.4 Define state machines and stable errors.**
  - Design: Preserve SDS #78 plan, step, and cutover lifecycles and map every transition to allowed actors, required evidence, idempotency behavior, owner-service responses, and rollback safety.
  - Output: Transition matrix for `draft`, `preflighting`, `blocked`, `ready_for_approval`, `approved`, `executing`, `paused`, `cutover_pending`, `cutover_in_progress`, `verifying`, `completed`, `rollback_required`, `rolling_back`, `rolled_back`, `failed`, and `cancelled`, plus stable errors.
  - Validation: Tests reject invalid transitions, duplicate side effects, missing idempotency keys, rollback after unsafe irreversible cutover, conflicting cursors, and finalization while Overwatch evidence is not durable.

- **2.5 Create deterministic fixtures and golden traces.**
  - Design: Build fixtures for plan creation, inventory, preflight blockers, graph ordering, safe Phase 7 non-critical migration, snapshot-plus-replay, queue drain, route prebind, ledger verification, cutover, rollback, integrity report, and replay.
  - Output: Fixture directory, canonical inputs, expected states, expected events, expected stable errors, BLAKE3/content hashes, usage refs, audit refs, redacted projections, and replay bundles.
  - Validation: Fixture tests produce stable ids, hashes, lifecycle states, reason codes, redaction behavior, usage refs, audit refs, and replay output across repeated runs.

## Phase 3: Plan Creation, Inventory, Preflight, Graph, And Approval Readiness

### Work Items

- **3.1 Implement migration plan creation and revisions.**
  - Design: Accept plan drafts for service, tenant, workload, data, route, namespace, queue, accounting, public/federation pool, AI-routing, native-app, or backbone migration only with explicit source, destination, scope, migration type, owner refs, requested cutover window, risk class, trace id, and idempotency key.
  - Output: `POST /migrations/plans`, plan revision records, audit events, stable errors, and authorized read projections.
  - Validation: Tests reject missing scope, owner service, source/destination refs, risk class, trace id, idempotency key, or unsupported migration type, and duplicate idempotency keys return the original result.

- **3.2 Implement source and destination inventory snapshots.**
  - Design: Collect service, tenant, workload, data store, route, namespace, queue, ledger, vault, package, node, policy, backup, restore, capacity, and health facts through owner-service read APIs with freshness and redaction classes.
  - Output: Inventory snapshot workers, freshness metadata, owner refs, source refs, BLAKE3/content hashes for snapshot manifests, and diffable inventory reports.
  - Validation: Tests prove stale, missing, forged, unauthorized, or mismatched inventory facts block preflight or require review before execution.

- **3.3 Implement side-effect-free preflight checks.**
  - Design: Aggregate Overguard policy, System-Service Workload Class, Compliance Boundary, backup readiness, restore point freshness, route ownership, package compatibility, destination capacity/trust, ledger checkpoint, queue drain, vault grant, release gate, and failure-domain checks.
  - Output: `POST /migrations/plans/{plan_id}/preflight`, reason-coded blockers, readiness summaries, owner-service refs, policy refs, and audit events.
  - Validation: Tests prove regulated or secret-bearing workloads cannot move to lower-trust destinations, destructive steps require restore evidence, ledger-impacting plans require accounting checkpoints, and route changes require route ownership.

- **3.4 Implement migration graph generation.**
  - Design: Build ordered steps from inventories and preflight results with dependency edges, side-effect classes, freeze requirements, parallelization groups, owner-service command refs, verification steps, and rollback links.
  - Output: `migration_graph` records, graph visualization/projection, topological ordering, conflict diagnostics, and replay fixtures.
  - Validation: Tests prove graph order keeps backup/restore and checkpoint steps before destructive/cutover work, verifies data and ledger continuity before route shifts, and prevents unsafe parallelization of dependent state changes.

- **3.5 Implement plan approval readiness.**
  - Design: Require operator/steward/service-owner approval based on scope, data class, regulated markers, secret-bearing state, accounting impact, public-provider exposure, system-service impact, and cutover reversibility.
  - Output: `POST /migrations/plans/{plan_id}/approve`, approval envelopes, go/no-go prerequisites, readiness state, audit refs, and stable errors for missing approvals.
  - Validation: Tests prove regulated, secret-bearing, accounting-impacting, public-provider, system-service, primary route shift, stateful writer promotion, rollback override, and founder-hardware removal plans cannot execute without required approvals.

## Phase 4: Phase 7 Non-Critical Grid Migration Slice

### Work Items

- **4.1 Implement non-critical migration candidate gating.**
  - Design: Start with the SDS-approved non-primary Overwatch/internal-observability replica and require that it not own the primary append/checkpoint head, tenant-private data, raw secrets, or user-facing outage risk.
  - Output: Candidate eligibility checklist, `eligible_for_test` and `eligible_for_noncritical` states, owner-service refs, risk-class records, and blocked-state diagnostics.
  - Validation: Tests reject primary write heads, tenant-private data stores, raw-secret workloads, regulated workloads, user-facing critical routes, and unrebuildable services as Phase 7 first candidates.

- **4.2 Integrate package, backup, failover, and release readiness.**
  - Design: Require Package Validator, Grid-Resident Service Packager, System-Service Workload Class, Backup and Restore, Release Strategy, Failover and Recovery, and Overwatch evidence to agree on package validity, rollback path, restore evidence, and drill readiness.
  - Output: Readiness adapter set, required ref checklist, approval blockers, and Phase 7 proof bundle.
  - Validation: Tests prove a non-critical plan cannot advance when package validation, restore evidence, failover path, release gate, rollback path, or Overwatch audit evidence is stale or missing.

- **4.3 Implement source-authoritative snapshot-plus-replay for the Phase 7 slice.**
  - Design: Keep the source authoritative until cutover; restore the destination from checkpointed snapshot, replay ordered events/change records with idempotency keys and monotonic cursors, run shadow integrity checks, and fence writers before accepting writes.
  - Output: Snapshot refs, replay cursor records, destination restore refs, writer-fencing refs, shadow integrity reports, and cutover evidence.
  - Validation: Tests prove early migration does not use active/active writes, rejects replay gaps, blocks cutover when writer fencing is missing, and preserves source authority until route/queue drain evidence is complete.

- **4.4 Implement founder-hardware fallback evidence tracking.**
  - Design: Track founder hardware as emergency fallback by evidence gate, not by convenience duration, and require drill evidence before removing it from the normal path.
  - Output: Founder-fallback status records, full-backbone drill refs, backup/restore/failover/rollback/queue recovery/Overwatch reconciliation/route-shift evidence, severe follow-up blockers, and cold-recovery/decommission states.
  - Validation: Tests prove founder hardware cannot leave normal production path without two consecutive full-backbone cutover drills, at least one failure-injection drill, required per-critical-service drills, and no unresolved `sev_0` or `sev_1` follow-up actions.

- **4.5 Produce Phase 7 migration proof reports.**
  - Design: Package the Phase 7 migration slice as evidence that the backbone can migrate one non-critical service without manual state edits, route edits, ledger edits, or private-data leakage.
  - Output: Phase 7 proof report, integrity report, rollback drill record, Overwatch audit bundle, Incident Response drill handoff, and Stewardship Reporting redacted summary.
  - Validation: Review confirms the proof report includes expected behavior, actual behavior, evidence refs, blocker refs, rollback outcome, monitoring window, and no raw private topology or sensitive evidence for public views.

## Phase 5: Idempotent Step Execution And Owner-Service Commands

### Work Items

- **5.1 Implement execution start and resume.**
  - Design: Start or resume migration execution from stored cursors only after plan approval, fresh preflight, graph readiness, safe cutover window, and owner-service availability checks pass.
  - Output: `POST /migrations/plans/{plan_id}/execute`, execution sessions, resume tokens, cursor conflict detection, audit refs, and stable errors.
  - Validation: Tests prove duplicate execute calls do not duplicate side effects, stale preflight blocks execution, and conflicting cursors require operator review.

- **5.2 Implement owner-service command envelopes.**
  - Design: Dispatch copy, restore, replay, queue drain, route prebind, namespace update, vault grant check, ledger verification, integrity check, rollback, and monitoring commands through owner-service APIs with signed envelopes and idempotency keys.
  - Output: Command envelope schema, dispatch adapter, response records, retry state, timeout handling, owner denial preservation, and timeline events.
  - Validation: Tests prove owner-service denial is preserved with reason codes and Migration Tooling never edits data, objects, secrets, routes, queues, ledgers, package artifacts, or release truth directly.

- **5.3 Implement data copy and object transfer coordination.**
  - Design: Coordinate Overbase, Overstore, and backup/restore copy work through refs, cursors, hashes, integrity reports, retention/tombstone constraints, and redaction classes.
  - Output: Copy session refs, object transfer refs, chunk/hash manifests, retry records, mismatch refs, quarantine refs, and integrity-check inputs.
  - Validation: Tests prove mismatched copies quarantine destination refs, block cutover, preserve source authority, and do not expose raw private data in migration records.

- **5.4 Implement event replay, queue drain, and ledger verification adapters.**
  - Design: Coordinate ordered event replay, queue drain/freeze, ledger checkpoint/reconciliation, usage continuity, payout-hold awareness, and accounting-impact blockers without mutating owner-service truth.
  - Output: Replay cursor records, queue drain refs, ledger checkpoint refs, reconciliation reports, accounting blockers, and stable errors such as `ledger_checkpoint_missing`, `queue_not_drained`, and `migration_cursor_conflict`.
  - Validation: Tests prove event replay gaps, queue drain timeouts, ledger checkpoint mismatches, usage reconciliation gaps, or payout-risk blockers stop cutover and create evidence for owner review.

- **5.5 Implement route prebinding, vault grant, and namespace checks.**
  - Design: Coordinate Overmesh route prebinds, Universal Namespace authority checks, Overvault grant refs, Overkey/key metadata refs, and route ownership checks before cutover.
  - Output: Route prebind refs, namespace authority refs, vault grant check refs, route ownership blockers, secret-bearing migration blockers, and redacted status records.
  - Validation: Tests prove route rebinding cannot proceed without owner authority, secret-bearing migrations cannot proceed without vault grant refs, and user-visible status does not reveal private topology or secret refs.

## Phase 6: Cutover Windows, Go/No-Go Review, Route Rebinding, And Monitoring

### Work Items

- **6.1 Implement cutover window classes.**
  - Design: Class windows by user impact, reversibility, data class, service criticality, native-app exposure, AI-route quality/safety risk, public-provider risk, accounting impact, and compliance markers.
  - Output: Cutover class schema, defaults for native apps, AI-routing migrations, public-provider flows, system-service moves, low-risk route/read-model shifts, and blocked high-risk windows.
  - Validation: Tests prove native-app migrations require notice and rollback bounds, AI-routing migrations require canary and quality/safety rollback thresholds, and public-provider flows require conservative windows, capacity warm-up, fraud/reputation checks, and allowed payout/finality holds.

- **6.2 Implement go/no-go review.**
  - Design: Require fresh readiness checks, approved rollback contract, owner-service ack, monitoring plan, Overwatch durability, freeze/drain state, accounting continuity, route readiness, and redaction profile before cutover.
  - Output: `POST /migrations/plans/{plan_id}/cutover`, go/no-go checklist, approver refs, readiness diagnostics, cutover state changes, and stable errors.
  - Validation: Tests prove cutover cannot start with stale evidence, missing rollback contract, missing monitoring window, unresolved queue drain, replay gap, ledger mismatch, route denial, or Overwatch degradation.

- **6.3 Implement route rebinding coordination.**
  - Design: Coordinate Overmesh and Universal Namespace rebinding through owner-service commands, route state refs, traffic modes, dual-run scope, rollback deadline, and monitoring state.
  - Output: Route rebinding refs, traffic-shift timeline, dual-run status, rollback deadline, owner-service responses, and redacted public/tenant status.
  - Validation: Tests prove route truth remains in owner services, partial route rebinds pause or roll back from checkpoints, and public/tenant projections hide private topology.

- **6.4 Implement post-cutover monitoring.**
  - Design: Track copied data refs, event replay counts, ledger checkpoints, route refs, queue state, namespace refs, vault grants, package versions, health windows, and mismatch refs after traffic shifts.
  - Output: Monitoring session records, integrity summaries, alert rules, stale monitoring diagnostics, and completion blockers.
  - Validation: Tests prove completion is blocked until monitoring windows close cleanly and mismatch, stale evidence, health regression, route failure, ledger mismatch, or queue gap opens rollback or incident workflows.

- **6.5 Implement tenant and user status projections.**
  - Design: Provide authorized status for tenant admins, service owners, affected users, native apps, and operators using redaction classes that avoid private topology, provider internals, security-sensitive details, raw secrets, and private data.
  - Output: `GET /migrations/plans/{plan_id}`, timeline/status projections, cutover notices, correction notices, and access-denied stable errors.
  - Validation: Tests prove each audience sees only allowed scope and that public/provider/native-app views exclude sensitive migration internals.

## Phase 7: Pause, Resume, Cancel, Rollback, Fallback, And Checkpoint Safety

### Work Items

- **7.1 Implement safe pause and resume.**
  - Design: Pause at the next safe checkpoint with step state, owner-service state, cursor positions, data/event/queue/ledger refs, rollback options, and pending side-effect notes.
  - Output: `POST /migrations/plans/{plan_id}/pause`, resume tokens, checkpoint records, owner-service pending-state records, and paused-state diagnostics.
  - Validation: Tests prove pause waits for safe checkpoint, resume uses stored cursors, duplicate resume is idempotent, and unsafe pause requests create clear blocker records.

- **7.2 Implement cancel before irreversible cutover.**
  - Design: Allow cancel only while the graph has not crossed declared irreversible cutover points and required cleanup/owner-service compensation can be recorded.
  - Output: `POST /migrations/plans/{plan_id}/cancel`, cancel eligibility checks, cleanup refs, skipped-by-revision records, and audit events.
  - Validation: Tests prove cancellation is blocked after unsafe irreversible cutover, preserves prior evidence, and does not delete step history.

- **7.3 Implement rollback plan records and execution.**
  - Design: Generate rollback plans from graph rollback links, restore refs, route fallback refs, data rewind limits, queue/ledger checkpoint state, package version pins, and owner-service response requirements.
  - Output: `POST /migrations/plans/{plan_id}/rollback`, rollback session records, rollback command refs, fallback route refs, restore refs, result refs, incident refs, and stable errors.
  - Validation: Tests prove rollback requires policy and checkpoint safety, preserves original failure evidence, records owner-service responses, and cannot rewrite completed migration history.

- **7.4 Implement fallback and degraded-mode handling.**
  - Design: Handle route rebind rejection, queue drain timeout, event replay gap, ledger mismatch, vault grant missing, Overwatch degradation, and owner-service outage by pausing, falling back, or opening incident handoff.
  - Output: Fallback refs, degraded-mode records, local bounded spool for Overwatch degradation, Incident Response handoff refs, and closure blockers.
  - Validation: Tests prove finalization is blocked until Overwatch evidence is durable, owner-service denials are preserved, and fallback never silently drops data, events, queue state, or accounting evidence.

- **7.5 Implement checkpoint conflict and replay mismatch handling.**
  - Design: Detect conflicting cursors, repeated owner-service responses, replay count mismatch, package version drift, route state drift, queue state drift, ledger checkpoint drift, and stale inventory between execution attempts.
  - Output: Conflict diagnostics, correction records, re-preflight requirements, rollback-required state, and replay mismatch reports.
  - Validation: Tests prove conflicts stop execution, require explicit correction or reapproval, and appear in replay bundles with stable reason codes.

## Phase 8: Tenant, Data, Route, Federation, Public-Provider, Native-App, And AI Migration Classes

### Work Items

- **8.1 Implement tenant workload and data-store migration classes.**
  - Design: Add migration profiles for tenant workloads, Overbase records, Overstore artifacts, Overvault grant refs, queues, event streams, namespace records, and route bindings with strict data-class and compliance markers.
  - Output: Tenant/data migration profiles, policy fact bundles, backup/restore prerequisites, data-class blockers, redacted status, and test fixtures.
  - Validation: Tests prove tenant-private, secret-bearing, regulated, or high-compliance migrations cannot target lower-trust destinations and cannot expose raw private data in records or status views.

- **8.2 Implement route, namespace, and service-ref migration classes.**
  - Design: Coordinate route rebinding, namespace authority, service registry refs, package version refs, and route rollback for service moves and native app endpoints.
  - Output: Route/namespace migration profiles, service-ref update refs, prebind/canary states, rollback deadlines, and owner-service response mappings.
  - Validation: Tests prove route truth remains in Overmesh/Universal Namespace/owner services and that partial route migrations pause or roll back without manual edits.

- **8.3 Implement trusted federation and public-interest migration classes.**
  - Design: Support moves between private swarms, trusted federation pools, and public-interest pools with federation template refs, participant roles, capacity contributions, grant/purpose refs, dispute boundaries, and stewardship evidence.
  - Output: Federation migration profiles, public-interest migration records, Overgrant and Purpose Tag refs, participant-boundary blockers, and reporting exports.
  - Validation: Tests prove federation moves require known participant boundaries, purpose eligibility, accounting/dispute boundaries, and public-interest reporting refs.

- **8.4 Implement public-provider and public-safe pool migration classes.**
  - Design: Support only low-sensitivity public-provider moves with public sandbox checks, reputation/fraud/challenge refs, payout-hold awareness, no in-flight high-risk jobs, and conservative cutover windows.
  - Output: Public-provider migration profiles, sandbox eligibility refs, fraud/reputation blockers, public-pool warm-up refs, payout/finality hold refs, and redacted status.
  - Validation: Tests prove private, regulated, tenant-sensitive, secret-bearing, system-service, or high-risk jobs cannot migrate into public-provider pools.

- **8.5 Implement native-app and AI-routing migration classes.**
  - Design: Support native app service moves, read-model shifts, AI-routing route migrations, assistant/RAG adapter moves, and public status with privacy redaction, canary windows, policy dry-run comparison, and rollback thresholds.
  - Output: Native-app migration profiles, AI-routing migration profiles, canary metrics, quality/safety threshold refs, privacy/redaction checks, and user-visible notice records.
  - Validation: Tests prove AI-routing cutovers fall back when classification, privacy, model-resource, or quality/safety evidence is stale, and native-app notices avoid raw private data or internal topology.

## Phase 9: Evidence, Metering, Observability, Reporting, Security, And Scale Hardening

### Work Items

- **9.1 Implement migration metering and accounting refs.**
  - Design: Emit usage refs for preflight, inventory, data copy, object transfer, event replay, ledger verification, queue drain, route rebinding, integrity checks, rollback, operator review, reporting, and replay.
  - Output: Usage event schema, Overmeter refs, plan/session attribution, system-service usage classification, accounting handoff refs, and reconciliation fixtures.
  - Validation: Tests prove migration overhead is visible as system-service usage where applicable and Migration Tooling never creates, edits, erases, or finalizes ledger history.

- **9.2 Implement operational metrics and alerts.**
  - Design: Track plan backlog, blocked preflights, step age, copy/replay progress, checkpoint freshness, cutover readiness, rollback readiness, verification status, stale backups, queue drain timeouts, ledger mismatches, route failures, vault grant failures, and rollback failures.
  - Output: Metrics schema, alert rules, operator dashboard refs, escalation records, and runbook notes.
  - Validation: Drills prove stale backup, destination ineligibility, queue drain timeout, ledger mismatch, route rejection, vault grant failure, cutover overrun, verification mismatch, and rollback failure alerts fire and create timeline evidence.

- **9.3 Implement audit, incident, stewardship, and governance exports.**
  - Design: Export redacted migration evidence to Overwatch, Incident Response, Stewardship Reporting, PIP Registry, Compliance Boundary, and governance review without exposing private data, raw secrets, private topology, or security-sensitive details.
  - Output: Evidence export APIs, report manifests, redaction profiles, audit packages, public-summary refs, and replay bundle refs.
  - Validation: Tests prove public and stewarded exports are specific enough to audit decisions while excluding unauthorized private payloads and security-sensitive migration details.

- **9.4 Run threat modeling and security review gates.**
  - Design: Review migration authority misuse, stale inventory, forged owner refs, replay abuse, route hijack, namespace drift, ledger checkpoint spoofing, vault grant leakage, rollback override abuse, founder fallback misuse, public-provider migration leakage, and AI-route rollback failure.
  - Output: Threat model entries, security review findings, mitigations, owners, acceptance criteria, tests, and residual-risk records.
  - Validation: Review confirms each high-risk finding has a mitigation, test, monitor, accepted-risk record, or blocker before broad migration support ships.

- **9.5 Run reliability and scale drills.**
  - Design: Drill non-critical service migration, data-copy mismatch, event replay gap, queue drain timeout, route rebind rejection, ledger mismatch, Overwatch outage, owner-service outage, rollback failure, founder-hardware cutover, native-app notice, AI-route fallback, and public-provider conservative cutover.
  - Output: Drill scenarios, expected behavior, actual behavior, evidence refs, follow-up actions, Incident Response refs, and readiness reports.
  - Validation: Drills prove pause/resume/rollback/replay behavior and create follow-up work for every mismatch or unhandled failure mode.

## Phase 10: Validation, Documentation, Queue State, And Implementation Handoff

### Work Items

- **10.1 Validate sub-build-plan structure and backlinks.**
  - Design: Check the `SUB BUILD PLAN #78` title, attached SDS link, phases 1 through 10, work-item Design/Output/Validation structure, and backlinks across SDS, service catalog, master plan, crosswalk, Phase 7, and Phase 13 docs.
  - Output: Validation report covering local Markdown links, heading order, work-item counts, backlink presence, and final newlines.
  - Validation: Focused structure checks and Markdown link checks pass.

- **10.2 Validate tech-stack guardrails and authority boundaries.**
  - Design: Scan the changed docs for accidental conventional product boundaries, direct mutation authority, Kubernetes-first assumptions, blockchain/NFT wording, pricing/revenue/customer-count assumptions, and BLAKE3-as-encryption mistakes.
  - Output: Guardrail scan report with negative-control explanations for allowed prohibition wording.
  - Validation: Review proves the plan respects `docs/overrid_tech_stack_choice.md` and native Overrid service boundaries.

- **10.3 Validate queue and progress metadata.**
  - Design: Mark `078-build-plan` complete, move the next incomplete build-plan task to SDS #79, and record validation evidence in build-plan progress and queue progress.
  - Output: Updated `.codex55_sds_queue/state.json`, `.codex55_sds_queue/progress.md`, and `docs/build_plan/progress.md`.
  - Validation: JSON validation passes and queue progress counts match the state file.

- **10.4 Refresh Docdex and verify retrieval.**
  - Design: Reindex the new sub-build-plan file and linked SDS/service/build-plan docs so future agents retrieve the aligned plan.
  - Output: Docdex targeted index refresh, search result for SDS #78, symbol extraction for the new plan, and DAG export for the final search session.
  - Validation: Docdex search returns the SDS #78 sub-build plan in the result set with SDS/service/master/crosswalk/phase evidence, and Docdex stats reflect the indexed document.

- **10.5 Prepare implementation handoff.**
  - Design: Summarize the SDS #78 implementation order, prerequisites, owner-service boundaries, test strategy, known blockers, and validation evidence for the next build or phase-execution task.
  - Output: Handoff note in build-plan progress and Docdex memory covering the final aligned scope.
  - Validation: Review confirms the handoff names Phase 7 early slice, Phase 13 full build, required owner services, no direct mutation boundaries, and test/validation prerequisites without unresolved alignment gaps.
