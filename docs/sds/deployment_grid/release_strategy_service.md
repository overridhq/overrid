SDS #50

# Release Strategy Service SDS

## Purpose

Define, approve, and supervise rollout strategies for app and system-service deployments.

Release Strategy Service owns rollout intent: rolling update, blue-green deployment, canary deployment, route-weight changes, version pins, promotion rules, freeze windows, rollback triggers, and release approvals. It does not build packages, validate package internals, generate full deployment plans, execute workload steps, store backups, or perform emergency failover by itself.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [release_strategy_service.md](../../service_catalog/deployment_grid/release_strategy_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 9: Overpack Deployment Platform](../../build_plan/phase_09_overpack_deployment_platform.md) |

## Service Family

- Family: Deployment and grid-resident backbone
- Owning layer: Release policy, rollout supervision, promotion, rollback, and version-pinning strategy
- Primary data scope: release plans, release channels, strategy templates, rollout windows, traffic-shift policy, version pins, promotion gates, health gates, rollback triggers, freeze records, approval records, and release evidence refs
- First build phase from service plan: [Phase 9: Overpack Deployment Platform](../../build_plan/phase_09_overpack_deployment_platform.md)

## Problem Statement

Overrid must support safe application and system-service updates without manual infrastructure edits. Deployment Planner can create ordered steps, and Overmesh can shift routes, but neither should own rollout policy. Without a release strategy boundary, canary percentages, blue-green cutovers, version pins, health gates, rollback thresholds, freeze windows, and operator approvals become scattered across deployments and hard to audit.

This service makes release intent explicit and reusable. It lets a developer, operator, or automation choose a strategy, bind it to a validated package and deployment plan, supervise promotion gates, and trigger rollback or freeze behavior when health or policy conditions fail.

## Goals

- Define release plans for rolling, blue-green, canary, manual rollout, automatic rollback, route-weight, and version-pin strategies.
- Bind release plans to Deployment Planner plans, Package Validator reports, Overpack package versions, and Overmesh route refs.
- Gate promotions on health, readiness, metering hook confirmation, policy decisions, route ownership, and optional operator approval.
- Support maintenance windows, release freezes, emergency pauses, and signed break-glass actions.
- Keep route-weight changes and version pins auditable and replayable.
- Trigger rollback plans through Deployment Planner and Failover and Recovery Coordinator where appropriate.
- Support native apps, system services, third-party apps, and AI-generated app deployments without bypassing package validation or policy.

## Non-Goals

- Do not generate full deployment step graphs; Deployment Planner owns plan graphs and execution cursors.
- Do not validate artifacts or package manifests; Package Validator owns validation reports.
- Do not run workloads or directly manipulate node-level execution.
- Do not store backup data or perform restore sessions; Backup and Restore Service owns those records.
- Do not perform live outage failover independently of Failover and Recovery Coordinator.
- Do not bypass Overguard, Overmesh, Overwatch, Overmeter, ORU, Seal Ledger, or Overbill.
- Do not include pricing, revenue, customer-count, blockchain, NFT, or per-transaction-fee assumptions.

## Primary Actors And Clients

- Developer, SDK, CLI, admin/developer UI, or AI-generated deployment assistant requesting a rollout strategy.
- Deployment Planner attaching release strategy to plan execution.
- Package Validator providing validation report refs for package versions.
- Overpack and Overregistry providing package version and provenance facts.
- Overmesh receiving route-weight and route-binding strategy commands.
- Overwatch providing health events, readiness evidence, traces, and release audit records.
- Overguard deciding release policy, freeze rules, break-glass permissions, and rollback authority.
- Failover and Recovery Coordinator receiving emergency rollback/failover handoff when health failure becomes an incident.
- Operators approving risky system-service or production release actions.

## Dependencies

- [Deployment Planner](deployment_planner.md) for ordered deployment steps, execution cursor, rollback plan refs, and deployment timelines.
- [Package Validator](package_validator.md) for validation report refs and package safety evidence.
- [Overpack](../execution_scheduling/overpack.md) and [Overregistry](../control_plane/overregistry.md) for package version facts, artifact refs, and manifest hashes.
- [Overmesh](../execution_scheduling/overmesh.md) for route weights, route bindings, endpoint health, and traffic-shift commands.
- [Overwatch](../control_plane/overwatch.md) for health gates, audit events, traces, incidents, and evidence bundles.
- [Overguard](../trust_policy_verification/overguard.md) for release policy, freeze windows, approval requirements, data-class restrictions, and break-glass authorization.
- [Backup and Restore Service](backup_restore_service.md) for restore-point refs required before risky releases or stateful rollbacks.
- [Failover and Recovery Coordinator](failover_recovery_coordinator.md) for incident-grade recovery when release rollback becomes service recovery.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), and [Overbill](../accounting/overbill.md) for release-related usage visibility and billing hook confirmation.

## Owned Responsibilities

Release Strategy Service owns:

- Release plan records and release strategy templates.
- Release channel, environment, and version-pin records.
- Promotion gate definitions, health gate thresholds, rollout windows, freeze records, and approval requirements.
- Canary, rolling, blue-green, and route-weight progression state.
- Rollback trigger definitions and rollback requests sent to Deployment Planner or Failover and Recovery Coordinator.
- Release evidence bundles linking plan refs, package refs, validation refs, route refs, health refs, policy refs, and operator approvals.
- Release pause, resume, promote, rollback, and freeze command envelopes.
- Strategy compatibility checks against workload class, data class, package version, route type, and deployment plan state.

## Data Model

- `release_plan`: root release record with `release_id`, `tenant_id`, `app_id_or_service_id`, `environment`, `channel`, `strategy_type`, `source_version_ref`, `target_version_ref`, `deployment_plan_ref`, `validation_report_refs`, `state`, `policy_refs`, and `audit_refs`.
- `release_strategy_template`: reusable template with `strategy_type`, `traffic_steps`, `promotion_gates`, `rollback_triggers`, `approval_requirements`, `max_duration`, `route_behavior`, and `supported_workload_classes`.
- `release_channel`: named channel such as `dev`, `test`, `private`, `production`, or `system`, with allowed actors, freeze windows, version-pin rules, and promotion policy.
- `traffic_shift_step`: route progression unit with `step_id`, `route_ref`, `from_version`, `to_version`, `weight`, `hold_duration`, `health_gate_ref`, and `state`.
- `health_gate`: readiness and health rule with `gate_id`, `metric_refs`, `thresholds`, `minimum_sample_window`, `failure_budget`, `incident_link_behavior`, and `state`.
- `version_pin`: durable pin with `pin_id`, `target_scope`, `package_version_ref`, `reason`, `expires_at`, `created_by`, `policy_refs`, and `state`.
- `rollback_trigger`: condition that requests rollback, including failed health gate, policy revocation, package revocation, operator action, incident ref, or budget/metering hook failure.
- `release_evidence_bundle`: immutable refs to deployment plan, package validation, Overwatch traces, route shifts, policy decisions, approvals, and rollback outcomes.

Accepted release records are append-only by transition. Changed rollout intent creates a release revision or a new release plan depending on whether side effects have started.

## API Surface

- `POST /release-plans`: creates a release plan from package/version refs, deployment plan ref, strategy type, channel, environment, and policy context.
- `POST /release-plans/{release_id}/preflight`: checks validation refs, route ownership, health gate availability, freeze windows, restore-point refs, and policy before rollout.
- `GET /release-plans/{release_id}`: returns release state, strategy, channel, version pins, gates, route steps, and evidence refs.
- `POST /release-plans/{release_id}/start`: starts rollout supervision after Deployment Planner is ready.
- `POST /release-plans/{release_id}/promote`: promotes to the next traffic step or finalizes blue-green cutover when gates pass.
- `POST /release-plans/{release_id}/pause`: pauses rollout and keeps current route/version state.
- `POST /release-plans/{release_id}/resume`: resumes a paused rollout after policy and gate freshness checks.
- `POST /release-plans/{release_id}/rollback`: requests rollback through Deployment Planner or Failover and Recovery Coordinator according to strategy and incident state.
- `POST /release-plans/{release_id}/freeze`: freezes rollout for a scope or channel with policy/approval refs.
- `POST /version-pins`: creates or updates a version pin for app, route, channel, or system-service scope.
- `GET /release-plans/{release_id}/timeline`: returns release audit, gate, route, approval, and rollback events.

Mutating APIs require actor identity, tenant or system scope, trace id, idempotency key, channel authorization, policy decision refs, and validation report refs when package versions are involved. Stable errors include `validation_report_missing`, `strategy_not_allowed`, `release_frozen`, `health_gate_failed`, `route_shift_denied`, `version_pin_conflict`, `rollback_not_available`, `approval_required`, and `deployment_plan_not_ready`.

## Event Surface

- `release_strategy.plan_created`: release plan was created.
- `release_strategy.preflight_passed`: release has required validation, policy, route, health, and restore evidence.
- `release_strategy.preflight_failed`: rollout cannot start with reason codes and refs.
- `release_strategy.rollout_started`: rollout supervision began.
- `release_strategy.traffic_step_started`: route or version progression step started.
- `release_strategy.health_gate_passed`: a promotion gate passed.
- `release_strategy.health_gate_failed`: a promotion gate failed and pause or rollback behavior is selected.
- `release_strategy.promoted`: release advanced to next step or final target.
- `release_strategy.paused`: rollout paused with reason.
- `release_strategy.rollback_requested`: rollback requested through planner or recovery coordinator.
- `release_strategy.version_pinned`: version pin created, changed, expired, or removed.
- `release_strategy.frozen`: release scope or channel frozen.
- `release_strategy.completed`: release reached terminal success.
- `release_strategy.failed`: release reached terminal failure.

Payloads include release id, channel, environment, app/service scope, source and target version refs, route refs, policy refs, health gate refs, operator refs where relevant, and redacted evidence refs.

## Core Workflow

1. Accept release request with package/version refs, deployment plan ref, channel, environment, strategy type, and actor envelope.
2. Fetch validation reports, package facts, deployment-plan state, route refs, health gate definitions, policy/freeze facts, and restore-point refs when required.
3. Ask Overguard whether the release strategy, channel, actor, data class, workload class, and approval state are allowed.
4. Build release plan and strategy-specific traffic/promotion steps.
5. Preflight health telemetry availability, route control, version pin conflicts, billing/metering hook readiness, and rollback availability.
6. Start rollout supervision and request Deployment Planner execution or route progression at safe step boundaries.
7. Evaluate health gates and promotion criteria after each step.
8. Promote, pause, freeze, or rollback according to strategy and policy.
9. Finalize release evidence and update version pins or channel state.

## State Machine

Release lifecycle:

1. `draft`
2. `preflighting`
3. `awaiting_approval`
4. `ready`
5. `rolling_out`
6. `paused`
7. `frozen`
8. `promoting`
9. `rolling_back`
10. `completed`
11. `failed`
12. `cancelled`

Traffic step lifecycle:

1. `pending`
2. `route_command_sent`
3. `observing`
4. `gate_passed`
5. `gate_failed`
6. `promoted`
7. `rolled_back`
8. `skipped`

Version pin lifecycle:

1. `proposed`
2. `active`
3. `conflicted`
4. `expired`
5. `superseded`
6. `removed`

Release history is append-only once rollout starts. Corrections after side effects are represented as pause, rollback, new release plan, or version-pin change, not silent mutation.

## Policy And Security

- Require Overguard decisions for release strategy, actor/channel authority, data-class constraints, production release, system-service release, freeze, rollback, and break-glass action.
- Require Package Validator report refs for package versions before rollout.
- Require route ownership and Overmesh authorization before traffic shifts.
- Require restore-point refs or explicit policy exception before stateful releases with destructive migrations.
- Require signed operator approval for system-service production releases, release freeze overrides, and risky rollback.
- Treat AI-generated releases as untrusted until validation, policy preview, and human approval rules pass.
- Redact health, route, provider, and package details according to data class and role.
- Preserve old version route/pin state until the strategy declares cutover final.

## Metering And Accounting

- Emit release orchestration usage, route-shift operations, observation windows, and rollback overhead to Overmeter where material.
- Confirm deployment plan metering and billing hooks before promoting a billable app release to active traffic.
- Link release usage to tenant, app/service, package version, channel, route, strategy type, and deployment plan.
- Use ORU and Seal Ledger only through accounting rails; Release Strategy Service does not maintain balances or ledger entries.
- Keep release mechanics structural and near-cost without encoding product financial forecasts.

## Observability And Operations

- Expose active releases, paused releases, frozen channels, failed gates, rollback requests, route-shift lag, promotion latency, and version-pin conflicts.
- Provide operator views for release timeline, current traffic weights, gate samples, policy decisions, validation refs, route refs, and rollback availability.
- Support release simulation using stored health and route fixtures before production rollout.
- Track strategy-level success/failure rates and most common rollback triggers.
- Support release diff between source and target package/version refs.
- Keep audit exports tied to Overwatch evidence for incident review, disputes, stewardship reporting, and governance hardening.

## Failure Modes And Recovery

- Validation report missing or stale: block release preflight.
- Release freeze active: deny start or promotion unless authorized override exists.
- Version pin conflict: stop before route or deployment changes and require pin resolution.
- Health gate unavailable: block promotion and either hold or rollback according to strategy.
- Health gate failed: pause, roll back, or escalate based on rollback triggers.
- Route shift rejected by Overmesh: preserve current route weights and mark step blocked.
- Deployment Planner not ready: keep release ready or blocked without issuing route changes.
- Package revoked mid-release: freeze rollout and request rollback or incident response.
- Rollback request fails: escalate to Failover and Recovery Coordinator and incident response with evidence refs.
- Evidence write degraded: do not mark release complete until Overwatch evidence is durable or recovered from spool.

## Validation Plan

- Canary deployment can start, observe health, promote, and complete.
- Canary health failure pauses or rolls back according to strategy.
- Blue-green cutover preserves previous route until final promotion.
- Rolling update advances only when prior step passes readiness and health gates.
- Release freeze blocks promotion unless signed override passes policy.
- Version pins are auditable, conflict-detected, and reversible.
- Route weights and package version refs are replayable from release evidence.
- Rollback request uses Deployment Planner rollback plan for deployment rollback and Failover and Recovery Coordinator for incident-grade recovery.
- AI-generated release cannot bypass validation, policy, or approval rules.

## Build Breakdown

1. Define release plan, strategy template, channel, traffic step, health gate, rollback trigger, version pin, and evidence schemas.
2. Implement release plan creation and read APIs for rolling, blue-green, canary, and manual strategies.
3. Add preflight checks for validation reports, deployment plan readiness, route ownership, policy, freeze windows, restore refs, and metering hooks.
4. Add Overmesh traffic-step command integration and route-weight evidence.
5. Add health gate evaluation from Overwatch events.
6. Add pause, resume, promote, rollback, freeze, and version-pin APIs.
7. Add Deployment Planner rollback integration and Failover and Recovery Coordinator escalation.
8. Prove canary or blue-green deployment for one signed Overpack app manifest.

## Handoff And Downstream Use

Release Strategy Service hands rollout state, route-step evidence, version-pin records, and rollback requests to Deployment Planner, Overmesh, Overwatch, SDK, CLI, admin/developer UI, incident response, and stewardship reporting. It should remain the release-policy owner while delegating step execution and emergency recovery to the services that own those domains.

Native apps, system services, third-party apps, and AI-generated deployment tools should use release plans instead of directly manipulating routes or version pins.

## Open Design Questions

- Resolved: the first Phase 9 app deployment must not be manual-only. The minimum first proof is a manually approved release plan with either canary or blue-green rollout, chosen by app shape: canary for route-weight-capable services with meaningful health samples, blue-green for services that can run a complete parallel target and cut over atomically. Rolling updates remain part of the Phase 9 release-strategy contract, but they do not need to be the first demonstrated app update because they are harder to reason about when per-replica health, rollback, and placement evidence are still maturing. By Phase 9 exit, Release Strategy Service should support manual, canary, blue-green, rolling, route-weight, rollback, and version-pin records; the first signed manifest proof can validate canary or blue-green plus manual pause/promote/rollback controls.
- Resolved: default health gates are versioned `health_gate_profile` records that Overguard can tighten by channel, workload class, data class, and system-service class. Low-risk private apps start with a simple gate: current package validation and policy refs, successful readiness probe, route endpoint availability, no critical Overwatch errors in the observation window, metering/billing hook confirmation when billable traffic is involved, and no active rollback trigger; traffic-bearing canaries should require a small minimum observation window or sample count before promotion. System-service releases require stricter defaults: current restore-point or rollback-command evidence, fresh health/route/lease/queue evidence according to System-Service Workload Class, durable Overwatch evidence, signed operator approval where the class requires it, no degraded audit/metering/policy hooks, and longer observation or maintenance-window approval before promotion. Threshold values are policy data, not hardcoded service constants.
- Resolved: freeze windows block normal starts, promotions, route-weight increases, forward version changes, and non-urgent cutovers. An urgent security rollback is allowed inside a freeze only as a signed, policy-approved exception to a previously validated known-good package/version pin or rollback plan, with Overguard break-glass or emergency rollback authority, Overwatch incident evidence, and preservation of the current release timeline. A freeze override must not be used to introduce an unvalidated new package through the rollback path; emergency forward fixes still require Package Validator, Deployment Planner, Overguard, route, health, and approval gates under an emergency release channel.
- Resolved: users and developers should directly see pins that affect their app, tenant, route, channel, or deployment decisions: active package version, target version, rollback target, channel pin, expiry, status, and a redacted reason/evidence ref when a pin blocks action. Operator-only controls include system-service and backbone pins, safety holds from Incident Response or Compliance Boundary, security quarantine pins, privileged signer or package revocation pins, internal route/replica pins, and placement or recovery pins that would reveal sensitive topology. When an operator-only pin affects a user-visible release, the API should return a stable reason code and redacted evidence ref instead of exposing hidden system details.
- Resolved: early automation should be conservative until Overwatch durability, incident-response playbooks, and recovery drills have proven the path. Phase 9 can automatically run preflight, build release plans, start private test rollouts after signed approval, pause on failed gates, and promote low-risk private canaries when all health gates pass. Automatic rollback should initially be limited to low-risk app releases with a known-good previous version, no destructive state migration, no system-service scope, and fresh Overwatch/Overmesh/Deployment Planner evidence. Production, public/user-facing, stateful migration, system-service, freeze override, break-glass, and incident-grade recovery actions require signed operator approval until the relevant backup, restore, failover, rollback, and incident drills are current for that workload class.
