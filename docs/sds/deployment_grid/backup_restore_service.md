SDS #45

# Backup and Restore Service SDS

## Purpose

Protect critical control-plane, ledger, registry, queue, policy, storage, and package state with scheduled backups, verifiable backup manifests, restore plans, restore drills, integrity reports, retention records, and disaster-recovery evidence.

Backup and Restore Service is the recovery contract that makes Phase 7 grid-resident backbone migration credible. Founder hardware can start Overrid, but the ecosystem cannot depend on founder machines forever. This service proves core state can be backed up, restored into clean trusted environments, verified, and audited without rewriting history or bypassing policy.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [backup_restore_service.md](../../service_catalog/deployment_grid/backup_restore_service.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md) |
| SDS sub-build plan | [SUB BUILD PLAN #45 - Backup and Restore Service](../../build_plan/sub_build_plan_045_backup_restore_service.md) |

## Service Family

- Family: Deployment and grid-resident backbone
- Owning layer: backup policies, backup manifests, snapshot sets, restore plans, restore sessions, integrity verification, restore drills, retention/tombstone evidence, disaster-recovery events, and recovery audit exports
- Primary data scope: backup target definitions, dependency snapshots, content/checksum refs, ledger checkpoints, registry snapshots, queue snapshots, policy snapshots, object/storage refs, secret/vault refs, restore commands, restore reports, and drill evidence
- First build phase from service plan: [Phase 7: Grid-Resident Backbone](../../build_plan/phase_07_grid_resident_backbone.md).

## Problem Statement

Phase 7 requires Overrid core services to run as protected grid-resident system workloads. That migration is unsafe if critical state cannot be restored after corruption, failed upgrades, node loss, split-brain, operator mistakes, or founder-hardware removal.

The ill-design to avoid is treating backup as a raw file copy or ad hoc database dump. Overrid needs service-aware manifests, integrity verification, policy-controlled restore commands, ledger/registry reconciliation, restore drills, and auditable recovery events.

## Goals

- Define backup targets and retention policies for core service state.
- Create verifiable backup manifests for Overbase, Overstore, Seal Ledger, Overregistry, Overqueue, Overwatch, policy stores, package refs, and supporting stores.
- Record scheduled and manual backup runs with checksums, signatures, dependency refs, and redaction/encryption policy refs.
- Restore critical stores into clean environments with explicit restore plans and restore sessions.
- Verify ledger, registry, queue, storage, package, and policy state after restore.
- Record restore drills in Overwatch before founder hardware leaves normal production.
- Provide recovery evidence to Failover and Recovery Coordinator, Release Strategy Service, Migration Tooling, and operators.

## Non-Goals

- Do not own the primary state engines. Overbase, Overstore, Seal Ledger, Overregistry, Overqueue, Overwatch, Overvault, and policy stores own their state.
- Do not rewrite ledger, registry, queue, or policy history during restore.
- Do not decide live traffic failover alone. Failover and Recovery Coordinator owns active failover.
- Do not package system services. Grid-Resident Service Packager owns deployment artifacts.
- Do not manage release rollout/rollback policy. Release Strategy Service owns release control.
- Do not expose raw secrets, key material, payment credentials, or private data in backup manifests.
- Do not allow restore into untrusted public nodes or lower-trust environments.

## Primary Actors And Clients

- Platform operators and signed break-glass operators.
- Failover and Recovery Coordinator, requesting restore readiness and drill evidence.
- Release Strategy Service, requiring pre-upgrade backup and rollback evidence.
- Grid-Resident Service Packager, requiring backup/restore commands in system-service packages.
- Migration Tooling, using restore evidence during founder-hardware migration and grid-resident cutover.
- Overbase, Overstore, Seal Ledger, Overregistry, Overqueue, Overwatch, Overguard, Overpass, and supporting stores, exporting/restoring service-aware snapshots.
- Overvault, supplying encryption key refs and secret-restore policy refs.
- Admin UI, CLI, SDK, and Overwatch, exposing authorized backup/restore status and audit history.

## Dependencies

- [Overbase](../data_storage_namespace/overbase.md) for structured state snapshots and restore-plan metadata.
- [Overstore](../data_storage_namespace/overstore.md) for content-addressed object snapshots, chunks, manifests, and repair refs.
- [Seal Ledger](../accounting/seal_ledger.md) for ledger streams, checkpoints, and accounting reconciliation after restore.
- [Overregistry](../control_plane/overregistry.md) for service/package/provider/node manifest snapshots.
- [Overqueue](../control_plane/overqueue.md) for queue item snapshots, dead-letter state, retry state, and worker handoff safety.
- [Overwatch](../control_plane/overwatch.md) for audit events, incident refs, restore drill evidence, and integrity reports.
- [Overvault](../data_storage_namespace/overvault.md) for backup encryption refs, secret refs, and restore grants.
- [Overguard](../trust_policy_verification/overguard.md) for restore authorization policy.
- [System-Service Workload Class](system_service_workload_class.md), [Failover and Recovery Coordinator](failover_recovery_coordinator.md), and [Release Strategy Service](release_strategy_service.md) for grid-resident operations integration.

## Owned Responsibilities

Backup and Restore Service owns:

- Backup target catalog for critical services and stores.
- Backup policy records, including frequency, retention, encryption, locality, replication, and drill requirements.
- Backup manifest records with target refs, content refs, sequence/checkpoint refs, checksums, signatures, and dependency graph refs.
- Snapshot set records that group multi-service backups into a restore-consistent recovery point.
- Integrity verification reports for backup creation and periodic recheck.
- Restore plan records that define target environment, service order, quiesce requirements, restore steps, expected checkpoints, and rollback path.
- Restore command records and restore session state.
- Restore drill reports with reconciliation evidence.
- Disaster recovery events and recovery audit exports.
- Retention expiry and tombstone records for obsolete backup refs.

It does not own raw primary state, live traffic routing, package builds, release rollout, or secret material.

## Data Model

The first implementation must define:

- `backup_target`: service/store id, data classes, snapshot method, consistency mode, quiesce requirement, encryption policy, retention class, and restore priority.
- `backup_policy`: target refs, schedule, retention, replication count, locality requirements, integrity-check cadence, drill cadence, and operator approval requirements.
- `backup_run`: run id, trigger type, target refs, start/end times, actor/service account, state, and failure reason codes.
- `backup_manifest`: manifest id, target refs, content refs, chunk refs, ledger checkpoint refs, registry version refs, queue high-water marks, policy version refs, checksums, signatures, and audit refs.
- `snapshot_set`: set id, member manifests, dependency graph, consistency window, recovery-point objective label, and restore-order hints.
- `integrity_verification_report`: manifest/set id, checksum verification, signature verification, dependency verification, sample restore verification, and mismatch refs.
- `restore_plan`: plan id, source snapshot set, destination environment, trust requirements, restore order, quiesce/maintenance requirements, secret/vault grant refs, and expected verification checks.
- `restore_session`: plan id, command id, actor/service account, state, restored target refs, verification refs, rollback refs, and incident refs.
- `restore_drill_report`: drill id, snapshot set, environment, start/end, findings, reconciliation evidence, and pass/fail state.
- `retention_tombstone`: backup refs removed/expired, retention policy version, deletion proof refs, and audit refs.
- `disaster_recovery_event`: incident/disaster id, affected targets, selected restore plan, actions taken, final state, and follow-up refs.

Common envelope fields:

- `id`
- `system_scope` or `tenant_id` where applicable
- `actor_id` or `service_account_id`
- `trace_id`
- `idempotency_key`
- `state`
- `schema_version`
- `policy_refs`
- `audit_refs`
- `created_at`
- `updated_at`

## API Surface

Phase 7 should expose:

- `POST /backup-targets`: register or update a backup target definition.
- `POST /backup-policies`: create or revise a backup policy.
- `POST /backup-runs`: trigger a manual backup run or enqueue an immediate scheduled run.
- `GET /backup-runs/{id}`: read backup state and failure/verification refs.
- `GET /backup-manifests/{id}`: read authorized manifest metadata and integrity refs.
- `POST /snapshot-sets`: assemble a consistent multi-service snapshot set from manifests.
- `POST /integrity-verifications`: verify one manifest or snapshot set.
- `POST /restore-plans`: create a restore plan against a snapshot set and destination environment.
- `POST /restore-sessions`: execute an approved restore plan.
- `POST /restore-drills`: execute or record a controlled restore drill.
- `GET /restore-sessions/{id}/report`: read restore verification and reconciliation evidence.
- `POST /retention/apply`: apply retention and create tombstone evidence.
- `GET /disaster-recovery/events/{id}`: read disaster-recovery actions and evidence.

API rules:

- Restore commands require signed operator or authorized system-service action plus Overguard policy approval.
- Restore destination must satisfy trust, system-service eligibility, data-class, and vault-grant requirements.
- Restore runs must be idempotent by plan/session id and safe to resume after interruption.
- Backup manifests expose metadata and refs, not raw private data or secret contents.
- Multi-service snapshot sets must declare consistency windows and dependency graph refs.

## Event Surface

- `backup_restore.target_registered`
- `backup_restore.policy_versioned`
- `backup_restore.backup_started`
- `backup_restore.backup_completed`
- `backup_restore.backup_failed`
- `backup_restore.manifest_created`
- `backup_restore.snapshot_set_created`
- `backup_restore.integrity_verified`
- `backup_restore.integrity_failed`
- `backup_restore.restore_plan_created`
- `backup_restore.restore_started`
- `backup_restore.restore_completed`
- `backup_restore.restore_failed`
- `backup_restore.restore_drill_recorded`
- `backup_restore.retention_applied`
- `backup_restore.disaster_recovery_event_recorded`

Events must include target ids, manifest ids, snapshot set ids, policy versions, actor/service account id, trace id, idempotency key, verification refs, and Overwatch audit refs. Events must not contain raw secrets or raw protected data.

## Core Workflow

1. Register backup targets for critical stores and services.
2. Attach backup policies with schedule, retention, encryption, replication, and drill requirements.
3. Scheduled or manual backup runs call service-specific backup/export commands.
4. Each target returns a manifest with content refs, checkpoints, signatures, and dependency refs.
5. Related manifests are grouped into a snapshot set for a consistent recovery point.
6. Integrity verification checks checksums, signatures, dependencies, and sample restore ability.
7. Restore plans define destination environment, trust requirements, restore order, maintenance mode, and verification checks.
8. Restore sessions execute plans, verify restored state, and reconcile critical refs.
9. Restore drills record evidence before backbone services migrate away from founder hardware.
10. Retention applies tombstones and deletion evidence for expired backups.

## State Machine

Backup run states:

1. `scheduled`
2. `queued`
3. `running`
4. `manifest_created`
5. `verified`
6. `failed`
7. `expired`
8. `tombstoned`

Snapshot set states:

- `assembling`
- `consistent`
- `verification_required`
- `verified`
- `invalid`
- `expired`

Restore plan/session states:

1. `draft`
2. `policy_pending`
3. `approved`
4. `quiescing`
5. `restoring`
6. `verifying`
7. `reconciled`
8. `completed`
9. `failed`
10. `aborted`
11. `rolled_back`

State transitions must preserve evidence. A failed restore is a durable report, not a deleted attempt.

## Policy And Security

- Backup and restore operations are system-service actions and require stricter policy than ordinary workloads.
- Unknown public nodes must never host backbone restore targets.
- Restore destination must satisfy system-service workload class, trust, storage, backup, and vault-access requirements.
- Backup payloads must be encrypted or content-addressed according to data class; manifests must reference encryption/vault policy, not raw keys.
- Secret restore requires Overvault grant refs and must never expose raw secret material in events or manifests.
- Ledger restore must verify checkpoints and append/import metadata rather than editing historical ledger entries.
- Registry and queue restore must avoid split-brain by requiring maintenance/quiesce or explicit recovery mode.
- Break-glass restore requires signed operator action, policy refs, and incident/audit refs.
- Retention deletion must preserve tombstone evidence without leaking protected content.

## Metering And Accounting

- Backup, integrity verification, restore drills, and restore sessions consume storage, network, compute, and operator attention; they must emit usage-relevant events.
- System-service usage should be accounted through ORU/Seal Ledger as system-service usage entries where Phase 5 integration exists.
- Backup retention policies should make storage cost visible without encoding revenue or pricing assumptions.
- Restore drills should carry resource dimensions and trace ids so operational cost is auditable.
- Backup/restore never creates or modifies user/provider balances directly.

## Observability And Operations

Expose:

- Backup policy coverage by critical service.
- Last successful backup and verification time per target.
- Snapshot set consistency and verification status.
- Recovery point age and restore point age.
- Failed backup/restore counts by target and reason code.
- Checksum/signature mismatch counts.
- Restore drill pass/fail history.
- Retention backlog and tombstone count.
- Restore readiness for Phase 7 migration gates.
- Founder-hardware dependency status for backed-up services.

Operators need a drill dashboard that shows which services can be restored, when they were last tested, and what reconciliation evidence proved the restore.

## Failure Modes And Recovery

- Target backup command unavailable: mark backup failed with target reason code; do not create a fake manifest.
- Manifest checksum mismatch: mark integrity failed, quarantine manifest, and raise Overwatch incident.
- Partial snapshot set: keep set `verification_required` or `invalid`; do not use for restore until dependency gaps are resolved.
- Restore destination not trusted: deny before data transfer with `restore_destination_not_eligible`.
- Vault grant unavailable: block restore with `secret_restore_grant_missing`.
- Ledger checkpoint mismatch after restore: fail restore verification and require reconciliation/incident handling.
- Registry or queue restore creates conflict with live state: require maintenance/quiesce and split-brain protection before retry.
- Restore interrupted: resume by restore session id and completed target refs.
- Retention deletion fails: keep retention backlog and retry with tombstone evidence after success.

## Validation Plan

Required tests:

- Backup manifest includes target refs, content refs, checkpoints, checksums, signatures, dependency refs, policy refs, and audit refs.
- Backup can be restored into a clean trusted environment.
- Seal Ledger checkpoints verify after restore.
- Registry versions reconcile after restore.
- Queue high-water marks and dead-letter state reconcile after restore.
- Overstore content refs and Overbase structured snapshots verify integrity.
- Restore drill emits Overwatch evidence and a pass/fail report.
- Restore to an untrusted/public node is denied.
- Secret-bearing restore requires Overvault grant refs and never exposes raw secrets.
- Retention deletes expired backup refs only with tombstone evidence.

## Build Breakdown

1. Define backup target, backup policy, backup run, backup manifest, and snapshot set schemas.
2. Implement target registration and policy versioning.
3. Implement manual backup run flow for one low-risk target.
4. Add scheduled backups and manifest verification.
5. Add snapshot set grouping for multiple critical stores.
6. Add restore plan and restore session APIs.
7. Add restore drill execution/reporting for a clean environment.
8. Add Seal Ledger, Overregistry, Overqueue, Overbase, and Overstore reconciliation checks.
9. Add retention/tombstone flow.
10. Wire restore readiness into Phase 7 migration/founder-hardware-removal gates.

## Handoff And Downstream Use

Backup and Restore Service feeds:

- Failover and Recovery Coordinator with restore readiness, restore plans, and drill evidence.
- Release Strategy Service with pre-upgrade backup and rollback evidence.
- Migration Tooling with founder-hardware migration recovery points.
- Grid-Resident Service Packager with required backup/restore command contracts.
- Overwatch with operational evidence and incident history.
- Admin UI and CLI with authorized recovery status.

No downstream service should treat raw backup artifacts as the primary state API. Restores must go through approved plans and service-specific restore contracts.

## Open Design Questions

- Phase 7 uses store-specific consistency classes. Seal Ledger active stream heads/checkpoint ranges, Overqueue claim/ack/dead-letter state, active Overregistry service/package/node manifests, active Overguard policy bundles, Overwatch append/checkpoint heads, Overvault secret-version/grant/lease metadata, and any Overbase collection under migration or index rewrite require maintenance/quiesce or explicit recovery mode before a production-grade snapshot can become restore eligible. Online snapshots are allowed for immutable or append-segmented state such as verified Overstore object/chunk manifests, archived/checkpointed Overwatch segments, read-only registry replicas, rebuildable indexes, package artifacts, cache refs, and Overbase collections or event streams that declare export cursor, high-water mark, dependency refs, consistency window, and verifier refs.
- Founder hardware can leave the normal production path only after every migrated stateful backbone service has a successful restore drill from its current backup policy at least once in the preceding 30 days, plus one final end-to-end cutover rehearsal within seven days of the removal. Any change to a state schema, storage engine, backup policy, encryption/key policy, signer/verifier set, restore command, or migration boundary resets the affected service's drill gate until a new drill passes and Overwatch records the evidence.
- The first grid-resident backbone requires at least three independent restore-eligible backup replicas for each critical snapshot set: one trusted grid-resident hot or warm replica in the active recovery domain, one trusted replica in a separate failure domain, and one cold/offline or separately controlled trusted replica. During migration, founder hardware may remain as an emergency source replica, but it does not count as the sole non-founder backup; unknown public nodes and public low-sensitivity providers are never eligible for these replicas. Restore promotion should require fresh integrity verification from at least two independent replicas and incident review when the third is missing or stale.
- Tenant admins may see tenant-scoped coverage and readiness metadata: protected service/app scope, policy class, retention class, backup cadence, last successful backup, last verification, last drill status, RPO/RTO labels, restore request state, safe reason codes, redacted evidence refs, and tombstone/expiry summaries for their own data. System-operator-only metadata includes raw snapshot-set manifests, exact object/chunk/checksum placement, provider/node topology, ledger checkpoint ranges for system streams, queue high-water marks, registry diffs for backbone services, Overvault key/grant refs, secret classes, signer identities beyond safe role labels, break-glass details, internal disaster-recovery command steps, private incident evidence, and cross-tenant/system-service dependency graphs.
- Partial founder-hardware migration uses explicit dual-root restore plans. Each plan must declare the source of truth per target, founder-source checkpoint/cursor, grid-destination checkpoint/cursor, writer-fencing rule, route/lease freeze rule, import idempotency key, rollback path, and cutover condition. Founder-resident state can be read for export, verification, or emergency fallback, but after cutover it becomes read-only/tombstoned and cannot act as a silent co-writer. Service-specific restore rules apply: Seal Ledger uses `migration_import` and restore metadata refs, Overqueue drains or fences lanes before queue-state import, Overregistry and Overguard version active facts instead of overwriting them, Overwatch preserves old evidence chains with checkpoint links, and Overvault carries encrypted payload/key-policy refs without broadening access.
