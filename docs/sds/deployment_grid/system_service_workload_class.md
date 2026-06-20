SDS #51

# System-Service Workload Class SDS

## Purpose

Define the privileged but tightly constrained workload class that allows Overrid backbone services to run on trusted grid nodes.

System-Service Workload Class is a policy and eligibility contract, not a scheduler, package builder, runner, or release system. It defines which services can be treated as backbone services, which nodes may host them, which operational controls are mandatory, and which evidence is required before placement, update, rollback, failover, or break-glass action is allowed.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [system_service_workload_class.md](../../service_catalog/deployment_grid/system_service_workload_class.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Sub-build plan | [SUB BUILD PLAN #51 - System-Service Workload Class](../../build_plan/sub_build_plan_051_system_service_workload_class.md) |
| Build phase alignment | [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md) |

## Service Family

- Family: Deployment and grid-resident backbone
- Owning layer: Backbone workload classification, system-service placement eligibility, and operational guardrails
- Primary data scope: system-service class definitions, eligible service records, node eligibility requirements, placement guardrails, operational requirements, signed operator action requirements, evaluation snapshots, and override records
- First build phase from service plan: [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md)

## Problem Statement

The founder servers and GPUs can bootstrap Overrid, but the backbone must eventually run inside the grid. That migration is unsafe if system services are treated like ordinary workloads. Overgate, Overregistry, Overqueue, Oversched, Overmeter, Overwatch, Overguard, Overpass, supporting stores, and internal observability need stricter placement, logging, backup, update, rollback, failover, and operator-action rules.

Without an explicit system-service workload class, unknown public nodes could accidentally receive backbone services, schedulers would not know which evidence to require, and release or recovery systems could bypass the proof needed to remove founder hardware from the normal production path.

## Goals

- Define the canonical system-service workload class and its versioned eligibility rules.
- Mark which services may be treated as system services and which package/version refs are eligible.
- Define node eligibility requirements for verified operator identity, uptime history, stable network, storage/backup capability, security baseline, dispute status, and explicit system-service approval.
- Publish placement guardrails that Oversched and Overguard can evaluate before any system-service placement.
- Require stricter logging, audit, backup, restore, update, rollback, failover, and break-glass controls for backbone workloads.
- Make system-service placement replayable from trusted evidence.
- Keep unknown public nodes permanently ineligible for system-service workloads.

## Non-Goals

- Do not perform scheduling, leases, deployment, package validation, failover, release rollout, backup, or restore.
- Do not replace Overguard policy decisions or Oververify trust evidence.
- Do not maintain mutable trust scores or provider reputation; this service consumes eligibility evidence and publishes class facts.
- Do not directly enroll nodes or providers; Overcell, Oververify, and onboarding services own those flows.
- Do not create a generic privileged workload escape hatch for apps or native services.
- Do not add pricing, revenue forecasts, customer counts, blockchain mechanics, NFT mechanics, or per-transaction fee economics.

## Primary Actors And Clients

- Overguard evaluating workload-class and placement policy.
- Oververify publishing trusted node/operator eligibility evidence.
- Oversched filtering candidate nodes for system-service workloads.
- Grid-Resident Service Packager attaching system-service package contracts.
- Deployment Planner and Release Strategy Service checking class requirements before deployment or rollout.
- Backup and Restore Service and Failover and Recovery Coordinator checking required recovery controls.
- Operators proposing signed maintenance, rollback, or break-glass actions.
- Overwatch recording placement, override, update, and recovery evidence.

## Dependencies

- [Overguard](../trust_policy_verification/overguard.md) for policy bundles, workload-class admission, deny-by-default decisions, and override handling.
- [Oververify](../trust_policy_verification/oververify.md) for verified operator identity, node evidence, security baseline, dispute status, and eligibility signals.
- [Oversched](../execution_scheduling/oversched.md) for placement candidate evaluation and reason-code publication.
- [Overlease](../execution_scheduling/overlease.md) for short-lived execution eligibility once placement is allowed.
- [Grid-Resident Service Packager](grid_resident_service_packager.md) for package command contracts and privilege profiles.
- [Backup and Restore Service](backup_restore_service.md), [Failover and Recovery Coordinator](failover_recovery_coordinator.md), and [Release Strategy Service](release_strategy_service.md) for operational controls required by the class.
- [Overwatch](../control_plane/overwatch.md) for placement audit, operator action evidence, and replayable traces.

## Owned Responsibilities

System-Service Workload Class owns:

- Versioned system-service class definitions and class compatibility windows.
- Eligible service records for core Overrid services and supporting stores.
- Node eligibility requirement records and minimum evidence refs.
- Placement guardrails consumed by Overguard and Oversched.
- Operational requirement bundles for backup, restore, health, readiness, update, rollback, failover, logging, audit, and incident hooks.
- Signed operator action requirement definitions for maintenance, rollback, migration, break-glass, and founder-hardware fallback.
- Eligibility evaluation snapshots that show why a service/package/node tuple was allowed or denied.
- Class-level override records with expiry, policy refs, operator refs, and Overwatch evidence.

## Data Model

- `system_service_class_definition`: `class_id`, `version`, `description`, `allowed_service_scopes`, `allowed_package_profiles`, `required_controls`, `forbidden_provider_classes`, `compatibility_window`, `state`, and `policy_refs`.
- `eligible_service_record`: `service_id`, `service_name`, `category`, `criticality`, `state_store_refs`, `package_requirements_ref`, `backup_requirement_ref`, `failover_requirement_ref`, `release_requirement_ref`, and `current_class_version`.
- `node_eligibility_requirement`: required evidence for `operator_identity`, `node_identity`, `uptime_history`, `network_baseline`, `storage_backup_capability`, `security_baseline`, `jurisdiction_or_region`, `dispute_status`, and `system_approval`.
- `placement_guardrail`: `guardrail_id`, `class_version`, `candidate_filter`, `required_policy_facts`, `hard_denials`, `soft_warnings`, `reason_codes`, and `overguard_policy_ref`.
- `operational_requirement_bundle`: required health, readiness, audit, backup, restore, rollback, failover, maintenance, logging, metrics, and incident hooks.
- `operator_action_requirement`: action type, required signer role, second-approval requirement, expiry, incident ref requirement, and break-glass disclosure behavior.
- `eligibility_evaluation_snapshot`: service/package/node evaluation with input refs, class version, policy decision refs, result, reason codes, warnings, and audit refs.

Accepted class definitions are immutable by version. Changes create a superseding class version and must preserve replay of old placement decisions.

## API Surface

- `POST /system-workload-classes`: proposes a new class version or draft update for operator review.
- `GET /system-workload-classes/{class_id}/versions/{version}`: returns the class definition, required controls, guardrails, and compatibility notes.
- `POST /system-services/{service_id}/eligibility`: proposes or updates an eligible service record.
- `GET /system-services/{service_id}/eligibility`: returns service eligibility, current class version, required controls, and blocking gaps.
- `POST /system-workload-classes/evaluate-placement`: evaluates service, package, node, operator, and policy refs without scheduling side effects.
- `POST /system-workload-classes/operator-actions/evaluate`: checks whether a maintenance, rollback, migration, or break-glass action has required signatures and policy refs.
- `GET /system-workload-classes/evaluations/{evaluation_id}`: returns replayable evaluation facts and reason codes.
- `POST /system-workload-classes/overrides`: records a signed, expiring class override after Overguard approval.

Mutating APIs require system operator identity, tenant/system scope, trace id, idempotency key, Overguard policy refs, and Overwatch audit refs. Stable errors include `unknown_public_node_denied`, `operator_not_verified`, `node_not_system_eligible`, `missing_backup_control`, `missing_failover_control`, `package_not_system_service`, `class_version_incompatible`, and `operator_action_not_signed`.

## Event Surface

- `system_workload_class.version_proposed`: new class version was proposed.
- `system_workload_class.version_approved`: class version became active.
- `system_workload_class.service_marked_eligible`: service was added or updated as system-service eligible.
- `system_workload_class.service_blocked`: service eligibility is blocked by missing package, backup, failover, or policy controls.
- `system_workload_class.placement_evaluated`: placement evaluation completed with allow/deny reason codes.
- `system_workload_class.placement_denied`: hard denial blocked system-service placement.
- `system_workload_class.operator_action_evaluated`: maintenance, rollback, migration, or break-glass action was evaluated.
- `system_workload_class.override_recorded`: signed expiring override was recorded.
- `system_workload_class.version_retired`: old class version is no longer valid for new placements.

Events include class version, service id, package ref where applicable, node/provider refs when policy allows, reason codes, policy refs, and redacted evidence refs.

## Core Workflow

1. Define class version with service scope, provider restrictions, placement guardrails, and required operational controls.
2. Mark one non-critical backbone service as eligible only after package, backup, restore, failover, update, and audit requirements are present.
3. Consume Oververify node/operator evidence and publish node eligibility requirements to Overguard and Oversched.
4. Evaluate candidate placements by service, package, node, class version, policy facts, and operational controls.
5. Deny unknown public nodes, active-dispute providers, missing backup/restore evidence, missing failover controls, or unsigned operator actions.
6. Record placement evaluation snapshots and Overwatch events for replay.
7. Expand service eligibility through the Phase 7 migration sequence as drills pass.

## State Machine

Class version lifecycle:

1. `draft`
2. `under_review`
3. `approved`
4. `active`
5. `superseded`
6. `retired`
7. `revoked`

Service eligibility lifecycle:

1. `proposed`
2. `blocked`
3. `eligible_for_test`
4. `eligible_for_noncritical`
5. `eligible_for_control_plane`
6. `active`
7. `suspended`
8. `retired`

Placement evaluation lifecycle:

1. `submitted`
2. `facts_loaded`
3. `policy_checked`
4. `allowed`
5. `denied`
6. `requires_operator_action`
7. `expired`

No historical placement evaluation is rewritten. Corrected evidence creates a new evaluation snapshot.

## Policy And Security

- Unknown public nodes, public sandbox nodes, and low-sensitivity public providers are hard-denied for system-service workloads.
- Require verified operator identity, node identity, security baseline, uptime evidence, backup capability, and no active dispute/abuse marker before eligibility.
- Require Overguard decisions for placement, service eligibility, class version approval, override, maintenance, rollback, and break-glass action.
- Require service package command contracts for health, readiness, backup, restore, rollback, drain, and diagnostics before eligibility can advance.
- Store only secret refs and required secret classes; raw secrets stay in Overvault.
- Require signed operator actions and audit refs for any state that affects backbone services.
- Publish redacted placement explanations for operators without leaking private topology or sensitive provider evidence.

## Metering And Accounting

- Emit system-service eligibility evaluation, policy evaluation, and operator-action usage events where material.
- Tag system-service resource usage with service id, class version, workload class, node eligibility, and deployment/release/failover refs.
- Keep backbone operating cost visible to Overmeter and stewardship reporting without per-action fees or speculative economics.
- Use ORU and Seal Ledger only through accounting rails when system-service work consumes resources.
- Do not store balances, billing documents, payout state, or ledger entries in this service.

## Observability And Operations

- Expose active class version, pending class updates, eligible services, blocked services, node eligibility blockers, override records, and stale evidence counts.
- Track placement denials by reason code and service id.
- Provide operator diagnostics for why a service/package/node tuple was allowed or denied.
- Support simulation mode against recorded node evidence and policy bundles.
- Alert when class versions are near expiry, required controls are missing, or old eligible services lack current package/backup/failover contracts.
- Provide audit views for maintenance, rollback, and break-glass action requirements.

## Failure Modes And Recovery

- Missing Oververify evidence: deny placement and return `node_evidence_missing`.
- Unknown public node: deny before scheduling and emit hard-denial evidence.
- Missing package command contract: keep service eligibility blocked.
- Missing backup or restore drill evidence: block stateful control-plane migration.
- Policy bundle unavailable: deny or keep evaluation blocked; never guess.
- Operator action lacks required signature: deny action and retain evidence.
- Class version revoked after placement: notify Deployment Planner, Release Strategy Service, Failover and Recovery Coordinator, and Oversched to evaluate suspension or migration.
- Conflicting eligibility evidence: choose denial or manual review rather than allowing placement.

## Validation Plan

- Unknown public nodes cannot host system-service workloads.
- A system-service placement can be replayed from class version, service eligibility, node evidence, policy refs, and package refs.
- Missing backup, restore, failover, or rollback contracts block stateful service eligibility.
- Maintenance, rollback, migration, and break-glass actions require signed operator actions and Overwatch evidence.
- Oversched candidate filtering uses class guardrails and returns stable denial reason codes.
- Revoking a class version prevents new placements without mutating historical placement evidence.
- Founder hardware is removed from the normal production path only after class, backup, failover, and release criteria are satisfied.

## Build Breakdown

1. Define class definition, eligible service, node requirement, placement guardrail, operator action, and evaluation schemas.
2. Implement read-only class version and service eligibility APIs.
3. Add placement evaluation with Overguard and Oververify facts.
4. Add package, backup, restore, rollback, failover, logging, and audit control checks.
5. Add signed operator action evaluation for maintenance, rollback, migration, and break-glass.
6. Add class version approval, supersession, retirement, and revocation.
7. Integrate with Oversched candidate filtering and Deployment Planner preflight.
8. Prove a non-critical service can be classified, placed, audited, rolled back, and later expanded through the Phase 7 migration sequence.

## Handoff And Downstream Use

System-Service Workload Class hands placement rules and eligibility evidence to Overguard, Oversched, Deployment Planner, Release Strategy Service, Grid-Resident Service Packager, Backup and Restore Service, Failover and Recovery Coordinator, Overwatch, and operator tooling.

Downstream services must call evaluation APIs and preserve evaluation refs instead of copying class logic into local checks.

## Open Design Questions

- Resolved: first private-grid system-service eligibility is an evidence checklist, not a score. A node can enter only `eligible_for_test` after Overkey-signed operator identity, Overregistry/Overcell node enrollment, current Overcell heartbeat and command-acceptance refs, Hardware Discovery inventory, Benchmark Runner capacity refs, Challenge Task Service liveness/resource/sandbox/result-consistency passes, Oververify `system_service` eligibility signal, Overguard class-policy allow decision, security-baseline and software-version refs, stable network and storage/backup-capability evidence, no active high-severity Overclaim dispute or abuse marker, accepted system-service class version, package/profile refs, and durable Overwatch audit refs. Unknown public nodes, public sandbox nodes, public low-sensitivity providers, and nodes with stale or missing critical evidence remain hard-denied; stronger states require service-specific package, backup, restore, failover, and release evidence.
- Resolved: the first service to advance from `eligible_for_test` to `eligible_for_noncritical` is a non-critical Overwatch/internal-observability replica that does not own the primary append/checkpoint head, does not carry tenant-private data or raw secrets, and can be rebuilt or drained without user-facing outage. This matches the Phase 7 migration sequence and the Grid-Resident Service Packager decision to prove a low-risk observability replica before read-only registry/API replicas, workers, queue workers, policy, metering, ingress, or primary control-plane paths. Promotion requires a validated system-service package, health/readiness/diagnostics commands, explicit no-primary-state or restore semantics, current class evaluation, noncritical release plan, rollback path, and a successful drill recorded in Overwatch.
- Resolved: freshness windows are classed by eligibility state and are reevaluated before every placement, release promotion, failover, rollback, and founder-hardware migration step. `eligible_for_test` placements may use live health, route, lease, heartbeat, and replacement-readiness facts up to 120 seconds old; `eligible_for_noncritical` requires those live facts no older than 60 seconds; `eligible_for_control_plane` requires service health, route state, queue/lease facts, and writer-guard evidence no older than 30 seconds plus policy, class, package, backup, and failover refs no older than five minutes; `active` primary-path shifts require direct heartbeat or endpoint evidence no older than 15 seconds and writer/quorum/fencing evidence no older than 30 seconds. Static identity, enrollment, benchmark, challenge, dispute, security-baseline, and certification refs keep their Oververify policy-versioned expiry windows, but any expired critical ref blocks new placement until a fresh evaluation snapshot exists.
- Resolved: two independent operator signatures are required from the first implementation for break-glass actions that can change backbone safety boundaries: system-service class or placement overrides, stale-evidence or policy-denial overrides, founder-hardware removal or emergency fallback, stateful writer promotion, fencing/quorum bypass, restore-backed cutover, destructive migration or rollback, release-freeze override, privileged package revocation exception, broad primary control-plane route shift, and Overvault/secret-grant expansion for a system service. Routine diagnostics, simulation, read-only reports, noncritical drains, and policy-allowed maintenance actions may be one-signer or service-automation flows when the current class policy allows them. No break-glass path may make unknown public nodes, public sandbox capacity, or public low-sensitivity providers eligible for system-service work.
- Resolved: public system-service placement reports expose only redacted stewardship facts: affected service family, workload class, class version, decision state, broad eligibility state, freshness band, package/profile refs or hash prefixes, safe reason codes, high-level control coverage, coarse region or failure-domain label when policy allows it, timestamp, and redacted Overwatch evidence refs. Public and tenant-facing reports must hide raw node/provider identity, exact topology, endpoint and route internals, lease details, backup manifests, object/chunk/checksum placement, ledger checkpoint ranges, queue high-water marks, registry diffs for backbone services, Overvault key/grant refs, secret classes, signer identities beyond safe role labels, break-glass details, recovery command steps, private incident evidence, and cross-tenant/system-service dependency graphs. Operator and audit views may dereference deeper evidence only through role-scoped Overwatch redaction profiles and audited access decisions.
