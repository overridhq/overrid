SDS #47

# Failover and Recovery Coordinator SDS

## Purpose

Coordinate safe recovery of grid-resident backbone services during node failure, service degradation, route failure, queue backlog, partial outage, and disaster-recovery drills.

Failover and Recovery Coordinator is the live reliability control point for protected system workloads. It consumes health, route, lease, queue, backup, and service-package evidence; decides whether recovery is needed; issues bounded failover commands through the proper services; and records recovery evidence. It does not store backups, execute releases, validate packages, replace Overwatch as health truth, or become a general-purpose orchestrator.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [failover_recovery_coordinator.md](../../service_catalog/deployment_grid/failover_recovery_coordinator.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md) |

## Service Family

- Family: Deployment and grid-resident backbone
- Owning layer: System-service reliability and recovery coordination
- Primary data scope: health signal snapshots, failover decisions, recovery plans, route-shift commands, active/passive locks, maintenance states, recovery drills, split-brain guards, and incident evidence refs
- First build phase from service plan: [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md)

## Problem Statement

The Overrid backbone must eventually run inside the grid instead of depending on founder machines. Once control-plane services become grid-resident, failures cannot be handled by manual server intervention or hidden centralized operations. Recovery must be explicit, policy-bound, auditable, and safe against double writers, split-brain state, stale health signals, and destructive restore mistakes.

This service turns health and topology evidence into controlled recovery actions. It must be simple enough to operate early, but strict enough to protect Overgate, Overregistry, Overqueue, Oversched, Overmeter, Overwatch, Overguard, Overpass, supporting stores, and internal observability.

## Goals

- Detect and classify backbone degradation from Overwatch health events, route health, queue backlog, lease failures, and dependency readiness.
- Decide between no action, observe, drain, route shift, worker failover, active/passive promotion, restore coordination, maintenance mode, or incident escalation.
- Prevent double writers and split-brain behavior before promoting a replacement service instance.
- Coordinate with Overmesh for route shifts and with Oversched/Overlease for replacement capacity.
- Use Backup and Restore Service restore plans when a healthy replacement needs recovered state.
- Record drills and real recoveries as replayable evidence.
- Keep founder hardware as fallback only until Phase 7 failover and restore drills pass.

## Non-Goals

- Do not store snapshot data or implement backup engines; Backup and Restore Service owns backup manifests, restore sessions, and integrity reports.
- Do not deploy new versions or choose rollout policy; Release Strategy Service owns release strategy and update/rollback intent.
- Do not validate service packages; Package Validator and Grid-Resident Service Packager own package contracts and validation evidence.
- Do not replace Overwatch health/event storage or incident response workflows.
- Do not bypass Overguard, Overmesh, Overlease, Overqueue, Overwatch, or system-service workload-class policy.
- Do not create autonomous destructive recovery actions without policy, lock, and evidence gates.
- Do not add pricing, revenue, customer-count, blockchain, NFT, or per-transaction-fee assumptions.

## Primary Actors And Clients

- Overwatch sending health, trace, incident, and anomaly events.
- Overmesh reporting endpoint health and accepting route-shift commands.
- Overqueue and Oversched reporting queue backlog, worker failures, and lease placement failures.
- Overlease reporting expired, revoked, or failed leases for system-service workloads.
- Backup and Restore Service providing restore plan refs and restore-session evidence.
- Operators initiating drills, maintenance mode, break-glass actions, and incident-linked recovery.
- System-Service Workload Class policy and Overguard deciding whether a node or service instance may host backbone work.
- Incident Response Service consuming recovery timelines in later hardening phases.

## Dependencies

- [System-Service Workload Class](system_service_workload_class.md) for protected workload eligibility, placement, backup, upgrade, and logging rules.
- [Overwatch](../control_plane/overwatch.md) for health signals, traces, audit records, and incident evidence.
- [Overmesh](../execution_scheduling/overmesh.md) for endpoint health, service discovery, route weights, and route-shift execution.
- [Oversched](../execution_scheduling/oversched.md) and [Overlease](../execution_scheduling/overlease.md) for replacement placement and active lease facts.
- [Overqueue](../control_plane/overqueue.md) for durable queue state, worker backlog, dead-letter signals, and replayable command handoff.
- [Backup and Restore Service](backup_restore_service.md) for restore plans, restore sessions, snapshot integrity, and disaster-recovery drill evidence.
- [Overguard](../trust_policy_verification/overguard.md) for recovery authorization, node eligibility, workload-class rules, and operator action policy.
- [Grid-Resident Service Packager](grid_resident_service_packager.md) for service package health, readiness, restore, and rollback command contracts.

## Owned Responsibilities

Failover and Recovery Coordinator owns:

- Health signal aggregation snapshots for system-service workloads.
- Recovery policy inputs and failover decision records.
- Recovery plan templates for route shift, worker replacement, active/passive promotion, restore coordination, drain, maintenance, and escalation.
- Split-brain guard checks, active-writer lock refs, quorum or fencing refs, and promotion preconditions.
- Route-shift command envelopes sent to Overmesh.
- Replacement capacity requests sent to Oversched and Overlease.
- Recovery cursor state for multi-step failover operations.
- Drill scheduling, drill execution records, and follow-up work refs.
- Evidence bundles that link health, decision, command, restore, route, and incident records.

## Data Model

- `health_signal_snapshot`: normalized view of health facts with `snapshot_id`, `service_id`, `instance_id`, `node_id`, `signal_type`, `severity`, `source_ref`, `observed_at`, `freshness_window`, `confidence`, and `data_class`.
- `recovery_policy_input`: facts supplied to Overguard, including workload class, node eligibility, active incident, maintenance state, restore requirement, route criticality, and operator role.
- `failover_decision`: durable decision with `decision_id`, `service_id`, `scope`, `trigger_refs`, `decision_type`, `selected_action`, `reason_codes`, `policy_decision_refs`, `operator_ref`, and `state`.
- `recovery_plan`: planned sequence with `plan_id`, `decision_id`, `plan_type`, `preconditions`, `steps`, `route_shift_spec`, `replacement_lease_ref`, `restore_plan_ref`, `lock_refs`, `rollback_action`, and `approval_required`.
- `recovery_step`: executable action such as `drain_instance`, `fence_writer`, `request_replacement`, `restore_state`, `promote_instance`, `shift_route`, `pause_queue_lane`, `resume_queue_lane`, or `escalate_incident`.
- `active_writer_guard`: current writer/fencing state with `guard_id`, `service_id`, `leader_ref`, `lease_ref`, `lock_epoch`, `quorum_ref`, `last_verified_at`, and `state`.
- `recovery_drill`: scheduled or completed drill with `drill_id`, `service_scope`, `scenario`, `expected_outcome`, `started_at`, `completed_at`, `evidence_refs`, `findings`, and `follow_up_refs`.

Every recovery record uses trace id, idempotency key where commands are mutating, tenant or system scope, service account identity, policy refs, and Overwatch audit refs.

## API Surface

- `POST /recovery/signals/ingest`: internal endpoint for bounded health-signal ingestion when event delivery needs synchronous acknowledgement.
- `POST /recovery/evaluate`: evaluates a service or route scope and returns a failover decision without issuing commands unless `execute=true` is authorized.
- `GET /recovery/decisions/{decision_id}`: returns decision facts, reason codes, policy refs, and redacted evidence.
- `POST /recovery/plans/{plan_id}/execute`: starts or resumes an approved recovery plan.
- `POST /recovery/plans/{plan_id}/pause`: pauses at the next safe boundary.
- `POST /recovery/plans/{plan_id}/abort`: aborts a plan that has not reached promotion or route-shift finality.
- `POST /recovery/drills`: schedules or starts a controlled drill with target service, scenario, safety bounds, and rollback behavior.
- `GET /recovery/drills/{drill_id}`: returns drill state, evidence, and follow-up items.
- `GET /recovery/services/{service_id}/status`: returns current health, active recovery, route state, writer guard state, and last drill summary.
- `POST /recovery/maintenance-mode`: enters or exits maintenance mode for a service scope with signed operator approval.

Mutating endpoints require system-service identity, operator or automation authorization, trace id, idempotency key, and Overguard policy decision refs. Stable errors include `stale_health_signal`, `policy_denied`, `writer_guard_not_verified`, `replacement_not_eligible`, `restore_not_ready`, `route_shift_rejected`, `split_brain_risk`, and `operator_approval_required`.

## Event Surface

- `failover_recovery.signal_ingested`: health or route signal accepted into an aggregation snapshot.
- `failover_recovery.decision_created`: recovery decision recorded with reason codes.
- `failover_recovery.no_action_selected`: evaluator chose observation or no-op with evidence.
- `failover_recovery.plan_created`: recovery plan built from a decision.
- `failover_recovery.plan_started`: recovery execution began.
- `failover_recovery.step_started`: recovery step command issued.
- `failover_recovery.step_completed`: recovery step succeeded.
- `failover_recovery.step_blocked`: plan cannot continue without dependency, lock, approval, restore, or route condition.
- `failover_recovery.split_brain_prevented`: a promotion or route shift was blocked by writer/lock safety checks.
- `failover_recovery.route_shifted`: route changed through Overmesh with previous and new route refs.
- `failover_recovery.drill_completed`: drill finished with evidence and follow-up refs.
- `failover_recovery.plan_completed`: recovery reached terminal success.
- `failover_recovery.plan_failed`: recovery reached terminal failure and incident evidence was recorded.

Payloads must carry system scope, affected service, affected instance/node when safe, trace id, decision/plan ids, policy refs, command refs, and redacted evidence refs.

## Core Workflow

1. Aggregate fresh health, route, queue, lease, node, and backup-readiness signals.
2. Normalize signals by service, instance, route, node, workload class, and criticality.
3. Evaluate whether the condition is transient, degraded, failed, unsafe, or drill-driven.
4. Ask Overguard whether the proposed recovery action is allowed for the service scope and actor.
5. Check active-writer guard and split-brain preconditions.
6. Select recovery plan type: observe, drain, worker failover, route shift, active/passive promotion, restore-backed recovery, maintenance mode, or escalation.
7. Execute recovery steps through Overmesh, Oversched, Overlease, Overqueue, Backup and Restore Service, and service-package commands.
8. Verify health/readiness after each major step.
9. Record final evidence and follow-up work.

## State Machine

Decision lifecycle:

1. `observed`: signals exist but no decision has been made.
2. `evaluating`: signals, policy, freshness, and safety checks are being computed.
3. `no_action`: no recovery is needed; evidence is retained.
4. `plan_required`: action is needed and a plan must be created or selected.
5. `denied`: policy or safety checks reject action.
6. `superseded`: fresher signals or operator action replaced this decision.

Recovery plan lifecycle:

1. `draft`
2. `awaiting_approval`
3. `ready`
4. `executing`
5. `verifying`
6. `blocked`
7. `maintenance`
8. `completed`
9. `failed`
10. `aborted`

Writer guard lifecycle:

1. `unknown`
2. `single_writer_verified`
3. `fencing_requested`
4. `fenced`
5. `promotion_allowed`
6. `promotion_blocked`
7. `split_brain_risk`

State transitions are correction-based and auditable. A recovery plan may be superseded by a fresher plan, but the previous plan and its evidence remain readable.

## Policy And Security

- Only system-service workload scopes may be managed by this coordinator in Phase 7.
- Unknown public nodes must never host backbone services or receive promotion commands.
- Require Overguard decisions for recovery action, node eligibility, operator authority, break-glass action, and maintenance-mode entry/exit.
- Require active-writer guard verification before promotion, route finalization, or queue writer replacement.
- Require restore integrity evidence before restore-backed promotion.
- Use least-privilege service accounts for route, queue, lease, restore, and package commands.
- Redact provider topology, internal route details, private health content, and incident-sensitive evidence from user-visible responses.
- Record all operator overrides with signed action, reason code, expiry, and incident ref.

## Metering And Accounting

- Emit system-service resource usage for recovery drills, replacement workloads, restore sessions, route churn, and extended incident operations.
- Link recovery usage to system-service account, affected service, workload class, resource dimensions, and incident/drill refs.
- Use ORU and Seal Ledger only through accounting service hooks; this coordinator does not write balances or ledger entries directly.
- Do not charge per recovery action as a speculative transaction model; record resource use structurally for transparency and near-cost system operation.
- Distinguish normal user workload usage from system-service reliability overhead so public reports can show stewardship cost without exposing sensitive topology.

## Observability And Operations

- Expose coordinator health, signal freshness, event-ingestion lag, blocked plan count, active recoveries, drill schedule, and writer guard state.
- Track mean time to detect, decide, recover, and verify per system-service family.
- Provide operator views for signal snapshots, decision reason codes, plan steps, route changes, lock refs, restore refs, and final evidence.
- Support simulation mode for recovery logic using recorded signals without issuing route or promotion commands.
- Keep recovery playbooks versioned and link every decision to the playbook version used.
- Maintain a local command spool for recovery events when Overwatch is degraded, then reconcile when Overwatch returns.

## Failure Modes And Recovery

- Stale health signals: refuse destructive failover and request fresh checks.
- Conflicting signals: choose observation or manual approval rather than promotion.
- Writer guard unknown: block promotion and escalate to operator/incident response.
- Replacement node not eligible: request a new candidate or keep service in degraded mode.
- Route shift rejected: preserve current route, record reason, and fall back to maintenance or escalation.
- Restore plan unavailable: do not promote a stateful replacement; request Backup and Restore Service action.
- Overwatch degraded: use local spool and mark plan `blocked` if evidence cannot be made durable.
- Recovery command duplicated: rely on idempotency keys and downstream current-state checks.
- Split-brain detected: fence affected writers, freeze route promotion, and escalate to incident response.
- Drill failure: stop before production impact, record findings, and create follow-up work refs.

## Validation Plan

- Controlled node failure triggers the expected route shift or replacement plan.
- Queue worker failure drains or reassigns work without duplicate execution.
- Stateful service promotion is blocked unless active-writer guard and restore readiness pass.
- Split-brain scenario is detected and prevented in simulation.
- Route shift commands are idempotent and auditable through Overmesh and Overwatch.
- Recovery drill records evidence, timing, findings, and follow-up refs.
- Founder hardware can be removed from the normal production path only after backup, restore, failover, and rollback drills pass.
- Policy-denied recovery actions fail before route or writer side effects.
- Stale health signals cannot trigger destructive failover.

## Build Breakdown

1. Define health signal, failover decision, recovery plan, recovery step, writer guard, and drill schemas.
2. Implement signal aggregation from Overwatch, Overmesh, Overqueue, Overlease, and scheduler facts.
3. Implement read-only recovery evaluation with reason codes and Overguard policy checks.
4. Add route-shift coordination with Overmesh and idempotent command refs.
5. Add worker failover and queue-lane pause/resume integration.
6. Add active/passive promotion guarded by writer/lock safety checks.
7. Add Backup and Restore Service restore-plan integration for stateful services.
8. Add recovery drill runner, evidence bundle creation, and follow-up refs.
9. Prove a non-critical grid-resident service recovers without founder hardware in the normal production path.

## Handoff And Downstream Use

Failover and Recovery Coordinator feeds Overwatch, incident response, stewardship reporting, Deployment Planner, Release Strategy Service, and operator tooling with reliable recovery timelines. It should be called through recovery APIs and events, not by directly editing route, queue, or lease state.

Later governance and compliance hardening can use recovery evidence to prove that the backbone is protected, auditable, and no longer dependent on founder hardware for normal operation.

## Open Design Questions

- Phase 7 starts with an Overrid-owned single-writer fencing contract rather than an external lock service: Overlease issues the active lease, Overkey signs the recovery command, the owning store records a monotonic writer epoch or checkpoint fence, Overqueue lanes and Overmesh routes are drained or frozen before cutover, and Overwatch records the policy, lease, checkpoint, and command evidence. Promotion is allowed only when the prior writer is fenced, expired, revoked, or tombstoned, the downstream store accepts the current epoch/checkpoint, and the recovery plan has idempotency and rollback refs.
- Active/passive failover is acceptable first for stateless or rebuildable surfaces: non-critical observability replicas, read-only registry or API replicas, worker processes, queue workers after lane drain, API ingress replicas, and other route-shiftable services with no independent write head. Quorum-based replication or an equivalent replicated-log/checkpoint mechanism is required before founder fallback is removed for active control-plane write heads: Seal Ledger streams, Overqueue durable claim/ack/dead-letter state, Overwatch append/checkpoint heads, active Overregistry manifests, active Overguard policy bundles, Overvault grant/lease metadata, and later Overbase system collections that carry backbone state.
- Signal freshness is classed by System-Service Workload Class and can only be tightened by policy. `eligible_for_test` route shifts may use health, route, and lease evidence up to 120 seconds old because they do not carry production primary traffic. `eligible_for_noncritical` route shifts require health, endpoint, route, lease, and replacement-readiness facts no older than 60 seconds. `eligible_for_control_plane` route shifts require service health, route state, queue/lease facts, and writer-guard evidence no older than 30 seconds, plus policy/class/package refs no older than five minutes. `active` primary-path shifts require direct heartbeat or endpoint evidence no older than 15 seconds and writer/quorum/fencing evidence no older than 30 seconds; stale evidence downgrades automation to observe, drain, or escalate.
- During seed hardware operation, automation may ingest signals, evaluate decisions, create plans, request replacement capacity, drain unhealthy replicas, pause or resume non-critical queue lanes, retry idempotent worker commands, shift traffic away from clearly failed non-critical replicas, and run simulation or shadow drills when Overguard allows the action and Overwatch evidence is durable or safely spooled. Human or signed operator approval is required for stateful writer promotion, restore-backed cutover, founder-hardware removal, broad primary control-plane route shifts, break-glass overrides, destructive migration/restore steps, policy exceptions, and any case with conflicting or stale safety evidence.
- Recovery drills use a progressive schedule: replay/simulation drills run after every recovery-policy, package-command, storage, signer, or class-version change; non-production or shadow drills run at least weekly during Phase 7 buildout; one non-critical live service drill runs before advancing each service to `eligible_for_noncritical`; each migrated stateful backbone service needs a successful restore/failover drill under its current backup policy within the preceding 30 days; and final founder-path removal needs an end-to-end cutover rehearsal within seven days. Production-impacting drills must run in declared maintenance windows, affect only one failure domain or stateful control-plane family at a time, carry abort/rollback bounds, and record findings and follow-up refs in Overwatch.
