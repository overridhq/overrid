# SUB BUILD PLAN #27 - Overbase

Attached SDS: [docs/sds/data_storage_namespace/overbase.md](../sds/data_storage_namespace/overbase.md)

## Purpose

This sub-build plan turns SDS #27 into an implementation sequence for Overbase. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overbase is the governed structured-state substrate for documents, key-value records, append-only event streams, secondary indexes, vector indexes, schemas, consistency policies, replication metadata, backup snapshots, restore plans, and migration records. It is not object storage, a private vault, namespace ownership, a billing service, or a generic database product boundary. Large bytes remain in Overstore, secret-bearing and private encrypted records remain in Overvault, names and route ownership remain in the Universal Namespace Service, and usage/accounting settlement remains outside Overbase.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #27: Overbase](../sds/data_storage_namespace/overbase.md) | Controls Overbase purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machine, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overbase service plan](../service_catalog/data_storage_namespace/overbase.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, canonical JSON/JSON Schema discipline, signed envelopes, idempotency, trace ids, stable reason codes, local fixtures, deterministic reset/seed, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, registry, request-admission, idempotency, and audit primitives. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard, policy dry-run, workload/data-class policy, trust evidence, dispute evidence, and access-control inputs used by Overbase. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes Overbase raw usage facts through Overmeter while keeping rollups, ledger mutation, billing, pricing, payout, and settlement outside Overbase. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected backbone readiness, system-service placement expectations, backup/restore service patterns, failover, recovery, migration tooling, and grid-resident shard operation prerequisites. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Controls Overbase's first build point as the structured-state member of the native data/storage/namespace platform. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies application-intent provisioning hooks, package validation, deployment manifests, schema/index declarations, and rollback health constraints that Overbase consumes but does not own. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes Overbase APIs for workspace, directory, search, messaging, wallet, social, maps, personal AI, and central AI stewardship workflows. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, compliance, threat-model, incident, PIP, migration, audit-export, and governance hardening for Overbase contracts. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #27 first build work aligned to master Phase 8, with prerequisites from earlier phases and later provisioning, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, native Overrid service boundaries, and no conventional database product boundary. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 4, 5, 7, 8, 12, and 13 | Attach SDS #27, freeze structured-state boundaries, preserve Phase 8 as first build point, and record prerequisite and later hardening gates. |
| 2 | Master Phases 0, 1, and 8 | Build Rust contracts, schemas, storage abstraction, operation envelopes, fixtures, and local shard profiles before state side effects. |
| 3 | Master Phases 1, 4, and 8 | Create database scopes, collection lifecycle, schema-version registration, and policy admission around tenant/app/data-class ownership. |
| 4 | Master Phase 8 | Implement document and key-value records with validation, etags/conflict tokens, idempotency, redaction refs, tombstones, and bounded values. |
| 5 | Master Phases 4, 8, and 12 | Implement bounded queries, redaction, secondary indexes, stable pagination, query denial, and native-app read compatibility. |
| 6 | Master Phases 1, 3, 5, 8, and 13 | Add append-only event streams, queued operation jobs, usage/audit emission, and operation state that does not become an external event backbone. |
| 7 | Master Phases 6, 8, 12, and 13 | Add vector indexes and authorized RAG metadata for product adapters and native apps while preserving tenant/app/access-scope isolation. |
| 8 | Master Phases 7, 8, and 13 | Add consistency policy, replication metadata, backup snapshots, restore dry-run/apply, migrations, rollback, and grid-resident readiness. |
| 9 | Master Phases 6, 8, 9, and 12 | Connect Overpack provisioning, product adapters, native apps, SDK/CLI/admin reads, and operations dashboards without bypassing Overbase APIs. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, tech-stack alignment, queue state, progress evidence, and implementation handoff gates. |

## Tech Stack Guardrails

- Overbase core is a Rust service/module using shared contract types, Tokio for async work, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Overbase contracts use canonical JSON plus JSON Schema for docs-facing examples, fixtures, commands, API payloads, events, operation records, and replay reports. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating calls use signed envelopes, tenant/app context, actor or service-account identity, idempotency keys, trace ids, schema versions, stable reason codes, policy refs, and append-only audit events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for payload refs, object refs, snapshots, schema bundles, migration bundles, index build evidence, and replay evidence.
- Overbase owns its public structured-state boundary even when it uses an internal embedded shard engine. The v0 local shard profile may benchmark a redb-backed pure-Rust candidate; RocksDB-like engines remain internal candidates only.
- PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, S3, MinIO, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFT, pricing, revenue, customer-count, or external-payment assumptions must not become Overbase's product boundary.
- Overbase must not store raw large object bytes, raw secrets, private vault payloads, namespace ownership, asset-right records, ORU balances, Seal Ledger entries, or billing state.

## Phase 1: SDS Attachment, Structured-State Scope, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #27.**
  - Design: Link this document from the numbered Overbase SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/data_storage_namespace/overbase.md`, `docs/service_catalog/data_storage_namespace/overbase.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #27 returns both the Overbase SDS and this sub-build plan.

- **1.2 Freeze Overbase as the structured-state authority.**
  - Design: Record that Overbase owns databases, collections, schemas, records, queries, event streams, secondary indexes, vector indexes, consistency policy, replication metadata, backups, restores, and migrations.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overbase does not own object bytes, raw secrets, namespace ownership, route binding, accounting settlement, or direct dependency storage.

- **1.3 Preserve master Phase 8 as the first build point.**
  - Design: Keep first implementation in master Phase 8 because Overbase depends on signed identity/tenant/request/audit rails, policy controls, usage emission, and protected grid/storage readiness.
  - Output: Phase-gate note that Phases 0, 1, 4, 5, and 7 are prerequisites and Phase 8 proves native structured state before broader native app consumption.
  - Validation: Review proves this plan does not move Overbase into earlier master phases and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #27 decisions for single-record authoritative writes, idempotency keys, etags/conflict tokens, read-your-writes behavior, monotonic snapshot tokens, append-only stream partition ordering, access-scope-isolated vector indexes, queued jobs for long operations, redb-backed local shard evaluation, and migration approval boundaries.
  - Output: Resolved-decision checklist tied to SDS #27 open-question answers.
  - Validation: Review rejects global serializable v0 requirements, shared private vectors, synchronous bulk jobs, conventional database boundaries, or unapproved destructive/shared migrations.

- **1.5 Define service authority boundaries.**
  - Design: Create a boundary matrix for Overtenant, Overpass, Overkey, Overgate, Overguard, Overwatch, Overmeter, Overstore, Overvault, Overpack, Universal Namespace Service, Overmesh, Overasset, native apps, adapters, SDK, CLI, admin UI, and governance services.
  - Output: Boundary matrix listing consumed refs, emitted refs, denial behavior, queued-operation owner, redaction profile, and replay evidence for each dependency.
  - Validation: Review confirms every dependency has an owning service and Overbase receives private facts through explicit APIs/refs rather than direct storage access.

## Phase 2: Rust Contracts, Schemas, Storage Abstraction, And Fixtures

### Work Items

- **2.1 Create the Overbase Rust contract module.**
  - Design: Add contract types for database scopes, collection schemas, document records, key-value records, event streams, event records, secondary indexes, vector indexes, consistency policies, replication groups, backup snapshots, restore plans, migration records, operation jobs, errors, and state machines.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, reason-code types, API error types, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms Overbase contracts remain separate from Overstore, Overvault, namespace, accounting, and policy engine internals.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for database scope, collection schema, document upsert, point read, bounded query, key-value write/read, stream append/read, index build, vector index rebuild, backup snapshot, restore plan, migration dry-run/apply, and queued operation status.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing tenant, app, actor/service-account, trace, idempotency, data-class, retention, schema-version, collection, policy-ref, and audit-ref fields where required.

- **2.3 Define the internal storage abstraction.**
  - Design: Model shard engine interfaces for append-only operation records, bounded record reads/writes, value refs, index metadata, snapshot/export hooks, deterministic reset/seed, integrity checks, and migration hooks without exposing the engine as the product API.
  - Output: Storage abstraction traits, local shard profile, redb candidate benchmark hooks, fixture engine, error mapping, and engine compatibility notes.
  - Validation: Boundary tests prove public API contracts do not expose redb/RocksDB-like implementation details and can swap internal engines behind Overbase refs.

- **2.4 Build operation envelopes and replay evidence.**
  - Design: Define signed operation records for schema changes, writes, queries, stream appends, index builds, vector rebuilds, backups, restores, migrations, replication repairs, compaction, and tombstone cleanup.
  - Output: Operation envelope schema, BLAKE3 hash refs, audit refs, idempotency model, replay bundle fields, and redaction profiles.
  - Validation: Replay tests prove every mutating operation has source refs, policy refs, actor/service-account refs, stable reason codes, and append-only audit refs.

- **2.5 Create deterministic fixtures and harness scenarios.**
  - Design: Build fixtures for collection creation, document write, KV write, bounded query, event append, index build, vector search denial, backup dry run, restore conflict, migration approval, replication degradation, cross-tenant denial, and storage-engine recovery.
  - Output: Fixture directory, expected API responses, operation records, event payloads, usage facts, audit refs, and replay hashes.
  - Validation: Fixture tests produce stable output and prove ambiguous or missing facts fail closed as denied or blocked states.

## Phase 3: Database Scopes, Collection Lifecycle, And Policy Admission

### Work Items

- **3.1 Implement database scope creation.**
  - Design: Create tenant/app/environment database scopes from approved manifests with service account refs, default data class, default retention, allowed regions, policy refs, and audit refs.
  - Output: `POST /overbase/databases` contract, scope record, duplicate handling, state transitions, and `overbase.database_created` event.
  - Validation: Tests reject missing tenant/app refs, unauthorized service accounts, invalid regions, unsupported data classes, and duplicate incompatible scope definitions.

- **3.2 Implement collection schema registration.**
  - Design: Register collection schemas with record type, field definitions, schema version, indexes, data class, retention, redaction policy, compatibility flags, and migration constraints.
  - Output: `POST /overbase/collections`, collection metadata read, schema version record, compatibility report, and `overbase.collection_created` or `overbase.schema_versioned` event.
  - Validation: Schema tests reject incompatible changes without a migration record, missing redaction policy, forbidden data-class downgrade, and index declarations that violate policy.

- **3.3 Add collection lifecycle state handling.**
  - Design: Implement draft, validating, active, indexing, migrating, read_only, degraded, restoring, suspended, retired, and deleted/tombstoned states with explicit allowed transitions.
  - Output: State machine implementation, reason-code matrix, transition events, recovery notes, and operator diagnostics.
  - Validation: State tests prove writes cannot occur in draft, read_only, restoring, suspended, retired, or deleted states except explicitly allowed maintenance paths.

- **3.4 Gate every schema and collection operation through policy.**
  - Design: Call Overguard or policy refs for data class, retention, replication, query, access, redaction, backup, restore, and migration decisions before side effects.
  - Output: Policy-admission client, policy input bundle, deny/block reason mapping, audit refs, and safe remediation hints.
  - Validation: Policy tests prove missing policy refs deny or block rather than silently applying default access.

- **3.5 Publish collection metadata views.**
  - Design: Provide operator, app, SDK/CLI, and audit metadata views with redaction according to tenant/app/data-class scope and service role.
  - Output: Metadata read model, pagination, field redaction rules, safe diagnostics, and source refs.
  - Validation: Read tests prove cross-tenant collection metadata is denied and sensitive policy/internal engine details are redacted from ordinary app views.

## Phase 4: Document And Key-Value Records

### Work Items

- **4.1 Implement document insert and update.**
  - Design: Validate payloads against collection schema, require tenant/app/actor refs, schema version, idempotency key, trace id, data-class policy refs, payload hash, etag/version, and object refs for large fields.
  - Output: Document write API, versioned record, tombstone-compatible mutation record, `overbase.document_written` event, and usage/audit refs.
  - Validation: Write tests reject invalid schema, missing idempotency, stale etag/conflict token, wrong tenant, forbidden data class, and raw large payload spillover.

- **4.2 Implement document point reads.**
  - Design: Read one document through policy and redaction checks with read-your-writes behavior for same actor/session/service-account trace where required by SDS #27.
  - Output: Point read API, redacted payload view, safe not-found/denied semantics, trace refs, and read audit/usage events.
  - Validation: Read tests prove cross-tenant reads, suspended collections, secret-bearing raw fields, and private payload exposure are denied or redacted.

- **4.3 Implement key-value records.**
  - Design: Support bounded inline values or value/object refs, key namespace prefix, ttl, version, conflict token, idempotency, data-class policy, and tombstone behavior.
  - Output: `PUT /overbase/kv/{collection_id}/{key}` and `GET /overbase/kv/{collection_id}/{key}` contracts, KV state records, ttl expiry records, and usage/audit refs.
  - Validation: KV tests cover conflict tokens, ttl expiry, missing prefixes, oversized inline values, wrong tenant/app, and policy denial.

- **4.4 Implement tombstones and retention-aware deletion.**
  - Design: Convert deletes into tombstone records with retention refs, audit refs, cleanup eligibility, policy checks, and restore/migration compatibility.
  - Output: Tombstone schema, cleanup queue record, retention report, redacted delete reads, and `overbase.record_tombstoned` event if added.
  - Validation: Deletion tests prove physical cleanup never removes required audit/retention evidence and restore plans can identify tombstoned records.

- **4.5 Emit document/KV usage and audit facts.**
  - Design: Emit raw usage for document writes, point reads, KV writes/reads, storage bytes, value refs, denied reads, conflict retries, and tombstone cleanup without billing semantics.
  - Output: Overmeter raw usage event contract, Overwatch audit mapping, idempotent retry/spool behavior, and source refs.
  - Validation: Usage tests prove Overbase never mutates ORU balances, Seal Ledger entries, invoices, pricing, payouts, or settlement state.

## Phase 5: Bounded Query, Redaction, And Secondary Index Lifecycle

### Work Items

- **5.1 Implement bounded filtered queries.**
  - Design: Add query API with collection id, schema version, filter, projection, page size, page token, time budget, index requirement, stale-read tolerance, data-class policy, and monotonic snapshot token where needed.
  - Output: `POST /overbase/collections/{collection_id}/query`, stable pagination, result shape, denial response, and query usage/audit refs.
  - Validation: Query tests reject unbounded scans, cross-tenant filters, missing index requirements, oversized pages, incompatible stale-read tolerance, and policy-denied projections.

- **5.2 Implement redaction and projection rules.**
  - Design: Apply redaction by data class, actor/service account, role, grant, policy refs, field sensitivity, operator/audit view, and downstream native app view.
  - Output: Redaction engine, projection validator, redacted result refs, safe diagnostics, and operator/audit view split.
  - Validation: Redaction tests prove user-facing and app-facing queries never expose raw private payloads, secret-bearing fields, or unauthorized vector/document content.

- **5.3 Implement secondary index declaration and build state.**
  - Design: Model secondary indexes with fields, query capability flags, state, build cursor, last verified version, failure reason, policy refs, and rebuild compatibility.
  - Output: `POST /overbase/indexes`, index metadata records, build operation records, `overbase.index_build_started`, and `overbase.index_build_completed` events.
  - Validation: Index tests prove writes remain valid when indexes are degraded only if policy allows degraded mode, and index builds never bypass collection policy.

- **5.4 Add query planner guardrails.**
  - Design: Choose allowed indexes, deny unindexed expensive queries unless explicit policy permits, cap query cost/time, and return stable reason codes for rejected plans.
  - Output: Query-plan report, plan reason codes, cost/time caps, degraded-mode rules, and remediation hints.
  - Validation: Planner tests prove no unbounded cross-tenant analytics or implicit full-scan fallback exists in v0.

- **5.5 Emit query and index operations evidence.**
  - Design: Record query shape, index refs, result counts, page tokens, time budget, denied policies, index build work, and redacted diagnostics without raw payload values.
  - Output: Query audit records, raw usage facts, slow query logs with safe shape fields, index operation events, and replay refs.
  - Validation: Evidence tests prove slow query logs and index diagnostics contain collection/index/trace refs but no private values or dependency internals.

## Phase 6: Append-Only Event Streams And Queued Operations

### Work Items

- **6.1 Implement event stream declaration.**
  - Design: Register streams with partition strategy, ordering model, retention, compaction policy, consumer policy, data class, schema version, and policy refs.
  - Output: Event stream schema, stream metadata API, stream state machine, and `overbase.stream_created` event if added.
  - Validation: Stream declaration tests reject missing partition model, incompatible retention, unauthorized consumer policy, and cross-tenant stream definitions.

- **6.2 Implement event append.**
  - Design: Append events with stream id, partition key, sequence behavior, producer ref, event type, payload hash/ref, schema version, idempotency key, trace id, and audit refs.
  - Output: `POST /overbase/streams/{stream_id}/events`, event record, ordering response, duplicate handling, and `overbase.event_appended` event.
  - Validation: Append tests prove declared partition ordering, duplicate idempotency behavior, schema validation, policy denial, and no raw private payload leakage.

- **6.3 Implement event range reads.**
  - Design: Read event ranges with access policy, retention limits, consumer position, pagination, compaction awareness, and redaction rules.
  - Output: `GET /overbase/streams/{stream_id}/events`, page tokens, consumer state refs, redacted event view, and audit/usage refs.
  - Validation: Read tests deny unauthorized consumers, expired retention windows, cross-tenant reads, and raw private payloads.

- **6.4 Move long-running operations to queued jobs.**
  - Design: Use queued operation jobs for index/vector rebuilds, bulk import/export, large scans, backup snapshots, restore dry-runs/applies, migrations/backfills, replication repair, compaction, tombstone cleanup, and any operation outliving request budgets.
  - Output: Operation job schema, operation id response, progress state, retry policy, queue handoff refs, and Overwatch evidence refs.
  - Validation: Job tests prove synchronous APIs reject long-running work and queued jobs resume idempotently after worker crashes.

- **6.5 Add operation health and retry semantics.**
  - Design: Track queued, running, blocked, retrying, completed, failed, cancelled, and rolled-back operation states with reason codes, retry windows, dependency refs, and operator diagnostics.
  - Output: Operation status API, retry/backoff rules, cancellation API, failure evidence, and recovery procedure.
  - Validation: Recovery tests cover crash before progress, after partial progress, during dependency outage, during cancellation, and during rollback.

## Phase 7: Vector Indexes And Authorized RAG Metadata

### Work Items

- **7.1 Define vector index collection contracts.**
  - Design: Model vector index collections with embedding model ref, vector dimension, shard refs, source field refs, source document/object refs, access scope, data class, grant refs, rebuild policy, revocation handling, and policy refs.
  - Output: Vector index schema, metadata API, rebuild operation record, compatibility matrix, and fixture examples.
  - Validation: Schema tests reject missing access scope, embedding model mismatch, dimension mismatch, missing grant refs, unsupported data class, and cross-tenant index declarations.

- **7.2 Implement vector indexing jobs.**
  - Design: Run vector build/rebuild jobs through queued operations with source refs, data-class filters, access-scope filters, embedding model refs, rebuild cursors, revocation handling, and Overwatch evidence.
  - Output: Vector rebuild job, `overbase.vector_index_rebuilt` event, progress report, failure reasons, and usage/audit refs.
  - Validation: Job tests prove private vectors are never shared across tenants/apps and revocation or source deletion triggers rebuild or denial evidence.

- **7.3 Implement authorized vector search.**
  - Design: Run Overguard access filtering before exposing candidates, apply the same tenant/app/data-class filters as normal reads, and return redacted result refs unless the caller has normal read access.
  - Output: Vector search API/read model, access-filter report, result refs, redacted snippets policy, and reason-coded denials.
  - Validation: Search tests deny raw private context exposure, shared private embedding pools, missing grants, stale access scopes, and model/dimension mismatches.

- **7.4 Support encrypted Docdex RAG metadata.**
  - Design: Store authorized index metadata, hashes, refs, access-scope evidence, rebuild policy, and redacted source refs for encrypted Docdex RAG while payload bytes remain in Overvault or encrypted Overstore refs.
  - Output: RAG metadata schema, adapter contract, revocation workflow, source-ref lifecycle, and test fixtures.
  - Validation: Integration tests prove secret-bearing/private payload bytes are not stored raw in Overbase and Docdex-style adapters cannot bypass normal read access.

- **7.5 Emit vector/RAG usage and governance evidence.**
  - Design: Emit raw usage for vector index operations, vector searches, rebuilds, denied access, revocation handling, and RAG metadata reads without billing semantics.
  - Output: Overmeter usage contract, Overwatch evidence mapping, governance review refs, and redacted operator diagnostics.
  - Validation: Evidence tests prove vector usage can be audited without exposing private vectors, raw context, secrets, or cross-tenant query details.

## Phase 8: Consistency, Replication, Backup, Restore, And Migration

### Work Items

- **8.1 Implement v0 consistency policy options.**
  - Design: Support single-record authoritative writes, read-your-writes for same actor/session/service-account trace, etags/conflict tokens, monotonic snapshot tokens for bounded queries, stale-read tolerance, and event partition ordering.
  - Output: Consistency policy schema, enforcement logic, conflict reason codes, query snapshot tokens, and documentation examples.
  - Validation: Consistency tests prove cross-collection, cross-shard, and global serializable transactions are not required for v0 and are not implied by API behavior.

- **8.2 Implement replication metadata and degraded handling.**
  - Design: Track replication groups, shard ids, replica refs, placement policy, health state, repair cursor, failover refs, lag refs, degraded write/read rules, and repair operation jobs.
  - Output: Replication metadata records, health read model, repair job, degraded-state events, and policy-controlled write restrictions.
  - Validation: Replication tests cover lag, unavailable replica, stale repair cursor, degraded query mode, write restriction, and failover evidence without direct engine exposure.

- **8.3 Implement backup snapshots.**
  - Design: Create structured-state backup snapshots with collection scope, schema versions, object refs, checkpoint, retention, integrity hash, restore eligibility, policy refs, and operation evidence.
  - Output: `POST /overbase/backups`, backup snapshot record, integrity report, `overbase.backup_snapshot_created` event, and usage/audit refs.
  - Validation: Backup tests prove snapshots contain enough schema/index/tombstone metadata for restore and keep large bytes as Overstore refs, not inline Overbase payloads.

- **8.4 Implement restore dry-run and apply.**
  - Design: Require restore dry-run before apply, conflict policy, target scope, access checks, cutover state, integrity verification, rollback plan, signed admin approval where needed, and Overwatch refs.
  - Output: `POST /overbase/restores`, dry-run report, conflict report, apply operation, rollback refs, and `overbase.restore_dry_run_completed` or `overbase.restore_applied` events.
  - Validation: Restore tests block conflicting, unauthorized, cross-tenant, missing-snapshot, integrity-failed, and multi-tenant cutover cases until proper approval exists.

- **8.5 Implement migration dry-run, approval, apply, and rollback.**
  - Design: Support additive or compatibility-preserving tenant-local migrations by tenant admin after dry-run, and central governance approval for shared/system-service, data-class, retention, redaction, vector-access, secret-bearing, regulated, cross-tenant, namespace/asset/ledger-affecting, destructive, restore-cutover, storage-engine, sharding, or replication-policy changes.
  - Output: `POST /overbase/migrations`, migration record, dry-run report, approval refs, rollout stage, rollback ref, and `overbase.migration_applied` event.
  - Validation: Migration tests prove unauthorized approval scopes, data-class downgrades, retention reductions, destructive changes, and shared schema changes fail closed.

## Phase 9: Provisioning, Native App Integration, And Operations

### Work Items

- **9.1 Add Overpack provisioning hooks.**
  - Design: Consume app/service manifests declaring database scopes, collections, schemas, secondary indexes, vector indexes, event streams, backup policy, restore policy, and migration hooks.
  - Output: Overpack provisioning adapter, manifest validation report, idempotent create/version behavior, rollback hooks, and operation refs.
  - Validation: Provisioning tests prove app deployment can declare state without bypassing Overbase policy or overwriting existing incompatible schemas.

- **9.2 Integrate SDK, CLI, and admin/developer UI reads.**
  - Design: Expose safe client flows for schema registration, document/KV reads and writes, bounded queries, event reads, operation status, backup/restore dry-run, migration dry-run, and diagnostics through normal Overgate/admin APIs.
  - Output: SDK/CLI/admin read-model requirements, stable JSON output, redaction profiles, and troubleshooting reason codes.
  - Validation: Client tests prove tools cannot become privileged backdoors and cannot read private engine, provider, or cross-tenant details.

- **9.3 Support first stateful app fixtures.**
  - Design: Prove workspace, directory listings, search, personal AI, encrypted Docdex RAG metadata, or another simple native app can create collections, write state, query state, append events, and recover from backup/restore.
  - Output: First app fixture, schema declarations, sample records, bounded query examples, backup/restore scenario, and usage/audit evidence.
  - Validation: End-to-end tests prove the app uses Overbase APIs and event refs rather than direct database files or hidden external services.

- **9.4 Add operations dashboards and health checks.**
  - Design: Publish health for storage backend access, schema registry, policy dependency, audit emission, usage emission, replication lag, backup freshness, restore readiness, migration status, query latency, vector index freshness, and denial trends.
  - Output: Health endpoint/schema, metrics/events, redacted diagnostics, dashboard fields, and operator follow-up refs.
  - Validation: Operations tests prove diagnostics include evidence refs and reason codes but not raw private payloads, secret values, or internal engine details outside authority.

- **9.5 Prepare native-app and adapter handoff contracts.**
  - Design: Document how native apps, adapters, AI gateway, encrypted Docdex RAG, deployment planner, and central AI stewardship consume Overbase collections, query refs, vector refs, event refs, operation refs, and backup/restore refs.
  - Output: Handoff matrix, dependency owners, redaction profiles, app-facing examples, and phased readiness notes.
  - Validation: Handoff review confirms downstream consumers can use Overbase without weakening Overstore, Overvault, namespace, Overmeter, Overwatch, Overguard, or governance boundaries.

## Phase 10: Validation, Documentation, Queue State, And Handoff

### Work Items

- **10.1 Validate contract and schema coverage.**
  - Design: Run focused checks for database scopes, collection schemas, document records, KV records, event streams, secondary indexes, vector indexes, consistency policies, replication groups, backup snapshots, restore plans, migration records, operation jobs, APIs, events, and reason codes.
  - Output: Schema-test report, state-machine test report, fixture coverage matrix, failure notes, and remediation list.
  - Validation: Tests pass before implementation advances beyond each documented gate; any blocker is recorded in build-plan progress.

- **10.2 Validate Phase 8 structured state end to end.**
  - Design: Prove one signed tenant/app flow creates a database scope, registers a collection, writes a document/KV record, queries state, appends an event, builds an index, runs backup dry-run, and emits usage/audit evidence.
  - Output: End-to-end structured-state fixture, source-ref bundle, operation records, usage ref, audit trail, and replay report.
  - Validation: Replay confirms successful, denied, blocked, conflict, degraded, backup-failed, restore-conflict, and migration-failed paths produce distinct auditable states.

- **10.3 Validate security, privacy, and tech-stack alignment.**
  - Design: Scan implementation and docs for raw private payload leakage, raw secret storage, large object byte storage, direct dependency storage reads, namespace ownership drift, billing/ledger mutation, conventional database product boundaries, blockchain/NFT mechanics, pricing/revenue/customer-count assumptions, and TypeScript core-runtime drift.
  - Output: Security/privacy checklist, tech-stack alignment report, negative-control scan results, and remediation notes.
  - Validation: Review confirms Overbase remains Rust-first/native-Overrid infrastructure and uses canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, policy refs, and native service boundaries.

- **10.4 Validate master-plan and downstream handoff alignment.**
  - Design: Confirm SDS #27, the Overbase service plan, master build plan, build-plan crosswalk, Phase 8 plan, Phase 9 plan, Phase 12 plan, Phase 13 plan, queue state, and progress docs link to this plan and preserve the Phase 8 first build point.
  - Output: Updated source-document links, sub-build-plan index entries, progress evidence, queue status, and downstream handoff matrix.
  - Validation: Markdown link checks pass and review confirms no master Phase 0 through Phase 13 ordering change was required.

- **10.5 Validate implementation handoff readiness.**
  - Design: Prepare the handoff for builders by listing required crates/modules, schemas, reason-code catalogs, storage abstraction, service clients, fixture groups, local harness scenarios, acceptance tests, and phase gates.
  - Output: Implementation handoff checklist, validation command list, known blockers, dependency owners, and first stateful app fixture target.
  - Validation: Handoff review confirms a builder can start Overbase Phase 8 implementation without reading informal agent notes or weakening SDS boundaries.

## Alignment Review

- The sub-build plan keeps Overbase first build work in master Phase 8, matching SDS #27, the service catalog entry, Phase 8 plan, master build plan, and build-plan crosswalk.
- The plan treats Phases 0, 1, 4, 5, and 7 as prerequisites for shared schemas, local fixtures, signed identity/tenant/request/audit rails, policy controls, usage emission, and grid-resident backup/recovery readiness rather than as Overbase's first implementation phase.
- The plan keeps Overstore, Overvault, Universal Namespace Service, Overmesh, Overasset, Overmeter, Overwatch, Overguard, Overpack, and governance authority outside Overbase while defining the refs and evidence Overbase consumes or emits.
- The plan preserves SDS #27 v0 consistency decisions: single-record authoritative writes, etags/conflict tokens, read-your-writes for the same actor/session/service-account trace, monotonic snapshot tokens or explicit stale-read tolerance for bounded queries, and append-only ordering within declared stream partitions.
- The plan gates vector search and encrypted Docdex RAG metadata behind tenant/app/access-scope isolation, Overguard filtering, revocation handling, redacted result refs, and normal read-access checks.
- The plan keeps long-running operations in queued jobs and avoids synchronous bulk scans, index rebuilds, backup/restore, migration, replication repair, compaction, or tombstone cleanup inside ordinary request budgets.
- The plan preserves the master Phase 0 through Phase 13 order and uses later phases only for provisioning hooks, native-app consumption, federation/public considerations where policy allows, and governance/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first Overbase core, native Overrid boundaries, Tokio, Axum/Tower/Hyper-style service boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, Overrid-owned storage abstraction, and no conventional database, object-store, vault, queue, blockchain, NFT, pricing, revenue, customer-count, or external-payment assumptions.
