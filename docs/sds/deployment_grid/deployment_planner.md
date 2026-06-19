SDS #46

# Deployment Planner SDS

## Purpose

Turn a signed Overpack application-intent manifest into an ordered, resumable, policy-checked deployment plan.

Deployment Planner is the bridge between intent and execution. It does not build packages, validate artifacts, choose release strategies, run workloads, or store application data. It assembles dependency-aware deployment steps, records why each step exists, submits bounded commands to the services that perform work, and keeps enough evidence to resume, roll back, or explain the deployment later.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [deployment_planner.md](../../service_catalog/deployment_grid/deployment_planner.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 9: Overpack Deployment Platform](../../build_plan/phase_09_overpack_deployment_platform.md) |

## Service Family

- Family: Deployment and grid-resident backbone
- Owning layer: Deployment planning and application platform orchestration
- Primary data scope: deployment plans, plan revisions, provisioning steps, rollback steps, resource reservations, dependency graphs, execution cursors, and deployment evidence refs
- First build phase from service plan: [Phase 9: Overpack Deployment Platform](../../build_plan/phase_09_overpack_deployment_platform.md)

## Problem Statement

Overrid should let developers describe application intent rather than hand-build infrastructure. Without a deployment planner, manifest validation, policy admission, resource reservation, runtime provisioning, data/storage/vault setup, route binding, metering hooks, health checks, and rollback are scattered across operators and individual services. That would recreate centralized-platform fragility inside a distributed grid.

The planner must make deployment repeatable on seed hardware first and grid-resident later. Every plan must be deterministic enough to audit, resumable after partial failure, and reversible where the underlying dependency supports reversal.

## Goals

- Convert a signed Overpack application-intent manifest into a versioned deployment plan with ordered, dependency-aware steps.
- Preserve manifest intent while letting Oversched and Overlease choose eligible resources instead of hard-coding placement.
- Reserve budget and resource capacity before irreversible side effects.
- Coordinate Overbase, Overstore, Overvault, Universal Namespace Service, Overmesh, Overmeter, Overbill, Overguard, and Overwatch through explicit service APIs.
- Record plan inputs, policy decisions, validation reports, resource reservations, commands, state transitions, and evidence refs for replay.
- Make every mutating step idempotent, bounded, and resumable.
- Support rollback plans generated at the same time as forward deployment steps.

## Non-Goals

- Do not validate package internals; Package Validator owns validation reports and warning/error codes.
- Do not decide canary, blue-green, rolling, or route-weight strategy; Release Strategy Service owns those rollout policies.
- Do not run workloads; Overrun and Overcell execute assigned work through leases.
- Do not own data, object, vault, namespace, route, billing, or policy truth.
- Do not bypass Overguard, Overmeter, ORU, Seal Ledger, Overbill, or Overwatch to make deployment faster.
- Do not encode pricing, revenue projections, market assumptions, blockchain mechanics, NFT rights, or per-transaction fee behavior.
- Do not require manual infrastructure edits for normal app deployment once Phase 9 exits.

## Primary Actors And Clients

- Developer, SDK, CLI, admin/developer UI, or AI-generated deployment assistant submitting a signed manifest.
- Overpack providing the manifest, artifact refs, runtime cards, permissions, and package provenance.
- Package Validator providing signed validation reports.
- Overguard providing admission, policy, data-class, and workload-class decisions.
- Oversched and Overlease receiving capacity reservation requests.
- Overbase, Overstore, Overvault, Universal Namespace Service, and Overmesh fulfilling provisioning commands.
- Release Strategy Service requesting rollout-specific plan variants.
- Overwatch, Overmeter, Overbill, and ORU account services consuming evidence and usage hooks.

## Dependencies

- [Overpack](../execution_scheduling/overpack.md) for manifest envelopes, artifact refs, runtime contracts, and provenance.
- [Package Validator](package_validator.md) for package validation reports, policy-compatibility previews, and warning/error codes.
- [Overguard](../trust_policy_verification/overguard.md) and [Policy Dry-Run API](../trust_policy_verification/policy_dry_run_api.md) for admission decisions and previewable denials.
- [Oversched](../execution_scheduling/oversched.md) and [Overlease](../execution_scheduling/overlease.md) for candidate placement, resource reservation, and lease proofs.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), and [Overvault](../data_storage_namespace/overvault.md) for state, object, and secret provisioning.
- [Universal Namespace Service](../data_storage_namespace/universal_namespace_service.md) and [Overmesh](../execution_scheduling/overmesh.md) for names, routes, endpoints, and traffic activation.
- [Overmeter](../execution_scheduling/overmeter.md), [ORU Account Service](../accounting/oru_account_service.md), [Overbill](../accounting/overbill.md), and [Seal Ledger](../accounting/seal_ledger.md) for usage visibility, budget checks, billing refs, and settlement evidence.
- [Overwatch](../control_plane/overwatch.md) for audit events, traces, incidents, and evidence bundles.
- [Backup and Restore Service](backup_restore_service.md) for restore checkpoints required before destructive migrations or risky updates.

## Owned Responsibilities

Deployment Planner owns:

- Deployment plan records and plan revision history.
- Manifest-to-step expansion logic and dependency graph generation.
- Plan-level idempotency, execution cursors, retry windows, and resume checkpoints.
- Preflight aggregation across validation, policy, budget, route ownership, storage/vault readiness, and runtime availability.
- Forward step contracts and rollback step contracts.
- Command envelopes sent to dependency services, including expected idempotency keys and compensation rules.
- Deployment timelines that link plan state, downstream command state, Overwatch events, validation reports, and metering/billing refs.
- Safe cancellation and rollback initiation when the deployment has not reached an irreversible handoff.

## Data Model

- `deployment_plan`: canonical plan produced from a manifest. Fields include `plan_id`, `tenant_id`, `app_id`, `manifest_ref`, `manifest_hash`, `manifest_version`, `requested_by`, `plan_revision`, `state`, `strategy_ref`, `validation_report_refs`, `policy_decision_refs`, `budget_precheck_ref`, `created_at`, `updated_at`, and `audit_refs`.
- `deployment_graph`: ordered dependency graph with `step_id`, `depends_on`, `blocks`, `resource_scope`, `side_effect_class`, `rollback_step_id`, and `parallelization_group`.
- `deployment_step`: executable unit with `step_id`, `service_target`, `command_type`, `command_payload_ref`, `idempotency_key`, `expected_result_schema`, `timeout_policy`, `retry_policy`, `compensation_policy`, `state`, and `evidence_refs`.
- `provisioning_request_ref`: pointer to downstream requests for runtime, state, object storage, vault scopes, namespace records, routes, policy bindings, metering hooks, and billing account links.
- `rollback_plan`: generated rollback graph with `rollback_plan_id`, `source_plan_id`, `safe_to_auto_execute`, `required_restore_points`, `route_reversion_steps`, `data_migration_reversion`, and `operator_approval_required`.
- `deployment_cursor`: current execution cursor with `current_step_ids`, `last_successful_step_ids`, `blocked_step_ids`, `resume_token`, `retry_count`, and `last_error_code`.
- `deployment_evidence_bundle`: immutable refs to Overwatch traces, validation reports, policy decisions, budget checks, route bindings, lease proofs, and final readiness results.

All records use the common Overrid envelope where applicable: stable id, tenant/system scope, actor or service account, trace id, idempotency key, lifecycle state, schema version, policy refs, and audit refs. Draft plan records may be mutable until submitted; submitted plan history is append-only through revisions.

## API Surface

- `POST /deployment-plans`: accepts a signed manifest ref, validation report refs, optional release strategy ref, budget scope, and idempotency key; returns a submitted plan or deterministic denial.
- `POST /deployment-plans/{plan_id}/preflight`: runs dependency, budget, validation, route ownership, policy, and restore-point prechecks without provisioning side effects.
- `GET /deployment-plans/{plan_id}`: returns the plan, graph, current state, step summaries, warnings, and evidence refs subject to tenant and role filters.
- `GET /deployment-plans/{plan_id}/steps`: returns ordered steps with state, dependency, retry, and compensation metadata.
- `POST /deployment-plans/{plan_id}/execute`: starts or resumes execution from the stored cursor after policy and freshness checks.
- `POST /deployment-plans/{plan_id}/pause`: pauses execution at the next safe step boundary.
- `POST /deployment-plans/{plan_id}/cancel`: cancels an unstarted or safely stoppable plan and records downstream cancellation commands.
- `POST /deployment-plans/{plan_id}/rollback`: starts the stored rollback plan or requests operator approval if automatic rollback is unsafe.
- `GET /deployment-plans/{plan_id}/timeline`: returns trace-linked timeline events for debugging, audit, incident review, and user-facing deployment status.

All mutating APIs require tenant context, signed actor or service-account identity, trace id, idempotency key, schema version, and Overguard decision refs. Errors use stable reason codes such as `manifest_not_validated`, `policy_denied`, `budget_not_reserved`, `route_not_owned`, `dependency_not_ready`, `step_conflict`, `rollback_not_safe`, and `stale_plan_revision`.

## Event Surface

- `deployment_planner.plan_submitted`: a manifest was accepted for planning.
- `deployment_planner.preflight_started`: preflight aggregation started.
- `deployment_planner.preflight_passed`: validation, policy, budget, and dependency readiness are sufficient for execution.
- `deployment_planner.preflight_failed`: the plan cannot execute; payload includes stable reason codes and refs, not private content.
- `deployment_planner.plan_revised`: a new plan revision superseded an earlier draft or submitted revision.
- `deployment_planner.step_started`: a downstream command was issued for a step.
- `deployment_planner.step_completed`: a step reached a successful terminal state.
- `deployment_planner.step_blocked`: a step is waiting for dependency, retry window, budget, policy, restore point, or operator action.
- `deployment_planner.rollback_started`: rollback began from a recorded rollback plan.
- `deployment_planner.plan_completed`: deployment reached terminal success and readiness evidence exists.
- `deployment_planner.plan_failed`: deployment reached terminal failure with reason codes and evidence refs.

Events must include `plan_id`, `plan_revision`, `tenant_id`, `app_id`, `trace_id`, `step_id` where relevant, `policy_decision_refs`, `audit_refs`, and data-class-safe summaries.

## Core Workflow

1. Accept a signed Overpack manifest ref and caller envelope through Overgate or an internal service account.
2. Fetch manifest, package validation report, route ownership facts, tenant facts, budget scope, and requested release strategy.
3. Ask Overguard for admission using manifest, validation, workload class, data class, budget, and tenant facts.
4. Generate a dependency graph for runtime, state, storage, vault, namespace, route, policy, metering, billing, health, and release steps.
5. Build a rollback graph before executing the forward graph.
6. Reserve budget and capacity through ORU/Overbill prechecks and Oversched/Overlease reservation paths.
7. Execute provisioning steps in dependency order, recording each downstream command and result.
8. Activate routes only after health/readiness checks and release-strategy gates pass.
9. Confirm metering and billing hooks before marking the deployment complete.
10. On failure, pause, retry, cancel, or roll back according to the step's compensation policy.

## State Machine

Plan lifecycle:

1. `draft`: plan is being generated from manifest and dependency facts.
2. `submitted`: caller has submitted the plan request with idempotency and trace context.
3. `preflighting`: validation, policy, budget, dependency, and restore-point checks are running.
4. `ready`: preflight passed and no side effects have started.
5. `executing`: one or more deployment steps are active.
6. `paused`: execution is intentionally stopped at a safe step boundary.
7. `blocked`: execution cannot continue until a dependency, policy, budget, operator, or retry condition changes.
8. `rolling_back`: rollback plan is active.
9. `completed`: deployment is healthy, routed, metered, and auditable.
10. `failed`: deployment reached terminal failure with evidence and reason code.
11. `cancelled`: deployment stopped before irreversible side effects or after safe compensation.

Step lifecycle:

1. `pending`
2. `ready`
3. `command_sent`
4. `waiting_for_result`
5. `succeeded`
6. `retry_wait`
7. `blocked`
8. `compensating`
9. `failed`
10. `skipped_by_revision`

Submitted plan revisions are never overwritten. Corrections create a new revision linked to the previous revision and its evidence.

## Policy And Security

- Require Overgate admission, Overpass identity, Overtenant scope, Overkey signature verification, and Overguard decisions before side effects.
- Treat AI-generated manifests as untrusted input until Package Validator and Overguard produce evidence.
- Deny deployment when validation reports are missing, stale, mismatched to artifact hash, or issued under incompatible schema versions.
- Never store raw secrets; store Overvault scope refs and command refs only.
- Enforce route ownership and namespace authority before route binding.
- Require restore points or explicit operator approval before destructive migration or irreversible data changes.
- Require signed operator action for override, forced rollback, break-glass route activation, or bypassing a non-fatal warning.
- Keep deployment timelines redacted by data class; user-facing status must not expose provider, secret, internal route, or private topology details unless policy allows it.

## Metering And Accounting

- Emit planning CPU/time usage and operator-attention events to Overmeter when they are material enough to track.
- Require ORU budget precheck and reservation refs before provisioning compute, storage, network, model, or service resources.
- Link plan, app, tenant, workload class, and resource dimensions to metering bindings.
- Confirm Overbill hooks before final route activation for billable app workloads.
- Use Seal Ledger refs only as immutable accounting evidence; Deployment Planner must not mutate ledger history or maintain balances.
- Keep native/public-utility deployments structural and near-cost without revenue or customer-count assumptions.

## Observability And Operations

- Expose health for manifest fetch, validation report fetch, policy decisions, budget prechecks, scheduler reservation, provisioning adapters, route activation, and Overwatch event writes.
- Track plan queue depth, preflight latency, execution latency, blocked-step counts, retry counts, rollback counts, and stale-plan counts.
- Provide operator views for plan graph, step state, downstream command refs, current cursor, reason codes, and redacted evidence bundles.
- Support replay mode that recomputes a plan from stored inputs without issuing side effects.
- Support dry-run comparisons between two manifest versions to show added, removed, and changed deployment steps.
- Keep migration/backfill operations separate from normal deployment execution and expose readiness before accepting mutating commands.

## Failure Modes And Recovery

- Missing or invalid validation report: reject preflight and identify the required Package Validator output.
- Policy denial: stop before side effects and return policy decision refs and safe correction paths.
- Budget reservation failure: leave plan in `blocked` or `failed` without provisioning resources.
- Capacity reservation conflict: retry through Oversched/Overlease or ask the caller for changed requirements.
- Downstream command timeout: preserve idempotency key and query dependency state before retry.
- Partial provisioning failure: continue only if failed step is safely retryable; otherwise pause and offer rollback.
- Route activation failure: keep old route active, record route decision failure, and avoid half-routed state.
- Health gate failure: call Release Strategy Service for rollback or pause behavior when a strategy is attached.
- Rollback failure: escalate to incident response with restore-plan refs and operator approval requirements.
- Evidence write failure: do not mark a deployment completed until Overwatch records are durably written or recovered from a local spool.

## Validation Plan

- A signed Overpack manifest provisions one app through runtime, state, storage, vault, namespace, route, metering, and billing bindings.
- The same manifest and inputs produce the same plan graph and reason codes.
- Invalid, stale, or hash-mismatched validation reports fail before side effects.
- Policy denial, missing route ownership, insufficient budget, and missing restore points fail during preflight.
- Duplicate `POST /deployment-plans` and `POST /execute` calls with the same idempotency key return the same plan/cursor outcome.
- Simulated downstream timeout resumes from stored step state without duplicate provisioning.
- Failed deployment can pause, resume, cancel, or roll back with evidence.
- Route activation is blocked until health/readiness checks pass.
- No normal deployment path requires manual infrastructure edits.
- Plan replay reconstructs key decisions from stored inputs, refs, and policy versions.

## Build Breakdown

1. Define `deployment_plan`, `deployment_step`, `deployment_graph`, `deployment_cursor`, and `rollback_plan` schemas in the shared schema package.
2. Implement manifest intake, validation-report binding, and read-only plan generation.
3. Add preflight aggregation for Overguard, budget, route ownership, restore point, storage/vault readiness, and scheduling feasibility.
4. Add idempotent step execution with downstream command refs and cursor persistence.
5. Add rollback-plan generation and rollback execution for safe step classes.
6. Add health/readiness integration and route activation gates.
7. Add metering and billing hook confirmation.
8. Add plan replay, timeline queries, and operator diagnostics.
9. Prove one signed manifest deploys and rolls back without manual infrastructure edits.

## Handoff And Downstream Use

Deployment Planner hands a ready, evidence-backed deployment timeline to SDK, CLI, admin/developer UI, Overwatch, Overmeter, Overbill, Release Strategy Service, and incident response. Release Strategy Service may attach rollout policy to a plan, but should not edit private planner records directly.

Future native apps, mobile services, and AI-generated deployment tools should call the planner through the documented API, not by writing deployment records or provisioning dependencies themselves.

## Open Design Questions

- Resolved: the stable plan graph format is `deployment_graph_v0` in the shared schema package, represented first as canonical JSON plus JSON Schema. It is a versioned DAG with typed node and edge records for plan/step ids, dependencies, blockers, service targets, command types, resource scopes, side-effect classes, idempotency keys, rollback links, parallelization groups, reason codes, manifest and policy refs, and Overwatch evidence refs. UI views, replay, dry-run diffing, and operator timelines must render from this canonical graph or generated read models, not from a separate UI-only graph. Protobuf can be generated later for compact internal transport, but it must not replace the canonical docs-facing schema.
- Resolved: automatic compensation is allowed only for steps whose effects are provisional, idempotent, and not yet externally visible: releasing unused budget or lease reservations, cancelling queued downstream commands, deleting unactivated route drafts, removing provisional namespace/service records, discarding staged artifacts, clearing unfinalized metering or billing hook drafts, and reverting health-gated deployment work before traffic activation. Operator approval is required for destructive state migrations, schema/index rewrites, restore-backed rollback, secret/vault grant or rotation changes, ledger/accounting corrections, production or system-service route cutovers, break-glass overrides, data deletion, and any step whose downstream owner reports `operator_approval_required`.
- Resolved: draft plans may remain readable for review for 24 hours, but execution eligibility is always based on a fresh preflight. Manifest and validation refs stay valid only while their hashes, schema/ruleset versions, signer authority, and revocation state remain current; policy, route ownership, restore readiness, and health-gate facts must be refreshed at submit and execute time and after 15 minutes of draft age; budget and capacity reservations must be refreshed after five minutes or the reservation/lease TTL, whichever is shorter. A stale draft becomes `blocked` or `stale_plan_revision`, and correction creates a new plan revision instead of mutating submitted history.
- Resolved: planner-only activation is limited to first-time route bindings or internal/private test routes with no existing live traffic, no version-pin change, no route-weight progression, and passing Overguard, namespace ownership, Overmesh authorization, health, readiness, metering, billing, and Overwatch evidence gates. Release Strategy Service approval is required for canary, rolling, blue-green, production channel, public/user-facing, system-service, route-weight, version-pin, route transfer, freeze-window, rollback-trigger, break-glass, and any change that shifts or preserves existing live traffic while another version remains eligible.
- Resolved: Phase 9 stateful migrations require a restore-point ref produced through Backup and Restore Service or a service-native restore contract that it recognizes. The minimum contract is a verified snapshot set or migration checkpoint with tenant/app/plan refs, source service refs, manifest and package refs, dependency graph refs, consistency window, high-water marks or checkpoints/cursors, checksums/signatures, integrity verification report, restore order hints, Overvault grant refs where secrets are involved, destination trust requirements, rollback path, expected verification checks, and Overwatch evidence. Destructive migration steps cannot enter `ready` until the restore point is current for the declared RPO/RTO class, verified after creation, policy-approved, and safe for the target workload class.
