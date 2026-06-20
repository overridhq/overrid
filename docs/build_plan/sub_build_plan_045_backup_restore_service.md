# SUB BUILD PLAN #45 - Backup and Restore Service

Attached SDS: [docs/sds/deployment_grid/backup_restore_service.md](../sds/deployment_grid/backup_restore_service.md)

## Purpose

This sub-build plan turns SDS #45 into an implementation sequence for Backup and Restore Service. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Backup and Restore Service is the recovery contract for Overrid's grid-resident backbone. It defines backup targets, policies, runs, manifests, snapshot sets, integrity checks, restore plans, restore sessions, restore drills, retention tombstones, disaster-recovery evidence, and founder-hardware removal gates without owning the primary state engines or exposing raw secrets/private data.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #45: Backup and Restore Service](../sds/deployment_grid/backup_restore_service.md) | Controls purpose, non-goals, actors, dependencies, owned responsibilities, data model, API/event surfaces, workflows, state machines, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Backup and Restore Service plan](../service_catalog/deployment_grid/backup_restore_service.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical master Phase 0 through Phase 13 order. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared contracts, deterministic fixtures, signed command envelopes, stable reason codes, idempotency keys, trace ids, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies identity, tenant, service-account, Overgate ingress, Overkey signing, Overregistry refs, Overwatch audit, and Overqueue state prerequisites. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard policy approval, workload/data class facts, signed policy refs, dry-run checks, and denial reason discipline for backup and restore commands. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Supplies Seal Ledger checkpoints, ORU/Seal Ledger system-service usage entries, accounting reconciliation refs, and cost visibility without pricing/revenue assumptions. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Controls the first build point: system-service backup/restore hooks, trusted restore destinations, restore drills, failover readiness, rollback readiness, and founder-hardware removal gates. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Supplies native Overbase snapshots, Overstore content-addressed backup artifacts, Overvault secret/vault grants, data classes, retention controls, and repair refs. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies package/deployment validation, release strategy, deployment planner, rollback evidence, and system-service package backup/restore command contracts. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies compliance boundaries, security reviews, reliability drills, incident response, migration tooling, audit exports, and public reporting hardening. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #45 first build work aligned to master Phase 7, with Phase 8 storage/vault integration and Phase 13 governance/reliability hardening. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, signed envelopes, Ed25519 signatures, BLAKE3/content hashes, native Overrid service boundaries, and no conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain, NFT, speculative-market, pricing, revenue, customer-count, or external-provider-as-core-boundary drift. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 7, 8, 9, and 13 | Attach SDS #45, freeze authority boundaries, preserve Phase 7 as the first build point, and record later storage/vault/deployment/governance gates. |
| 2 | Master Phases 0, 1, 7, and 8 | Define Rust contracts, canonical schemas, target catalogs, policy records, manifests, snapshot sets, events, reason codes, and deterministic fixtures. |
| 3 | Master Phases 1, 4, 7, and 8 | Implement target registration, policy versioning, coverage reporting, authorization, and service-owned snapshot adapter contracts. |
| 4 | Master Phases 7 and 8 | Implement backup runs, scheduled execution, manifest creation, snapshot-set grouping, integrity verification, and replica freshness checks. |
| 5 | Master Phases 4, 7, 8, and 9 | Implement restore plans, Overguard/Overvault/system-service approval, restore sessions, maintenance/quiesce controls, resume, rollback, and denial evidence. |
| 6 | Master Phases 5, 7, 8, and 13 | Implement store-specific reconciliation for Seal Ledger, Overregistry, Overqueue, Overbase, Overstore, Overwatch, Overguard, and Overvault. |
| 7 | Master Phases 7, 9, and 13 | Implement restore drills, three-replica restore eligibility, final cutover rehearsal, dual-root migration plans, and founder-hardware removal gates. |
| 8 | Master Phases 7, 8, and 13 | Implement retention, tombstones, disaster-recovery events, tenant/admin visibility rules, audit exports, and protected metadata redaction. |
| 9 | Master Phases 6, 7, 8, 9, 12, and 13 | Add operations dashboards, CLI/SDK/admin views, system-service usage events, failover/release/migration handoffs, and package-command contracts. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, stack guardrails, authority boundaries, queue state, Docdex retrieval, and implementation handoff readiness. |

## Tech Stack Guardrails

- Backup and Restore Service core is a Rust service/module using shared contract crates, Tokio for bounded backup/restore workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Backup targets, policies, runs, manifests, snapshot sets, verification reports, restore plans, restore sessions, drill reports, retention tombstones, disaster-recovery events, API objects, events, fixtures, redaction profiles, and audit exports use canonical JSON plus JSON Schema for docs-facing contracts and tests. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating endpoints require signed service envelopes, authorized operator or system-service authority, tenant/system scope, trace id, idempotency key, schema version, policy refs, stable reason codes, and append-only Overwatch events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for manifests, chunk refs, snapshot-set members, checkpoint refs, verification reports, replay bundles, tombstone proofs, and deterministic fixture comparison.
- Backup payload placement must use native Overrid boundaries: Overbase for structured snapshots, Overstore for content-addressed objects and backup artifacts, and Overvault for encrypted private refs, key policy metadata, and secret restore grants.
- Planning and implementation must not make PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Ceph, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFTs, speculative assets, revenue projections, customer-count assumptions, raw secret storage, direct payment execution, or untrusted public nodes the platform boundary.

## Phase 1: SDS Attachment, Recovery Authority, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #45.**
  - Design: Link this document from the Backup and Restore Service SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/deployment_grid/backup_restore_service.md`, `docs/service_catalog/deployment_grid/backup_restore_service.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #45 returns both the Backup and Restore Service SDS and this sub-build plan.

- **1.2 Freeze Backup and Restore Service authority boundaries.**
  - Design: Record that the service owns target catalogs, backup policies, backup runs, manifests, snapshot sets, integrity reports, restore plans, restore sessions, drills, retention tombstones, disaster-recovery events, readiness reports, and recovery audit exports.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms the service does not own primary state engines, live traffic failover, system-service package builds, release rollout policy, ledger history mutation, queue history mutation, registry fact mutation, raw secret material, or external storage/provider boundaries.

- **1.3 Preserve master Phase 7 as the first build point.**
  - Design: Keep first implementation in Phase 7 after identity, policy, audit, accounting, private execution, and product integration prerequisites exist.
  - Output: Phase-gate note that Phase 7 builds backup/restore for grid-resident backbone migration, Phase 8 adds native storage/vault depth, Phase 9 consumes package/release command contracts, and Phase 13 hardens governance, compliance, and reliability.
  - Validation: Review proves this plan does not move native Overbase/Overstore/Overvault full ownership into Phase 7 and does not delay Phase 7 restore drills behind later native-app work.

- **1.4 Carry forward resolved SDS #45 decisions.**
  - Design: Preserve store-specific consistency classes, recent restore-drill gates, final cutover rehearsal, three independent trusted restore-eligible replicas, redacted tenant/admin metadata, and dual-root founder migration plans.
  - Output: Resolved-decision checklist tied to contract, API, operations, and validation reviews.
  - Validation: Review rejects one-size-fits-all online snapshots, founder-hardware removal without fresh drills, single-replica critical backups, raw snapshot topology exposure to tenants, silent co-writer founder state, and restore into unknown public nodes.

- **1.5 Define upstream and downstream dependency boundaries.**
  - Design: Create a dependency matrix for Overbase, Overstore, Overvault, Seal Ledger, Overregistry, Overqueue, Overwatch, Overguard, Overpass, System-Service Workload Class, Grid-Resident Service Packager, Failover and Recovery Coordinator, Release Strategy Service, Migration Tooling, Admin UI, CLI, and SDK.
  - Output: Boundary matrix listing owner, backup target type, consistency mode, export/restore command contract, policy gate, secret/vault refs, reconciliation checks, audit refs, and downstream consumer.
  - Validation: Review confirms every handoff uses explicit APIs, versioned refs, signed evidence, reason codes, trace ids, policy refs, and Overwatch events rather than privileged shared tables or raw artifact reads.

## Phase 2: Rust Contracts, Schemas, Target Catalogs, And Fixtures

### Work Items

- **2.1 Create the Backup and Restore Rust contract module.**
  - Design: Add contract types for backup targets, backup policies, backup runs, manifests, snapshot sets, integrity reports, restore plans, restore sessions, restore drills, retention tombstones, disaster-recovery events, API errors, events, redaction profiles, and audit exports.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, lifecycle enums, target-kind enums, consistency-class enums, reason-code enums, fixture hooks, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms contracts stay separate from Overbase, Overstore, Overvault, Seal Ledger, Overregistry, Overqueue, Failover and Recovery Coordinator, Release Strategy Service, and Migration Tooling internals.

- **2.2 Define canonical backup target and policy schemas.**
  - Design: Model `backup_target` and `backup_policy` with service/store id, data classes, snapshot method, consistency mode, quiesce requirement, encryption policy, locality, replication count, retention class, drill cadence, integrity-check cadence, and operator approval requirements.
  - Output: JSON Schema files, valid examples, invalid examples, generated validators, and docs-facing examples.
  - Validation: Schema tests reject missing owner service, data class, consistency mode, encryption policy, retention class, trust requirement, drill cadence, policy refs, audit refs, trace id, idempotency key, or schema version.

- **2.3 Define backup run, manifest, and snapshot-set schemas.**
  - Design: Model `backup_run`, `backup_manifest`, and `snapshot_set` with trigger type, target refs, content refs, chunk refs, ledger checkpoints, registry versions, queue high-water marks, policy versions, checksums, signatures, dependency graph refs, consistency windows, recovery-point labels, and restore-order hints.
  - Output: Run/manifest/snapshot schemas, state-machine fixtures, checksum/signature fixtures, dependency graph examples, and recovery-point examples.
  - Validation: Tests reject fake manifests, missing dependency refs, unsigned manifests, checksum gaps, missing consistency windows, cross-scope snapshot members, and snapshot sets assembled from incompatible policy versions.

- **2.4 Define restore, verification, retention, and disaster-recovery schemas.**
  - Design: Model `integrity_verification_report`, `restore_plan`, `restore_session`, `restore_drill_report`, `retention_tombstone`, and `disaster_recovery_event` with source snapshot set, destination trust requirements, restore order, quiesce/maintenance requirements, vault grant refs, expected checks, verification refs, rollback refs, incident refs, deletion proof refs, and follow-up refs.
  - Output: Schema files, redacted examples, invalid examples, reason-code catalog, state-transition examples, and report fixtures.
  - Validation: Tests reject restore plans without destination trust, vault grant refs for secret-bearing targets, expected verification checks, policy approval refs, rollback path, audit refs, and idempotency keys.

- **2.5 Create deterministic backup/restore fixtures.**
  - Design: Build fixtures for one low-risk target, a multi-service snapshot set, failed checksum verification, trusted restore, denied untrusted restore, missing Overvault grant, ledger checkpoint mismatch, queue high-water mismatch, retention tombstone, and disaster-recovery event.
  - Output: Fixture directory, expected manifests, expected reports, expected events, reason codes, hash values, signatures, redacted views, and replay examples.
  - Validation: Fixture tests produce stable ids, BLAKE3 hashes, signatures, state transitions, denial reasons, audit refs, and idempotency outcomes across repeated runs.

## Phase 3: Target Registration, Policy Versioning, And Coverage Readiness

### Work Items

- **3.1 Implement backup target registration APIs.**
  - Design: Add `POST /backup-targets` with create/update semantics, idempotency, target-owner validation, data-class declaration, consistency class, quiesce mode, export command ref, restore command ref, and Overwatch audit.
  - Output: Target registration handler, contract tests, target registry storage abstraction, reason codes, and audit events.
  - Validation: Tests prove unauthorized targets, duplicate ids with different payloads, missing owner refs, missing command refs, unsupported data classes, and public-node restore eligibility are rejected before target activation.

- **3.2 Implement backup policy versioning.**
  - Design: Add `POST /backup-policies` with immutable policy versions, target refs, schedule, retention, replication count, locality requirements, encryption refs, integrity-check cadence, drill cadence, and approval requirements.
  - Output: Policy version model, active-policy selector, policy history query, coverage reason codes, and `backup_restore.policy_versioned` events.
  - Validation: Tests prove policy updates create new versions, old manifests keep original policy refs, missing replication/drill requirements block critical targets, and policy changes reset affected restore-drill gates.

- **3.3 Implement service-owned adapter contracts.**
  - Design: Define export/restore adapter interfaces for Seal Ledger checkpoints, Overregistry versions, Overqueue high-water marks, Overbase snapshots, Overstore content refs, Overwatch evidence segments, Overguard policy bundles, and Overvault secret metadata refs.
  - Output: Adapter trait contracts, adapter capability records, compatibility fixtures, and missing-adapter reason codes.
  - Validation: Tests prove Backup and Restore Service asks owner services for snapshots/restores and never reads raw primary state or raw secret contents directly.

- **3.4 Implement coverage and readiness reporting.**
  - Design: Track which critical services have active targets, active policies, successful recent backups, recent integrity checks, recent restore drills, replica health, and unresolved blockers.
  - Output: Coverage report API, readiness projection, reason-code catalog, Overwatch metrics/events, and operator-facing summary fields.
  - Validation: Tests prove missing policy, stale backup, stale verification, stale drill, insufficient replicas, blocked vault grant, and failed reconciliation produce distinct readiness reasons.

- **3.5 Gate target and policy changes with signed authority.**
  - Design: Require signed operator or authorized system-service envelopes, Overguard approval where applicable, stable idempotency, trace ids, and audit refs for target and policy mutation.
  - Output: Authorization middleware, policy fact bundle, denial events, and break-glass record shape.
  - Validation: Tests reject unsigned, wrong-scope, stale-key, wrong-role, missing-policy, replayed, and break-glass-without-incident target/policy changes.

## Phase 4: Backup Runs, Manifests, Snapshot Sets, And Integrity Verification

### Work Items

- **4.1 Implement manual backup run flow for one low-risk target.**
  - Design: Start with a non-critical internal observability or read-only registry target so backup orchestration, manifest creation, idempotency, audit, and failure handling can be proven before critical state.
  - Output: `POST /backup-runs`, run worker, run state machine, target adapter invocation, manifest stub, failure reason codes, and Overwatch events.
  - Validation: Tests prove successful runs create manifests, failed runs do not create fake manifests, duplicate manual requests return original run refs, and interrupted runs expose resumable state.

- **4.2 Add scheduled backup execution.**
  - Design: Implement schedule evaluation from active policy versions with jitter, concurrency limits, target backpressure, maintenance windows, and bounded retry behavior.
  - Output: Scheduler worker, queue records, retry model, stale schedule detection, and run history queries.
  - Validation: Tests prove schedules trigger correctly, do not overlap incompatible targets, respect quiesce requirements, handle retries without duplicate manifests, and record missed-run reasons.

- **4.3 Implement backup manifest creation and signing.**
  - Design: Build manifest records from owner-service snapshot outputs with content refs, chunk refs, checkpoint refs, policy refs, dependency refs, BLAKE3 hashes, Ed25519 signatures, actor/service-account refs, and Overwatch audit refs.
  - Output: Manifest builder, signer integration, hash verifier, manifest query API, and redaction profile.
  - Validation: Tests prove manifests expose metadata and refs only, reject raw secrets/private payloads, verify signatures/hashes, and remain tied to the policy and schema versions active when the run executed.

- **4.4 Implement multi-service snapshot-set grouping.**
  - Design: Assemble compatible manifests into a snapshot set with consistency window, dependency graph, restore-order hints, recovery-point objective labels, and partial-set handling.
  - Output: `POST /snapshot-sets`, dependency graph validator, consistency classifier, set state machine, and grouping fixtures.
  - Validation: Tests reject incompatible consistency windows, missing critical dependencies, mismatched policy versions, stale manifests, cross-scope members, and partial sets being marked restore eligible.

- **4.5 Implement integrity verification reports.**
  - Design: Add verification for checksums, signatures, dependency refs, policy refs, replica availability, sample restore eligibility, and mismatch reporting.
  - Output: `POST /integrity-verifications`, verification worker, report model, quarantine behavior, mismatch reason codes, and `backup_restore.integrity_verified` or `backup_restore.integrity_failed` events.
  - Validation: Tests prove checksum mismatch quarantines a manifest, signature mismatch blocks restore eligibility, missing dependency refs keep a set invalid, and failed verification creates durable evidence rather than deleting attempts.

## Phase 5: Restore Plans, Authorization, Sessions, And Resume Safety

### Work Items

- **5.1 Implement restore plan creation.**
  - Design: Add `POST /restore-plans` for approved snapshot sets with destination environment, trust requirements, system-service eligibility, restore order, quiesce/maintenance mode, recovery mode, vault grant refs, expected checks, rollback path, and incident refs.
  - Output: Restore plan API, plan state machine, destination validator, rollback contract, and plan examples.
  - Validation: Tests reject restore plans against unverified snapshot sets, untrusted destinations, missing quiesce rules, missing vault grants, missing rollback paths, and plan scopes that widen access.

- **5.2 Gate restores through Overguard and Overvault refs.**
  - Design: Require Overguard policy approval for restore commands and Overvault grant refs for secret-bearing targets without exposing key material, secret values, or raw private payloads.
  - Output: Policy fact bundle, vault grant ref validator, denial reason catalog, and restore authorization events.
  - Validation: Tests prove secret-bearing restores block when grant refs are missing, deny-by-default policy blocks unsafe destinations, and events contain refs and reason codes rather than raw secrets.

- **5.3 Implement restore session execution.**
  - Design: Add `POST /restore-sessions` with command id, actor/service account, session state, restored target refs, step refs, verification refs, rollback refs, incident refs, and idempotent resume behavior.
  - Output: Session worker, step executor, session report API, resumable checkpoint model, and abort/rollback commands.
  - Validation: Tests prove sessions resume by session id, completed target refs are not repeated unsafely, failed restores become durable reports, and rollback/abort preserves evidence.

- **5.4 Implement maintenance, quiesce, and recovery-mode controls.**
  - Design: Coordinate restore sessions with owner-service maintenance/quiesce or explicit recovery modes for Seal Ledger stream heads, Overqueue claim/ack/dead-letter state, Overregistry active facts, Overguard policies, Overwatch append heads, Overvault secret metadata, and mutable Overbase collections.
  - Output: Quiesce contract, recovery-mode contract, owner-service command refs, split-brain prevention checks, and route/lease freeze hooks.
  - Validation: Tests prove live-writer conflicts are denied, split-brain risks are blocked before transfer, and online snapshots are allowed only for immutable or append-segmented targets that declare export cursor and verifier refs.

- **5.5 Implement restore report and denial evidence.**
  - Design: Record plan refs, session refs, destination checks, restored target refs, mismatch refs, denial reasons, incident refs, Overwatch events, and redacted report views.
  - Output: `GET /restore-sessions/{id}/report`, report schema, redaction logic, operator view, and tenant-safe view.
  - Validation: Tests prove raw snapshot manifests, exact placement, secret refs, signer identities beyond safe labels, private incidents, and cross-tenant dependency graphs are hidden from tenant-scoped views.

## Phase 6: Store-Specific Reconciliation And Recovery Checks

### Work Items

- **6.1 Implement Seal Ledger checkpoint reconciliation.**
  - Design: Verify ledger stream heads, checkpoint ranges, entry fingerprints, migration_import metadata, correction refs, and append-only semantics after restore.
  - Output: Seal Ledger reconciliation adapter, checkpoint verifier, mismatch reason codes, and incident refs.
  - Validation: Tests prove checkpoint mismatch fails restore verification, historical ledger entries are never edited, and reconciliation creates evidence refs for follow-up correction-by-new-entry workflows.

- **6.2 Implement Overregistry and Overguard reconciliation.**
  - Design: Verify active service/package/node/provider facts, version refs, policy bundle refs, signer refs, and active/inactive transitions without overwriting accepted history.
  - Output: Registry/policy reconciliation adapters, version comparison reports, active-fact diff fixtures, and approval gates.
  - Validation: Tests prove registry and policy restore uses versioned facts, rejects stale active facts, and records diffs without mutating prior accepted facts.

- **6.3 Implement Overqueue reconciliation.**
  - Design: Verify queue high-water marks, pending/claimed/acked/dead-letter state, retry windows, worker handoff safety, lane drains, and split-brain protection after restore.
  - Output: Queue reconciliation adapter, lane drain/fence contract, high-water verifier, and dead-letter report.
  - Validation: Tests prove claim/ack conflicts are blocked, dead-letter state survives restore, retry state is not duplicated, and queue import requires maintenance or explicit recovery mode.

- **6.4 Implement Overbase, Overstore, and Overwatch verification.**
  - Design: Verify structured snapshots, event streams, index cursors, object/chunk refs, BLAKE3 hashes, repair refs, archived evidence segments, append/checkpoint heads, and audit chains.
  - Output: Data/storage/evidence verification adapters, sample restore checks, repair/quarantine refs, and report sections.
  - Validation: Tests prove Overbase snapshots and Overstore content refs verify integrity, rebuildable indexes are marked as rebuildable not authoritative, and Overwatch evidence chains remain linked after restore.

- **6.5 Implement Overvault metadata and grant reconciliation.**
  - Design: Verify secret-version refs, encrypted payload refs, key-policy refs, grant refs, mount/lease metadata, revocation state, and access boundaries without exposing raw key material.
  - Output: Vault metadata adapter, grant verifier, secret-restore policy report, and redacted failure reasons.
  - Validation: Tests prove raw secrets are never emitted, revoked grants block restore, missing grant refs fail before data transfer, and access scope never broadens during restore.

## Phase 7: Restore Drills, Replica Eligibility, And Founder-Hardware Exit Gates

### Work Items

- **7.1 Implement restore drill execution and reporting.**
  - Design: Execute restore plans into clean trusted environments, run reconciliation checks, record findings, and publish pass/fail drill reports with Overwatch evidence.
  - Output: `POST /restore-drills`, drill runner, clean-environment marker, drill report schema, and readiness updates.
  - Validation: Tests prove drills require trusted environments, emit pass/fail reports, attach reconciliation evidence, and update readiness only after all blocking checks pass.

- **7.2 Enforce recent-drill gates for founder-hardware removal.**
  - Design: Require each migrated stateful backbone service to have a successful drill from its current policy in the preceding 30 days plus one final end-to-end cutover rehearsal within seven days of removal.
  - Output: Founder-hardware exit gate API, drill freshness calculator, schema/policy-change reset logic, and gate report.
  - Validation: Tests prove stale drills, schema changes, storage engine changes, backup policy changes, key policy changes, signer changes, restore command changes, and migration-boundary changes reset affected gates.

- **7.3 Enforce three independent trusted restore-eligible replicas.**
  - Design: Require one trusted grid-resident hot/warm replica in the active recovery domain, one trusted replica in a separate failure domain, and one cold/offline or separately controlled trusted replica for each critical snapshot set.
  - Output: Replica eligibility model, failure-domain validator, integrity freshness report, and missing/stale replica incident behavior.
  - Validation: Tests prove founder hardware cannot be the sole non-founder backup, unknown public nodes are never eligible, two fresh replicas are required for promotion, and missing third replica requires incident review.

- **7.4 Implement dual-root founder migration plans.**
  - Design: Model source of truth per target, founder-source checkpoint/cursor, grid-destination checkpoint/cursor, writer-fencing rule, route/lease freeze rule, import idempotency key, rollback path, and cutover condition.
  - Output: Dual-root restore plan extension, migration import refs, route/lease freeze hooks, founder read-only/tombstone state, and cutover report.
  - Validation: Tests prove founder state can be read for export/verification/emergency fallback but cannot become a silent co-writer after cutover.

- **7.5 Feed failover, release, and migration owners.**
  - Design: Provide restore readiness, drill evidence, replica health, rollback readiness, and cutover blockers to Failover and Recovery Coordinator, Release Strategy Service, Grid-Resident Service Packager, and Migration Tooling.
  - Output: Handoff APIs/events, report schemas, owner-service fixtures, and gating examples.
  - Validation: Integration tests prove failover cannot promote a target with failed restore readiness, release rollout can require pre-upgrade backup evidence, and migration tooling consumes dual-root evidence without bypassing restore approval.

## Phase 8: Retention, Tombstones, Disaster Recovery, And Redacted Visibility

### Work Items

- **8.1 Implement retention application and tombstone evidence.**
  - Design: Apply backup retention policies to expired backup refs while preserving tombstone records, deletion proof refs, policy version refs, actor/service account refs, and audit refs.
  - Output: `POST /retention/apply`, retention worker, tombstone schema, backlog report, and deletion proof fixtures.
  - Validation: Tests prove retention never deletes current restore-eligible refs, failed deletion remains in backlog, tombstones do not leak protected content, and expired refs remain auditable by metadata.

- **8.2 Implement disaster-recovery event records.**
  - Design: Record disaster/incident id, affected targets, selected restore plan, actions taken, verification results, final state, incident refs, and follow-up refs.
  - Output: `GET /disaster-recovery/events/{id}`, event creation path, report view, and Overwatch linkage.
  - Validation: Tests prove disaster-recovery events cite restore sessions and verification reports, preserve failed attempts, and hide raw private evidence in redacted views.

- **8.3 Implement tenant-scoped readiness metadata.**
  - Design: Expose tenant-safe coverage and readiness fields: protected scope, policy class, retention class, cadence, last successful backup, last verification, last drill status, RPO/RTO labels, restore request state, safe reason codes, redacted evidence refs, and tombstone summaries.
  - Output: Tenant-safe API/view model, redaction rules, field allowlist, and denial reasons.
  - Validation: Tests prove raw snapshot manifests, exact chunk placement, provider/node topology, system ledger checkpoints, queue high-water marks, Overvault refs, signer identities, break-glass details, private incidents, and cross-tenant graphs are not exposed.

- **8.4 Implement operator-only recovery metadata.**
  - Design: Provide authorized operators with raw snapshot-set manifests, dependency graphs, restore command steps, topology refs, break-glass refs, and private incident evidence only under strict role/policy/audit gates.
  - Output: Operator view model, policy facts, audit events, and break-glass constraints.
  - Validation: Tests prove operator metadata requires signed authority, incident refs when break-glass is used, Overwatch audit, and least-privilege filters by role and scope.

- **8.5 Implement retention and disaster-recovery audit exports.**
  - Design: Generate audit export bundles for retention, tombstones, drills, restore sessions, disaster-recovery events, and follow-up work without exposing protected payloads.
  - Output: Export schema, redaction profile, BLAKE3 hash manifest, signature, and report examples.
  - Validation: Tests prove export bundles are deterministic, signed, redact protected fields, preserve evidence refs, and can be verified independently.

## Phase 9: Operations, Interfaces, Metering, And Downstream Handoffs

### Work Items

- **9.1 Implement operator readiness dashboard data.**
  - Design: Expose backup policy coverage, last successful backup, last verification, snapshot consistency, recovery point age, restore point age, failed counts, mismatch counts, drill history, retention backlog, and founder dependency status.
  - Output: Dashboard API projection, metrics/events, status cards, filters, and alert thresholds for Admin UI consumers.
  - Validation: Tests prove dashboard fields derive from service-owned records and do not require raw primary-state or raw backup artifact access.

- **9.2 Implement CLI and SDK recovery commands.**
  - Design: Add generated CLI/SDK commands for target registration, policy versioning, manual runs, manifest reads, verification, restore plans, restore sessions, restore drills, retention apply, and disaster-recovery event reads.
  - Output: CLI command contract, SDK bindings, stable JSON output, error examples, and idempotency behavior.
  - Validation: Contract tests prove commands pass signed envelopes, trace ids, idempotency keys, schema versions, policy refs, and stable reason codes through generated contracts.

- **9.3 Emit system-service usage and accounting refs.**
  - Design: Emit usage-relevant events for backup runs, integrity verification, restore drills, restore sessions, storage retention, transfer work, and operator attention without encoding pricing or revenue assumptions.
  - Output: Usage event schema, ORU/Seal Ledger handoff refs, resource dimensions, trace ids, and audit refs.
  - Validation: Tests prove backup/restore usage creates system-service usage entries where Phase 5 integration exists and never creates/modifies user/provider balances directly.

- **9.4 Integrate release, package, and failover workflows.**
  - Design: Provide pre-upgrade backup evidence, rollback evidence, package backup/restore command requirements, restore readiness, and drill evidence to Release Strategy Service, Grid-Resident Service Packager, Package Validator, and Failover and Recovery Coordinator.
  - Output: Owner-service events, readiness APIs, package contract checks, release gate fixtures, and failover gate fixtures.
  - Validation: Integration tests prove releases can block on missing backup evidence, package validation catches missing backup/restore commands, and failover promotion requires restore readiness.

- **9.5 Prepare native app and governance consumers.**
  - Design: Shape read-only redacted views for Admin UI, Wallet and Usage Center where system-service usage matters, stewardship reporting, incident response, compliance boundaries, and security review tracking.
  - Output: Consumer view contracts, redaction profiles, report refs, and follow-up backlog entries.
  - Validation: Review confirms client/native/governance consumers receive safe metadata and evidence refs only through owner APIs, not privileged reads into backup manifests or storage artifacts.

## Phase 10: Validation, Security Review, Queue Closure, And Handoff Readiness

### Work Items

- **10.1 Validate plan structure and local links.**
  - Design: Check title prefix, attached SDS link, phases 1 through 10, five work items per phase, Design/Output/Validation structure, final newline, tabs, and local Markdown links.
  - Output: Validation command output and progress evidence.
  - Validation: Focused script passes for this file and every changed cross-document link.

- **10.2 Validate tech-stack guardrails.**
  - Design: Scan this plan and changed docs for conventional database, queue, object-store, vault/KMS, Kubernetes-first, blockchain/NFT, speculative-market, pricing, revenue, customer-count, raw-secret-storage, public-node-restore, direct-primary-state-read, and external-provider-as-core-boundary drift.
  - Output: Guardrail scan output with only allowed negative-control or explicit non-choice references.
  - Validation: Review confirms the plan keeps Rust-first/native Overrid service boundaries and does not turn comparison tools or local stubs into product boundaries.

- **10.3 Validate SDS, service catalog, master plan, and crosswalk alignment.**
  - Design: Confirm this plan is linked from the SDS source table, service catalog sub-build-plan section, master per-SDS table, and service-catalog alignment index.
  - Output: Updated backlinks and index rows for SDS #45.
  - Validation: Local link checks pass and Docdex search returns the SDS, service plan, crosswalk row, and this sub-build plan in aligned results.

- **10.4 Validate queue state and progress evidence.**
  - Design: Mark `045-build-plan` complete in the Codex55 queue, update `.codex55_sds_queue/progress.md`, append validation evidence to `docs/build_plan/progress.md`, and preserve the next incomplete build-plan task as SDS #46.
  - Output: Updated queue JSON, queue progress summary, and build-plan progress notes.
  - Validation: JSON validation passes; queue validation confirms `045-build-plan` is complete, no task is running, and `046-build-plan` is the next incomplete build-plan task.

- **10.5 Validate implementation handoff readiness.**
  - Design: Confirm the 10-phase plan gives builders contract, API, adapter, worker, reconciliation, drill, retention, operations, and governance work in dependency order without changing master Phase 0 through Phase 13 order.
  - Output: Handoff checklist, Docdex index refresh, search evidence, DAG export reference, and test-runner status.
  - Validation: Docdex targeted index refresh succeeds, Docdex retrieval finds the plan, `git diff --check` passes, and any `docdexd run-tests` blocker is recorded explicitly.
