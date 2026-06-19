SDS #78

# Migration Tooling SDS

## Purpose

Move services, tenants, data, routes, workloads, accounting refs, event streams, and backbone responsibilities between seed hardware, private swarms, trusted federation pools, public-safe pools, and grid-resident system workloads.

Migration Tooling is the planned, resumable, audited migration coordinator for Overrid. It is how the system moves away from founder-operated seed machines and how tenants, services, routes, data stores, and workloads evolve without hand edits. It does not own deployment planning, live failover, raw data storage, route truth, ledger truth, backup content, or release strategy. It coordinates those services through explicit migration plans, step cursors, preflight checks, cutover windows, verification reports, rollback records, and Overwatch evidence.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [migration_tooling.md](../../service_catalog/governance_ops/migration_tooling.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md), with grid migration tools starting in [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md) |

## Service Family

- Family: Governance, compliance, and operations
- Owning layer: Planned migration sessions, cutover coordination, verification, rollback, resume, and audit evidence
- Primary data scope: migration plans, source/destination inventories, preflight reports, dependency graphs, step records, checkpoints, cursors, data copy refs, event replay refs, route rebinding refs, ledger verification refs, cutover windows, rollback records, and post-migration integrity reports
- First build phase from service plan: grid migration tools begin in [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md), with broader hardening in [Phase 13: Governance, Compliance, and Scale Hardening](../../build_plan/phase_13_governance_compliance_scale_hardening.md)

## Problem Statement

Overrid starts on founder-provided servers and GPUs, but the backbone must eventually run inside the grid without relying on those machines for normal operation. The same migration discipline is needed for tenants, native apps, AI services, storage, routes, federation boundaries, and control-plane upgrades. If migration is a manual operator procedure, the system cannot prove safety, resume after failure, verify ledger and route integrity, or roll back without data loss.

Migration Tooling must make migration a normal platform operation. It should orchestrate preflight checks, backups, copy/replay, dual-run checks, route rebinding, cutover, verification, and rollback through existing service contracts.

## Goals

- Create migration plans for services, system-service workloads, tenant workloads, data stores, routes, namespaces, event streams, accounting refs, and public/federation pool moves.
- Support founder-hardware-to-grid migration for core services while keeping founder hardware only as emergency fallback until drills pass.
- Preflight source and destination eligibility, policy, backup readiness, route ownership, data integrity, ledger checkpoints, queue state, vault grants, and release/failover constraints.
- Track every migration step with idempotency, cursor state, checkpoint refs, retry policy, and rollback requirements.
- Coordinate data copy, event replay, ledger verification, route rebinding, service package migration, and post-migration integrity checks through owning services.
- Support pause, resume, cancel, cutover, rollback, and post-cutover monitoring.
- Produce evidence packages for Incident Response, Stewardship Reporting, and public trust without exposing private data.

## Non-Goals

- Do not replace Deployment Planner for app deployment plans or service provisioning.
- Do not perform live failover decisions; Failover and Recovery Coordinator owns live recovery.
- Do not store or mutate primary data, objects, vault secrets, ledger entries, queue records, or route truth.
- Do not choose release rollout policy; Release Strategy Service owns release channels, health gates, and rollback triggers.
- Do not bypass Overguard, Overwatch, Backup and Restore Service, Overmesh, Overbase, Overstore, Overvault, Seal Ledger, Overqueue, or system-service workload rules.
- Do not silently move regulated or secret-bearing workloads into lower-trust environments.
- Do not add pricing, customer-count, revenue, blockchain, NFT, or speculative transaction assumptions.

## Primary Actors And Clients

- Operators planning founder-hardware removal, grid-resident backbone migration, service moves, and tenant moves.
- Deployment Planner and Grid-Resident Service Packager supplying service packages, config contracts, backup/restore commands, and deployment refs.
- Backup and Restore Service supplying backup manifests, snapshot sets, restore plans, restore session refs, and drill evidence.
- Overbase, Overstore, Overvault, Overqueue, Overregistry, Overwatch, Overmesh, Overtenant, Overpass, and Universal Namespace Service supplying source/destination state and accepting migration commands.
- Seal Ledger, ORU Account Service, Overbill, Overmeter, Provider Payout Service, and Overgrant supplying accounting/usage refs and verification reports.
- Overguard and System-Service Workload Class enforcing destination eligibility, workload class, data class, and operator authority.
- Failover and Recovery Coordinator, Release Strategy Service, Incident Response Service, Compliance Boundary Service, and Stewardship Reporting Service consuming migration evidence.
- Tenant admins, service owners, and affected users receiving authorized status or cutover notices.

## Dependencies

- [Deployment Planner](../deployment_grid/deployment_planner.md), [Grid-Resident Service Packager](../deployment_grid/grid_resident_service_packager.md), and [Package Validator](../deployment_grid/package_validator.md) for deployable service/package contracts.
- [System-Service Workload Class](../deployment_grid/system_service_workload_class.md), [Overguard](../trust_policy_verification/overguard.md), and [Compliance Boundary Service](compliance_boundary_service.md) for destination eligibility, regulated boundaries, and operator authority.
- [Backup and Restore Service](../deployment_grid/backup_restore_service.md) for restore points, snapshot sets, restore sessions, integrity reports, and rollback prerequisites.
- [Failover and Recovery Coordinator](../deployment_grid/failover_recovery_coordinator.md), [Release Strategy Service](../deployment_grid/release_strategy_service.md), and [Overmesh](../execution_scheduling/overmesh.md) for cutover readiness, health gates, route shifts, and rollback behavior.
- [Overbase](../data_storage_namespace/overbase.md), [Overstore](../data_storage_namespace/overstore.md), [Overvault](../data_storage_namespace/overvault.md), [Overqueue](../control_plane/overqueue.md), [Overregistry](../control_plane/overregistry.md), and [Overwatch](../control_plane/overwatch.md) for state copy, object transfer, secret grants, queue drain/replay, registry facts, and audit evidence.
- [Seal Ledger](../accounting/seal_ledger.md), [ORU Account Service](../accounting/oru_account_service.md), [Overbill](../accounting/overbill.md), [Overmeter](../execution_scheduling/overmeter.md), and [Provider Payout Service](../accounting/provider_payout_service.md) for accounting checkpoints, usage reconciliation, billing hooks, and payout safety.

## Owned Responsibilities

Migration Tooling owns:

- Migration plan records, revisions, source/destination inventory snapshots, and dependency graphs.
- Migration preflight aggregation across policy, backup, restore, package, route, namespace, data, vault, ledger, queue, accounting, and health requirements.
- Migration session state, step cursors, checkpoints, retry windows, and idempotency keys.
- Cutover window records, readiness gates, freeze windows, drain requirements, dual-run checks, and final cutover decision refs.
- Command envelopes sent to owning services for copy, replay, route rebind, queue drain, restore, verify, or rollback steps.
- Integrity verification reports and post-migration monitoring records.
- Rollback plans, rollback session records, fallback refs, and resume tokens.
- Migration audit exports and evidence bundles for incident, reporting, and governance review.

It does not own primary state, live routing truth, ledger truth, backup artifacts, package artifacts, or final release strategy.

## Data Model

- `migration_plan`: plan id, migration type, source refs, destination refs, owner refs, scope, phase, plan revision, requested cutover window, risk class, and current state.
- `source_destination_inventory`: service, tenant, workload, data store, route, namespace, queue, ledger, vault, package, node, and policy facts with freshness and owner refs.
- `migration_preflight_report`: validation, policy, backup, restore, capacity, destination trust, route ownership, data integrity, event replay, ledger checkpoint, queue drain, vault grant, release gate, and compliance results.
- `migration_graph`: ordered steps, dependencies, side-effect classes, required freezes, rollback links, parallelization groups, owner services, and verification requirements.
- `migration_step`: step id, command type, owner service, input refs, idempotency key, expected output, timeout, retry policy, state, evidence refs, and rollback step ref.
- `migration_checkpoint`: checkpoint id, step id, data cursor, event cursor, queue cursor, ledger checkpoint, route state, package version, backup/snapshot refs, and integrity hash.
- `cutover_window`: planned start/end, freeze requirements, traffic mode, dual-run scope, go/no-go checks, approver refs, and rollback deadline.
- `integrity_check_report`: copied data refs, event replay counts, ledger checkpoints, route refs, queue state, namespace refs, vault grants, package versions, and mismatch refs.
- `rollback_record`: trigger, rollback plan refs, affected steps, fallback route refs, restore refs, data rewind limits, result refs, and incident refs.
- `migration_replay_bundle`: plan, inventories, preflight, graph, steps, checkpoints, cutover, integrity checks, rollback refs, and Overwatch events.

Common envelope fields: `id`, `tenant_id` or `system_scope`, `actor_id` or `service_account_id`, `trace_id`, `idempotency_key`, `schema_version`, `state`, `created_at`, `updated_at`, `policy_refs`, `evidence_refs`, and `audit_refs`.

## API Surface

- `POST /migrations/plans`: create a migration plan for service, tenant, workload, data, route, namespace, queue, accounting, or backbone migration.
- `POST /migrations/plans/{plan_id}/preflight`: run side-effect-free readiness checks and return reason-coded blockers.
- `GET /migrations/plans/{plan_id}`: read authorized plan state, inventories, graph, blockers, step summaries, and evidence refs.
- `POST /migrations/plans/{plan_id}/approve`: approve a plan, cutover window, and rollback contract.
- `POST /migrations/plans/{plan_id}/execute`: start or resume migration execution from the stored cursor.
- `POST /migrations/plans/{plan_id}/pause`: pause at the next safe checkpoint.
- `POST /migrations/plans/{plan_id}/cancel`: cancel a plan that has not crossed a declared irreversible cutover.
- `POST /migrations/plans/{plan_id}/cutover`: execute final cutover after readiness and approval checks.
- `POST /migrations/plans/{plan_id}/rollback`: execute rollback or fallback plan where policy and checkpoint state allow it.
- `POST /migrations/plans/{plan_id}/integrity-checks`: run or record post-migration integrity verification.
- `GET /migrations/plans/{plan_id}/timeline`: return trace-linked migration timeline and evidence.
- `GET /migrations/replay/{plan_or_session_id}`: reconstruct migration decisions, commands, and results.

Mutating APIs require signed actor/service account, tenant/system scope, trace id, idempotency key, operator approval refs where required, and Overguard policy refs. Stable errors include `migration_scope_missing`, `destination_not_eligible`, `backup_required`, `restore_point_stale`, `route_not_owned`, `ledger_checkpoint_missing`, `queue_not_drained`, `vault_grant_missing`, `cutover_gate_failed`, `rollback_not_safe`, and `migration_cursor_conflict`.

## Event Surface

- `migration_tooling.plan_created`: migration plan created or revised.
- `migration_tooling.preflight_started`: preflight checks started.
- `migration_tooling.preflight_failed`: blockers found with reason-coded refs.
- `migration_tooling.preflight_passed`: plan eligible for approval or execution.
- `migration_tooling.plan_approved`: signed approval recorded.
- `migration_tooling.step_started`: migration step command issued.
- `migration_tooling.step_completed`: migration step succeeded with evidence refs.
- `migration_tooling.step_blocked`: migration step waiting on dependency, retry, operator, or policy.
- `migration_tooling.checkpoint_created`: migration checkpoint recorded.
- `migration_tooling.cutover_started`: final cutover began.
- `migration_tooling.cutover_completed`: cutover completed and verification started.
- `migration_tooling.integrity_verified`: post-migration integrity passed.
- `migration_tooling.rollback_started`: rollback or fallback started.
- `migration_tooling.rollback_completed`: rollback completed with result refs.
- `migration_tooling.plan_completed`: migration completed with evidence.
- `migration_tooling.plan_failed`: migration failed with reason codes and incident refs.
- `migration_tooling.usage_emitted`: usage emitted for migration, verification, rollback, or replay.

Events include plan id, plan revision, source/destination refs, step id, checkpoint id, cutover window, policy refs, evidence refs, and audit refs. Payloads must redact private topology, secrets, private data, and security-sensitive migration details unless caller audience allows them.

## Core Workflow

1. Operator or owning service creates a migration plan with explicit source, destination, scope, and migration type.
2. Migration Tooling inventories source and destination facts through dependency APIs.
3. Preflight checks policy, destination trust, backup/restore readiness, route ownership, data integrity, vault grants, ledger checkpoint, queue drain, package compatibility, capacity, and compliance markers.
4. The tool builds a migration graph with forward steps, verification steps, rollback steps, and cutover gates.
5. Authorized actor approves plan, freeze/cutover window, and rollback contract.
6. Execution copies data, replicates stores, replays events, drains queues, prepares routes, verifies ledger/accounting refs, and confirms health/readiness through owning services.
7. Cutover rebinds routes and service refs after all gates pass.
8. Post-cutover verification checks data, events, ledger, routes, namespaces, queues, vault grants, packages, and monitoring windows.
9. If failure occurs, migration pauses, resumes, or rolls back from checkpoint according to rollback policy.
10. Evidence is exported to Overwatch, Incident Response, Stewardship Reporting, and governance review.

## State Machine

Plan lifecycle:

1. `draft`
2. `preflighting`
3. `blocked`
4. `ready_for_approval`
5. `approved`
6. `executing`
7. `paused`
8. `cutover_pending`
9. `cutover_in_progress`
10. `verifying`
11. `completed`
12. `rollback_required`
13. `rolling_back`
14. `rolled_back`
15. `failed`
16. `cancelled`

Step lifecycle:

1. `pending`
2. `ready`
3. `command_sent`
4. `waiting_for_result`
5. `checkpointed`
6. `succeeded`
7. `retry_wait`
8. `blocked`
9. `failed`
10. `compensated`
11. `skipped_by_revision`

Cutover lifecycle:

1. `not_started`
2. `freeze_requested`
3. `freeze_active`
4. `go_no_go_review`
5. `route_rebinding`
6. `traffic_shifted`
7. `monitoring`
8. `finalized`
9. `rolled_back`

Submitted plans and completed steps are append-only. Revisions, corrections, and rollback records link to prior evidence rather than rewriting history.

## Policy And Security

- Require Overguard decisions for migration scope, source access, destination eligibility, operator authority, regulated workload movement, and cutover/rollback commands.
- Unknown public nodes cannot host backbone services, secret-bearing workloads, or regulated high-compliance workloads.
- Require Backup and Restore Service restore points before destructive state migration or irreversible cutover.
- Require vault grant refs for secret-bearing migrations and never persist raw secrets in migration records.
- Require ledger checkpoint and reconciliation refs for accounting-impact migrations.
- Require route ownership and namespace authority before rebind.
- Require signed operator approval for system-service backbone migration, regulated workload migration, cutover, rollback override, and founder-hardware removal from normal path.
- User-visible migration status must redact private topology, provider details, security-sensitive data, and secret refs.

## Metering And Accounting

- Emit usage refs for preflight, data copy, object transfer, event replay, ledger verification, queue drain, route rebinding, integrity checks, rollback, and operator review.
- Link usage to migration plan, affected services, tenant/system scope, source/destination refs, resource dimensions, and incident/report refs.
- Accounting services remain authoritative for usage, balances, receipts, payouts, grants, and ledger entries.
- Migration Tooling verifies ledger/accounting continuity; it must not create, edit, or erase ledger history.
- Migration overhead for founder-hardware removal and system-service moves should be visible as system-service usage, not hidden operational expense.

## Observability And Operations

- Expose plan backlog, preflight failures, blocked steps, migration duration, copy/replay progress, checkpoint freshness, cutover readiness, rollback readiness, and post-cutover integrity status.
- Alert on stale backups, destination ineligibility, queue-drain timeout, ledger checkpoint mismatch, route rebind failure, vault grant failure, cutover window overrun, verification mismatch, and rollback failure.
- Provide operator views for migration graphs, step cursors, source/destination inventory, blockers, integrity reports, route state, and rollback options.
- Provide dry-run and simulation modes using recorded inventories without issuing side effects.
- Keep founder-hardware dependency status visible until core services leave the normal production path and drills pass.

## Failure Modes And Recovery

- Destination not eligible: block before data transfer and record required eligibility facts.
- Backup or restore point stale: block destructive steps until fresh restore evidence exists.
- Data copy mismatch: quarantine copied refs, rerun copy or roll back before cutover.
- Event replay gap: pause migration and reconcile event source before route changes.
- Ledger checkpoint mismatch: fail verification and involve accounting owners before completion.
- Queue not drained: keep old path active and retry drain or pause cutover.
- Route rebind rejected: keep old route active, record denial, and run rollback/fallback plan.
- Vault grant missing: block secret-bearing migration without exposing secrets.
- Cutover partially completed: use checkpoint, route state, and rollback records to resume or revert.
- Overwatch degraded: spool migration events locally and do not finalize until audit evidence is durable.

## Validation Plan

- A non-critical service can migrate from seed hardware to an eligible grid node with preflight, execution, cutover, verification, and rollback evidence.
- Founder hardware can be removed from the normal path only after backup, restore, failover, rollback, and migration drills pass.
- Ledger, route, queue, namespace, vault, package, and data integrity are verified after migration.
- Failed migration can pause, resume, or roll back safely from checkpoints.
- Regulated or secret-bearing workloads cannot migrate to lower-trust destinations.
- Duplicate execute/cutover/rollback calls with the same idempotency key do not duplicate side effects.
- Public and tenant-visible status hides private topology and sensitive evidence.
- Migration replay reconstructs plan, preflight, step commands, checkpoints, cutover, verification, and rollback.

## Build Breakdown

1. Define migration plan, inventory, preflight, graph, step, checkpoint, cutover, integrity report, rollback, and replay schemas.
2. Implement read-only plan creation and source/destination inventory for one non-critical service.
3. Add preflight checks for policy, backup readiness, destination eligibility, route ownership, package compatibility, and system-service class.
4. Add idempotent step execution with cursor persistence and command refs to owning services.
5. Add data copy, event replay, queue drain, route prebind, and ledger verification adapters.
6. Add cutover window, go/no-go review, route rebind, and post-cutover monitoring.
7. Add pause, resume, cancel, rollback, fallback, and integrity report workflows.
8. Add founder-hardware-to-grid migration drills, dashboards, reporting exports, and Phase 13 hardening tests.

## Handoff And Downstream Use

Migration Tooling hands migration plans, preflight reports, checkpoints, cutover refs, integrity reports, rollback refs, and audit exports to Deployment Planner, Failover and Recovery Coordinator, Backup and Restore Service, Release Strategy Service, Overmesh, Overwatch, Incident Response Service, Stewardship Reporting Service, SDK, CLI, and operator tooling.

Downstream services should expose explicit migration command APIs and return state refs. Migration Tooling must not reach into private storage, route tables, ledgers, queues, or vaults directly.

## Open Design Questions

Resolved decisions:

- The first safe Phase 7 non-critical migration candidate is a non-primary Overwatch/internal-observability replica. It must not own the primary append/checkpoint head, must not carry tenant-private data or raw secrets, must be rebuildable or drainable without user-facing outage, and may only advance through `eligible_for_test` or `eligible_for_noncritical` until Package Validator, System-Service Workload Class, Backup and Restore Service, Release Strategy Service, Failover and Recovery Coordinator, and Overwatch evidence all agree that the package, rollback path, and drill evidence are valid.
- Early data-store migration uses source-authoritative single-writer consistency with snapshot-plus-append-only-replay, not active/active multi-region writes. The source remains authoritative until cutover; the destination is restored from a checkpointed snapshot, replays ordered events or change records with idempotency keys and monotonic cursors, runs shadow integrity checks, then accepts writes only after queues/routes are drained or frozen, writer fencing is recorded, and Overwatch stores cutover evidence. Multi-writer or quorum-style replication is required later for critical write heads before founder fallback is removed.
- Service-account automation may run read-only inventories, dry-run preflights, package checks, backup freshness checks, copy/replay work, idempotent data/object transfer, queue drain within an approved window, non-critical route prebinding, integrity checks, monitoring, safe pause/resume, and bounded retries when Overguard permits the action and evidence is fresh. Signed operator approval is required for plan approval on regulated, secret-bearing, accounting-impacting, public-provider, or system-service scopes; stateful writer promotion; restore-backed cutover; primary route shifts; destructive or irreversible steps; rollback override; break-glass exceptions; founder-hardware removal; and any execution with stale/conflicting safety evidence.
- Cutover windows are classed by user impact and reversibility. Native-app migrations should use scheduled low-traffic windows with user-visible notice, rollback bounds, and no raw private-data exposure; low-risk app route or read-model shifts may use canary/dual-run windows. AI-routing migrations require short canary windows, policy dry-run comparison, quality/safety rollback thresholds, private-context redaction, and fallback to the prior route when classification, privacy, or model-resource evidence is stale. Public-provider flows require the most conservative windows: no in-flight high-risk jobs, payout/finality holds where accounting policy allows, reputation/fraud checks, public-pool capacity warm-up, and Stewardship Reporting/Compliance redaction for any public status.
- Founder hardware remains emergency fallback by evidence gate, not by a fixed convenience period. It can leave the normal path only after Phase 7 recovery evidence shows two consecutive full-backbone cutover drills, at least one planned and one failure-injection, plus per-critical-service backup restore, failover, rollback, queue recovery, Overwatch reconciliation, and route-shift drills with no unresolved `sev_0` or `sev_1` follow-up actions. After normal traffic migrates, founder hardware stays emergency-only until one clean post-cutover monitoring window and the required final rehearsal evidence are complete; then it is downgraded to cold recovery or decommissioned according to Backup and Restore, Failover and Recovery, and Incident Response drill cadence.
