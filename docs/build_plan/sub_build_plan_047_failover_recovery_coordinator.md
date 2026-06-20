# SUB BUILD PLAN #47 - Failover and Recovery Coordinator

Attached SDS: [docs/sds/deployment_grid/failover_recovery_coordinator.md](../sds/deployment_grid/failover_recovery_coordinator.md)

## Purpose

This sub-build plan turns SDS #47 into an implementation sequence for Failover and Recovery Coordinator. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Failover and Recovery Coordinator is the live recovery controller for protected grid-resident backbone services. It owns health signal snapshots, failover decisions, recovery plans, recovery steps, writer guards, route-shift command envelopes, replacement-capacity requests, drill records, maintenance states, and recovery evidence bundles without storing backups, executing releases, validating packages, replacing Overwatch, or becoming a general-purpose orchestrator.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #47: Failover and Recovery Coordinator](../sds/deployment_grid/failover_recovery_coordinator.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Failover and Recovery Coordinator service plan](../service_catalog/deployment_grid/failover_recovery_coordinator.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, deterministic fixtures, signed command envelopes, stable reason codes, idempotency keys, trace ids, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate ingress, Overpass identity, Overtenant scope, Overkey signatures, Overregistry refs, Overwatch audit, Overqueue state, and system-service account prerequisites. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Oversched placement, Overlease reservations, Overrun/Overcell execution handoffs, Overqueue worker behavior, and Overmeter raw usage facts. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard decisions, System-Service Workload Class policy facts, node eligibility, workload/data classification, policy dry-run behavior, and replayable denial reasons. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies system-service usage recording, ORU/Overbill/Seal Ledger hooks, accounting evidence refs, and stewardship cost visibility without pricing or revenue assumptions. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Controls the first build point: protected system-service runtime, failover readiness, restore readiness, writer fencing, route shifting, drills, rollback readiness, and founder-hardware removal gates. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies later native Overbase, Overstore, Overvault, Universal Namespace Service, and Overmesh depth for structured-state, artifact, secret, and namespace recovery refs. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies later package/deployment/release handoffs, Deployment Planner timelines, Release Strategy boundaries, and system-service package command contracts. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies incident response, threat modeling, security review, reliability drills, migration tooling, compliance boundaries, audit exports, and public reporting hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #47 first build work aligned to master Phase 7, with Phase 8/9 integrations and Phase 13 governance, incident, security, and reliability hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf through shared contracts, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 9, and 13 | Attach SDS #47, freeze authority boundaries, preserve Phase 7 as the first build point, and record later storage/deployment/governance gates. |
| 2 | Master Phases 0, 1, 4, and 7 | Define Rust contracts, canonical schemas, lifecycle states, reason codes, fixtures, signed command envelopes, and evidence refs. |
| 3 | Master Phases 1, 3, 4, 7, and 8 | Implement signal ingestion, aggregation, normalization, freshness windows, conflict handling, and observe/no-action decisions. |
| 4 | Master Phases 1, 4, 7, and 13 | Implement read-only evaluation, Overguard authorization, system-service class gates, maintenance/break-glass policy, and denial evidence. |
| 5 | Master Phases 3, 4, 7, and 8 | Implement active-writer guards, Overlease/Overkey/store epoch fencing, drain/freeze preconditions, split-brain prevention, and critical write-head prerequisites. |
| 6 | Master Phases 1, 3, 4, 7, 9, and 13 | Implement recovery-plan templates, execution cursors, idempotent downstream commands, pause/abort/resume behavior, and Overwatch spool recovery. |
| 7 | Master Phases 3, 4, 7, 8, and 9 | Implement route shifts, queue worker failover, replacement capacity requests, restore-backed recovery, post-step verification, and owner-service boundaries. |
| 8 | Master Phases 7 and 13 | Implement simulation, shadow drills, non-critical live drills, stateful restore/failover drills, and founder-hardware exit rehearsals. |
| 9 | Master Phases 5, 6, 7, 9, 12, and 13 | Add operations/status views, CLI/SDK/admin surfaces, usage hooks, incident/stewardship/release/deployment handoffs, and redacted evidence access. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Failover and Recovery Coordinator core is a Rust service/module using shared contract crates, Tokio for bounded signal/recovery workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Signal snapshots, decisions, plans, steps, writer guards, drill records, command envelopes, events, evidence bundles, API objects, fixtures, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be generated later only through the shared contract layer.
- Mutating endpoints require signed service or operator envelopes, tenant/system scope, trace id, idempotency key, schema version, policy refs, stable reason codes, and append-only Overwatch events or local spool refs when Overwatch is degraded.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for health snapshots, plan inputs, command payload refs, writer-epoch/checkpoint refs, drill replay bundles, evidence bundles, and deterministic fixture comparison.
- Recovery records may point to Overwatch, Overmesh, Overqueue, Oversched, Overlease, Overguard, Backup and Restore Service, Grid-Resident Service Packager, System-Service Workload Class, Overbase, Overstore, Overvault, Overmeter, ORU, Overbill, Seal Ledger, Deployment Planner, Release Strategy Service, and Incident Response refs, but the coordinator must not become the owner of those services' truth.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, revenue projections, customer-count assumptions, raw secret storage, direct payment execution, release-strategy ownership, package-validation ownership, backup storage ownership, or external-provider products the platform boundary.

## Phase 1: SDS Attachment, Recovery Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #47.**
  - Design: Link this document from the Failover and Recovery Coordinator SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/deployment_grid/failover_recovery_coordinator.md`, `docs/service_catalog/deployment_grid/failover_recovery_coordinator.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #47 returns both the Failover and Recovery Coordinator SDS and this sub-build plan.

- **1.2 Freeze Failover and Recovery Coordinator authority boundaries.**
  - Design: Record that the coordinator owns signal snapshots, failover decisions, recovery plans, recovery steps, writer guards, route-shift envelopes, replacement-capacity requests, maintenance states, drill records, and evidence bundles.
  - Output: Ownership checklist for architecture and implementation reviews.
  - Validation: Review confirms the coordinator does not store backups, execute releases, validate packages, replace Overwatch, mutate route/queue/lease/store truth directly, choose rollout strategy, own primary structured state, or perform general orchestration outside protected system-service recovery.

- **1.3 Preserve master Phase 7 as the first build point.**
  - Design: Keep first implementation in Phase 7 after identity, policy, audit, accounting, private execution, and product-integration prerequisites exist.
  - Output: Phase-gate note that Phase 7 builds protected backbone recovery, Phase 8 adds native storage/vault/namespace depth, Phase 9 consumes package/deployment/release contracts, and Phase 13 hardens incident, compliance, security, and reliability review.
  - Validation: Review proves this plan does not move Phase 8 data-platform ownership into Phase 7, does not delay Phase 7 failover drills behind native apps, and does not reorder master Phase 0 through Phase 13.

- **1.4 Carry forward resolved SDS #47 decisions.**
  - Design: Preserve Overrid-owned single-writer fencing through Overlease, Overkey, store epochs/checkpoints, Overqueue/Overmesh drains, and Overwatch evidence; active/passive first for stateless or rebuildable surfaces; quorum-style replicated logs/checkpoints before founder fallback removal for critical write heads; freshness windows by class; bounded seed-hardware automation; and progressive drill cadence.
  - Output: Resolved-decision checklist tied to contracts, evaluation, writer guards, execution, drills, and founder-exit reviews.
  - Validation: Review rejects external lock-service assumptions, generic leader-election wording without Overrid evidence refs, critical write-head promotion without fencing/checkpoint proof, stale-signal destructive failover, broad seed-hardware automation, and founder-path removal without recent rehearsal evidence.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for System-Service Workload Class, Overwatch, Overmesh, Oversched, Overlease, Overqueue, Backup and Restore Service, Overguard, Grid-Resident Service Packager, Overkey, Overbase, Overstore, Overvault, Overmeter, ORU Account Service, Overbill, Seal Ledger, Deployment Planner, Release Strategy Service, Incident Response Service, Admin UI, CLI, and SDK.
  - Output: Boundary matrix listing owner, input refs, output refs, command contract, freshness rule, policy gate, writer/lock behavior, restore behavior, audit refs, redaction class, and downstream consumer.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, idempotency keys, and Overwatch events rather than privileged shared tables or hidden control paths.

## Phase 2: Rust Contracts, Schemas, Lifecycles, And Fixtures

### Work Items

- **2.1 Create the Failover and Recovery Rust contract module.**
  - Design: Add contract types for health signal snapshots, recovery policy inputs, failover decisions, recovery plans, recovery steps, active-writer guards, recovery drills, command envelopes, evidence bundles, API errors, events, redaction profiles, and audit exports.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, signal enums, action enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overwatch, Overmesh, Oversched, Overlease, Overqueue, Backup and Restore Service, Overguard, package validation, release strategy, storage, ledger, and incident-response internals.

- **2.2 Define canonical signal and decision schemas.**
  - Design: Model `health_signal_snapshot`, `recovery_policy_input`, and `failover_decision` with service/instance/node refs, signal type, severity, source refs, observed time, freshness window, confidence, data class, trigger refs, decision type, selected action, policy refs, operator refs, state, trace id, and idempotency key.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, and docs-facing examples.
  - Validation: Schema tests reject missing system scope, service id, source ref, observed time, freshness window, data class, policy refs where required, trace id, idempotency key, schema version, lifecycle state, or stable reason codes.

- **2.3 Define recovery plan, step, and writer guard schemas.**
  - Design: Model `recovery_plan`, `recovery_step`, and `active_writer_guard` with plan type, preconditions, route-shift spec, replacement lease refs, restore plan refs, lock refs, rollback action, approval requirements, executable step type, downstream command ref, leader ref, lease ref, lock epoch, quorum/checkpoint ref, last verified time, and state.
  - Output: Plan/step/guard schemas, lifecycle fixtures, valid/invalid examples, writer-epoch examples, and split-brain blocking examples.
  - Validation: Tests reject plans without preconditions, mutating steps without idempotency keys, promotion without writer guard, route finalization without policy refs, restore-backed promotion without restore refs, and writer guards with stale or missing epoch/checkpoint proof.

- **2.4 Define drill, event, and evidence schemas.**
  - Design: Model `recovery_drill`, event payloads, replay bundles, evidence bundles, follow-up refs, redaction profiles, and incident/stewardship handoff refs with affected service, affected instance/node when safe, trace id, decision/plan ids, policy refs, command refs, timing, findings, and redacted evidence refs.
  - Output: Drill schema, event schemas, evidence-bundle schema, redacted tenant/operator examples, incident handoff examples, and replay fixtures.
  - Validation: Tests reject evidence bundles without source refs, timing data, policy refs, command refs for mutating actions, redaction class, final state, or follow-up refs for failed drills.

- **2.5 Create deterministic recovery fixtures.**
  - Design: Build fixtures for no action, degraded observe, stale signals, conflicting signals, route shift, worker failover, active/passive promotion, restore-backed recovery, writer guard unknown, split-brain prevention, policy denial, Overwatch degraded spool, non-critical live drill, stateful drill, and founder-exit rehearsal.
  - Output: Fixture directory, canonical inputs, expected decisions, expected plans, command refs, event sequences, evidence bundles, reason codes, hashes, signatures, and redacted views.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, signatures, state transitions, denial reasons, audit refs, and idempotency outcomes across repeated runs.

## Phase 3: Signal Ingestion, Aggregation, Freshness, And Conflict Handling

### Work Items

- **3.1 Implement Overwatch health-signal ingestion.**
  - Design: Consume Overwatch health, trace, incident, anomaly, heartbeat, and audit events through bounded event intake, with synchronous `POST /recovery/signals/ingest` only where event delivery needs acknowledgement.
  - Output: Signal ingester, source-ref validator, dedupe logic, ingestion lag metrics, `failover_recovery.signal_ingested` events, and safe rejection reason codes.
  - Validation: Tests prove duplicate, malformed, unsigned, wrong-scope, stale, or unsupported health signals are rejected or deduped without mutating recovery decisions.

- **3.2 Implement route, queue, lease, scheduler, and backup-readiness adapters.**
  - Design: Normalize Overmesh endpoint/route health, Overqueue backlog and worker facts, Oversched placement failures, Overlease lease failures, and Backup and Restore Service restore-plan readiness into signal snapshots.
  - Output: Adapter interfaces, normalized fact model, source freshness metadata, dependency availability states, and adapter-specific reason codes.
  - Validation: Integration tests prove each adapter records refs to owner-service facts, does not own downstream truth, distinguishes missing facts from negative facts, and blocks destructive action when required owner-service evidence is unavailable.

- **3.3 Enforce freshness windows by workload class and action type.**
  - Design: Implement SDS #47 freshness classes: test route shifts may use evidence up to 120 seconds old, non-critical route shifts require facts no older than 60 seconds, control-plane route shifts require route/queue/lease/writer evidence no older than 30 seconds, and active primary-path shifts require direct heartbeat or endpoint evidence no older than 15 seconds plus writer/quorum/fencing evidence no older than 30 seconds.
  - Output: Freshness policy table, class-specific validators, stale evidence reason codes, and policy override hooks that can only tighten windows.
  - Validation: Tests prove stale evidence downgrades automation to observe, drain, or escalate and never permits destructive failover, promotion, or route finalization.

- **3.4 Normalize signals and resolve conflicts.**
  - Design: Aggregate signals by service, instance, route, node, workload class, criticality, data class, and dependency readiness while preserving source confidence and disagreement.
  - Output: Aggregation engine, conflict classifier, confidence calculation, normalized snapshot query, and `conflicting_signals` reason codes.
  - Validation: Tests prove conflicting route, lease, heartbeat, queue, and restore signals choose observe/manual approval rather than promotion, and all source refs remain visible to authorized operators.

- **3.5 Implement observe and no-action decision paths.**
  - Design: Convert healthy, transient, low-confidence, stale, or conflicting signal sets into no-action, observe, or escalate decisions without issuing commands.
  - Output: Read-only decision records, no-action events, operator summaries, decision supersession rules, and safe tenant/admin redactions.
  - Validation: Tests prove no-action decisions are durable, replayable, superseded by fresher signals when needed, and side-effect free.

## Phase 4: Read-Only Evaluation, Overguard Policy, And Recovery Admission

### Work Items

- **4.1 Implement recovery evaluation APIs.**
  - Design: Add `POST /recovery/evaluate` and `GET /recovery/decisions/{decision_id}` to evaluate a service, route, queue lane, or drill scope and return reason-coded decisions without issuing commands unless `execute=true` is authorized.
  - Output: Evaluation handler, query projection, decision lifecycle, stable errors, and `failover_recovery.decision_created` events.
  - Validation: Tests prove read-only evaluation has no downstream side effects, returns stable reason codes, preserves evidence refs, and rejects unauthorized `execute=true` attempts before plan creation.

- **4.2 Build Overguard recovery policy input bundles.**
  - Design: Assemble workload class, node eligibility, actor/operator authority, active incident, maintenance state, restore requirement, route criticality, service account, data class, and policy version facts for Overguard decisions.
  - Output: Policy fact bundle schema, Overguard adapter, policy decision refs, denial mapping, and replay fixtures.
  - Validation: Tests prove policy-denied actions fail before route, queue, lease, restore, promotion, or writer side effects and include replayable Overguard refs.

- **4.3 Enforce System-Service Workload Class and node eligibility gates.**
  - Design: Permit Phase 7 automated recovery only for protected system-service scopes and trusted eligible nodes; unknown public nodes must never receive backbone hosting or promotion commands.
  - Output: Workload-class validator, node eligibility adapter, eligibility reason codes, and enforcement tests.
  - Validation: Tests reject ordinary workloads, public-node hosts, stale eligibility evidence, revoked node trust, wrong service class, and missing package/class refs before recovery plan readiness.

- **4.4 Implement operator, maintenance, and break-glass admission gates.**
  - Design: Require signed operator approval for stateful writer promotion, restore-backed cutover, founder-hardware removal, broad primary control-plane shifts, break-glass overrides, destructive migration/restore steps, policy exceptions, and conflicting or stale safety evidence.
  - Output: Operator approval contract, maintenance-mode API, break-glass record shape, approval expiry, incident refs, and audit events.
  - Validation: Tests prove missing, stale, wrong-role, wrong-scope, unsigned, or no-incident approvals block risky recovery before side effects.

- **4.5 Map admission outcomes to decision states and evidence.**
  - Design: Move decisions through `observed`, `evaluating`, `no_action`, `plan_required`, `denied`, or `superseded` with reason codes, policy refs, signal refs, and redacted summaries.
  - Output: Decision state machine, evidence bundle builder, supersession logic, denial events, and operator/tenant projection.
  - Validation: Tests prove every terminal state includes reason codes, audit refs, redacted evidence, and no private topology, raw health content, raw secret refs, or incident-sensitive details in user-visible responses.

## Phase 5: Writer Guards, Fencing, Promotion Safety, And Split-Brain Prevention

### Work Items

- **5.1 Implement active-writer guard records.**
  - Design: Track active writer, lease ref, lock epoch, quorum/checkpoint ref, last verification time, guard state, and service scope for every service that can become a writer or route-primary.
  - Output: Writer guard storage abstraction, guard query, guard update state machine, guard evidence refs, and guard events.
  - Validation: Tests reject promotion when guard state is `unknown`, stale, mismatched, revoked, or not tied to current service scope and policy refs.

- **5.2 Implement Overrid-owned fencing preconditions.**
  - Design: Use Overlease active leases, Overkey-signed recovery commands, owner-store monotonic epochs/checkpoints, Overqueue lane drains/freezes, Overmesh route drains/freezes, and Overwatch evidence before allowing cutover.
  - Output: Fencing validator, epoch/checkpoint adapter contracts, route/queue freeze checks, signed command refs, and rollback refs.
  - Validation: Tests prove prior writers must be fenced, expired, revoked, or tombstoned and downstream stores must accept the current epoch/checkpoint before route finalization or writer promotion.

- **5.3 Implement active/passive promotion for stateless and rebuildable surfaces.**
  - Design: Support first for non-critical observability replicas, read-only registry/API replicas, worker processes, queue workers after lane drain, API ingress replicas, and route-shiftable services with no independent write head.
  - Output: Active/passive plan template, eligibility checks, readiness checks, route-shift preconditions, rollback behavior, and evidence refs.
  - Validation: Tests prove stateless/rebuildable promotions cannot be applied to critical write heads and fail safely when readiness, route, policy, or evidence requirements are missing.

- **5.4 Gate critical write-head promotion on replicated-log or checkpoint readiness.**
  - Design: Require quorum-style replicated logs or equivalent checkpoint mechanisms before founder fallback removal for Seal Ledger streams, Overqueue claim/ack/dead-letter state, Overwatch append/checkpoint heads, active Overregistry manifests, active Overguard policy bundles, Overvault grant/lease metadata, and later Overbase system collections that carry backbone state.
  - Output: Critical-write-head readiness checklist, owner-service checkpoint adapter, replicated-state evidence refs, and founder-exit blocker reasons.
  - Validation: Tests prove critical write heads remain in degraded/fallback/approval-required states until replicated evidence, checkpoint freshness, restore readiness, and policy approval pass.

- **5.5 Implement split-brain prevention and recovery blocking.**
  - Design: Detect simultaneous writer refs, conflicting epochs, divergent checkpoints, unverified fences, duplicate route primaries, and duplicated queue worker ownership before destructive action.
  - Output: Split-brain detector, `failover_recovery.split_brain_prevented` event, freeze/fence commands, blocked-state reason codes, and incident handoff refs.
  - Validation: Simulation tests prove split-brain scenarios are blocked before promotion, affected writers are fenced or frozen where allowed, and incident evidence is recorded.

## Phase 6: Recovery Plans, Execution Cursors, Idempotent Commands, And Evidence Spooling

### Work Items

- **6.1 Implement recovery plan template selection.**
  - Design: Select observe, drain, worker failover, route shift, active/passive promotion, restore-backed recovery, maintenance mode, or escalation based on decision facts, policy gates, writer guard state, restore readiness, route criticality, and operator approvals.
  - Output: Plan template engine, precondition builder, rollback-action builder, approval flagging, and `failover_recovery.plan_created` events.
  - Validation: Tests prove selected plans match decision reason codes and block when prerequisites, policy refs, restore refs, writer guards, or approvals are missing.

- **6.2 Implement recovery cursor persistence and plan lifecycle.**
  - Design: Persist plan state, current step, last completed step, blocked step, retry count, resume token, abort boundary, failure reason, and evidence refs across `draft`, `awaiting_approval`, `ready`, `executing`, `verifying`, `blocked`, `maintenance`, `completed`, `failed`, and `aborted`.
  - Output: Cursor model, lifecycle handlers, resume-token logic, plan query projection, and state-transition events.
  - Validation: Tests prove interrupted plans resume from stored cursor state, duplicate execute calls return the same outcome, and superseded plans remain readable with evidence.

- **6.3 Implement idempotent downstream command envelopes.**
  - Design: Send bounded commands to Overmesh, Oversched, Overlease, Overqueue, Backup and Restore Service, Grid-Resident Service Packager command hooks, and owner services using signed envelopes, idempotency keys, trace ids, expected-state checks, timeout policy, and evidence refs.
  - Output: Command schemas, adapter interfaces, retry policy, downstream state pollers, timeout reason codes, and command result fixtures.
  - Validation: Contract tests prove every mutating command includes system scope, policy refs, idempotency key, trace id, schema version, stable reason codes, and expected evidence refs; timeout retries query owner state before resending.

- **6.4 Implement pause, abort, resume, rollback, and blocked behavior.**
  - Design: Allow pausing at safe boundaries, aborting before final promotion or route-shift finality, resuming after fresh safety checks, and blocking on policy, restore, route, writer, approval, dependency, or evidence conditions.
  - Output: `POST /recovery/plans/{plan_id}/pause`, `POST /recovery/plans/{plan_id}/abort`, resume behavior, blocked projection, rollback hooks, and operator reason codes.
  - Validation: Tests prove active commands are not interrupted unsafely, unstarted plans abort cleanly, irreversible handoff requires rollback policy, stale plans require fresh evaluation, and all outcomes preserve evidence.

- **6.5 Implement Overwatch degradation spool and reconciliation.**
  - Design: Maintain a local command/evidence spool when Overwatch is degraded, block destructive progress if evidence cannot be made durable, and reconcile events when Overwatch returns.
  - Output: Evidence spool, Overwatch writer, reconciliation worker, spool health metrics, and degraded-mode reason codes.
  - Validation: Tests prove evidence-write failure leaves plans blocked or retrying, spool recovery preserves order and trace refs, and completed recovery cannot be reported until required evidence is durable or safely reconciled.

## Phase 7: Route Shift, Worker Failover, Replacement Capacity, And Restore-Backed Recovery

### Work Items

- **7.1 Implement health-based route shifting through Overmesh.**
  - Design: Shift or drain routes only through Overmesh route APIs using route refs, previous/new route refs, policy refs, freshness proof, idempotency key, rollback action, and post-shift health checks.
  - Output: Route-shift adapter, route-shift plan step, route-state query, `failover_recovery.route_shifted` event, and rejection reason codes.
  - Validation: Tests prove route shifts are idempotent, preserve current route on rejection, do not reveal protected topology, and block when route evidence, policy refs, writer guard, or rollback refs are missing.

- **7.2 Implement queue worker failover and lane pause/resume.**
  - Design: Drain unhealthy workers, pause/resume affected queue lanes, reassign work where safe, and prevent duplicate execution using Overqueue and Overlease facts.
  - Output: Worker failover template, lane pause/resume commands, lease proof checks, duplicate-execution guards, and queue evidence refs.
  - Validation: Tests prove worker failure drains or reassigns work without duplicate execution, stale lease facts block automation, and failed worker recovery records reason-coded evidence.

- **7.3 Implement replacement capacity requests.**
  - Design: Request replacement capacity through Oversched and Overlease using system-service workload class, node eligibility, package refs, data class, route criticality, resource dimensions, and policy refs.
  - Output: Replacement-capacity adapter, scheduler request schema, lease request schema, eligibility proof refs, and blocked reason codes.
  - Validation: Tests prove replacement requests reject ineligible nodes, missing package/class refs, insufficient capacity, stale eligibility evidence, untrusted public nodes, and duplicate reservation attempts.

- **7.4 Implement restore-backed recovery coordination.**
  - Design: Require Backup and Restore Service restore-plan refs, restore-session evidence, restore integrity proof, maintenance/quiesce state, vault grant refs where needed, and owner-service readiness before promoting a stateful replacement.
  - Output: Restore readiness adapter, restore-backed plan template, restore step execution, restore verification checks, and rollback/abort behavior.
  - Validation: Tests prove stateful service promotion is blocked unless active-writer guard and restore readiness pass, and missing/stale restore refs produce `restore_not_ready` before side effects.

- **7.5 Verify health and readiness after each major recovery step.**
  - Design: Re-check Overwatch, Overmesh, Overqueue, Overlease, restore, writer guard, and service-package readiness after route shifts, worker replacements, promotions, maintenance changes, and restore steps.
  - Output: Verification step runner, health gate model, readiness reason codes, plan completion criteria, and `failover_recovery.plan_completed` or `plan_failed` events.
  - Validation: Tests prove recovery cannot complete without fresh post-step evidence, failed verification blocks or rolls back according to plan policy, and final evidence includes decision, plan, command, route, restore, writer, and health refs.

## Phase 8: Simulation, Drills, Stateful Rehearsal, And Founder-Hardware Exit Gates

### Work Items

- **8.1 Implement replay and simulation drills.**
  - Design: Run recovery logic against recorded signals without issuing route, lease, queue, restore, promotion, or package commands after every recovery-policy, package-command, storage, signer, or class-version change.
  - Output: Simulation runner, replay bundle schema, expected/actual decision report, drill evidence refs, and follow-up refs.
  - Validation: Tests prove simulation is side-effect free, deterministic from recorded inputs, and marks policy/contract differences when inputs or versions change.

- **8.2 Implement non-production and shadow drill cadence.**
  - Design: Schedule weekly shadow drills during Phase 7 buildout and run non-production drills under declared bounds with abort behavior and Overwatch evidence.
  - Output: Drill scheduler, safe target selector, cadence policy, shadow command blocker, and drill status query.
  - Validation: Tests prove shadow drills cannot touch production route/queue/writer state and missed cadence produces operator-visible follow-up work.

- **8.3 Implement non-critical live service drills.**
  - Design: Run one controlled live drill for each service before advancing it to `eligible_for_noncritical`, affecting only one failure domain and carrying rollback/abort bounds.
  - Output: Live drill plan template, non-critical eligibility gate, route/worker drill steps, rollback refs, and findings model.
  - Validation: Tests prove a non-critical grid-resident service recovers without founder hardware in the normal production path and failed drills block eligibility advancement.

- **8.4 Implement stateful backbone restore/failover drill gates.**
  - Design: Require each migrated stateful backbone service to pass a restore/failover drill under its current backup policy within the preceding 30 days before promotion or founder fallback removal.
  - Output: Stateful drill gate, backup-policy freshness check, restore/failover evidence refs, critical-write-head gate, and stale-drill blocker reasons.
  - Validation: Tests prove stale or missing drills block stateful promotion, critical write-head migration, and founder-path removal even when route and health signals are fresh.

- **8.5 Implement final founder-path removal rehearsal.**
  - Design: Require an end-to-end cutover rehearsal within seven days before removing founder hardware from the normal production path, with backup, restore, failover, rollback, writer guard, route, queue, and incident evidence.
  - Output: Founder-exit checklist, rehearsal plan, pass/fail criteria, Overwatch evidence bundle, follow-up refs, and approval requirements.
  - Validation: Review and tests prove founder hardware remains emergency fallback until restore, failover, rollback, and rehearsal evidence passes under current policies.

## Phase 9: Operations, Interfaces, Metering, And Downstream Handoffs

### Work Items

- **9.1 Implement service status and recovery timeline APIs.**
  - Design: Expose `GET /recovery/services/{service_id}/status`, decision reads, plan reads, drill reads, and timeline projections with current health, active recovery, route state, writer guard state, maintenance state, and last drill summary.
  - Output: Status projection, timeline projection, filters, pagination, redaction profiles, and operator/tenant/client view models.
  - Validation: Tests prove timeline entries are ordered, trace-linked, redacted by data class, and include refs rather than private content, protected topology, raw secret refs, or incident-sensitive details in tenant views.

- **9.2 Implement CLI, SDK, and Admin UI command surfaces.**
  - Design: Add generated client operations for signal ingest where allowed, evaluate, read decisions, execute approved plans, pause, abort, schedule drills, read drill status, maintenance mode, and status/timeline reads.
  - Output: CLI command contracts, SDK bindings, Admin UI view contracts, stable JSON output, error examples, idempotency behavior, and trace propagation.
  - Validation: Contract tests prove client commands pass signed envelopes, trace ids, idempotency keys, schema versions, policy refs, stable reason codes, and redaction rules through generated contracts.

- **9.3 Emit recovery usage and stewardship cost facts.**
  - Design: Emit system-service usage for recovery drills, replacement workloads, restore sessions, route churn, and extended incident operations through Overmeter and accounting hooks without charging per recovery action or mutating balances directly.
  - Output: Usage event schema, Overmeter handoff refs, ORU/Overbill/Seal Ledger refs where available, resource dimensions, and public-reporting classification.
  - Validation: Tests prove usage facts distinguish normal user workload usage from system-service reliability overhead and never create pricing, revenue, balance, or ledger mutations directly.

- **9.4 Feed incident, stewardship, deployment, and release owners.**
  - Design: Provide recovery timelines, decision refs, command refs, restore refs, route refs, drill reports, and follow-up refs to Overwatch, Incident Response Service, Stewardship Reporting Service, Deployment Planner, Release Strategy Service, and operator tooling.
  - Output: Owner-service event contracts, handoff APIs, incident evidence bundle, report refs, deployment/release handoff refs, and follow-up backlog entries.
  - Validation: Integration tests prove owner services consume refs through APIs and events, not coordinator-owned private records, and can reject or block recovery progress with explicit reason codes.

- **9.5 Harden redaction, audit, and operator diagnostics.**
  - Design: Redact provider topology, internal route details, private health content, incident-sensitive evidence, raw secret refs, and cross-tenant facts from user-visible responses while giving authorized operators signed diagnostic views.
  - Output: Redaction profile, role-gated operator diagnostics, audit export schema, break-glass view policy, and security review checklist.
  - Validation: Tests prove unauthorized views cannot access protected topology, health internals, private evidence, raw secrets, or cross-tenant data, and every operator view emits Overwatch audit refs.

## Phase 10: Validation, Security Review, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, coordinator-owned-backups, coordinator-owned-release-strategy, coordinator-owned-package-validation, coordinator-owned-storage, direct-ledger-mutation, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control, native Overrid service-name, authority-boundary, or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools, local stubs, or downstream owner services into the coordinator's product boundary.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated backlinks and index rows for SDS #47.
  - Validation: Local link checks pass and Docdex search returns the SDS, service plan, crosswalk row, and this sub-build plan in aligned results.

- **10.4 Validate queue state and progress evidence.**
  - Design: Mark `047-build-plan` complete in the Codex55 queue, update `.codex55_sds_queue/progress.md`, append validation evidence to `docs/build_plan/progress.md`, and preserve the next incomplete build-plan task as SDS #48.
  - Output: Updated queue JSON, queue progress summary, and build-plan progress notes.
  - Validation: JSON validation passes; queue validation confirms `047-build-plan` is complete, no task is running, and `048-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders contract, signal, evaluation, policy, fencing, execution, route/worker/restore, drill, interface, and validation work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.
