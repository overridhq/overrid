SDS #27

# Overbase SDS

## Purpose

Provide the distributed application-state substrate for Overrid: document collections, key-value records, append-only event streams, vector indexes, schema versions, secondary indexes, consistency policies, replication metadata, backup snapshots, and restore plans.

Overbase is the structured state layer. It is not object storage, secret storage, namespace ownership, or a generic replacement for every database pattern on day one. It gives native apps, adapters, AI/RAG services, and platform services a governed way to store and query state through identity, tenant, policy, audit, and metering rails.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overbase.md](../../service_catalog/data_storage_namespace/overbase.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) |

## Service Family

- Family: Data, storage, and namespace
- Owning layer: Governed structured state, query, index, replication, and backup contracts
- Primary data scope: databases, collections, schemas, documents, key-value records, event streams, vector indexes, secondary indexes, consistency policies, replication groups, snapshots, restore plans, migrations, and access-scope refs
- First build phase from service plan: [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md)

## Problem Statement

Overrid native apps and platform services need durable state without each product inventing its own database, access-control layer, backup strategy, query rules, and tenant isolation model. If every native app stores state differently, policy, privacy, metering, recovery, search, and AI/RAG integration become fragile.

Overbase solves the structured-state part of the platform. It gives applications a common state contract while keeping large binary objects in Overstore, secrets and private encrypted records in Overvault, and human-readable naming in the Universal Namespace Service.

## Goals

- Support document and key-value collections for the first stateful apps.
- Support append-only event streams for app events, state changes, and integration trails.
- Support vector index collections for search and authorized RAG metadata.
- Enforce tenant, app, actor, role, data-class, and policy refs on every collection and query.
- Keep schemas, indexes, consistency policy, replication policy, backup snapshots, and restore plans versioned.
- Provide deterministic migration, index build, backup, restore, and replay behavior.
- Emit usage and audit events that can be consumed by Overmeter and Overwatch.

## Non-Goals

- Do not store large blobs, media, packages, model artifacts, backups, or chunk data. Overstore owns object persistence.
- Do not store raw secrets or act as a private vault. Overvault owns secret and encrypted private-state controls.
- Do not own names, route bindings, or asset-right records.
- Do not bypass Overguard when data class, retention, replication, or query policy changes.
- Do not provide unbounded cross-tenant analytics in v0.
- Do not promise strong global consistency for every collection before the phase plan proves the operational model.

## Primary Actors And Clients

- Native apps such as workspace, directory listings, search, messaging, wallet, social, maps, and central AI stewardship.
- Personal AI, AI Gateway Router, and encrypted Docdex RAG, storing authorized indexes and metadata refs.
- Product adapters such as Docdex, Mcoda, Codali, and mSwarm bridge components.
- Deployment planner and grid-resident backbone services storing service state.
- Admin/developer UI, CLI, and SDK clients reading diagnostics and managing schemas.
- Overwatch and Overmeter consuming state-change and usage events.

## Dependencies

- [Overtenant](../control_plane/overtenant.md) for tenant boundaries, membership, role scope, suspension, and offboarding refs.
- [Overpass](../control_plane/overpass.md) for stable identities, app refs, service refs, and namespace-linked subjects.
- [Overkey](../control_plane/overkey.md) for service-account credentials and signing refs.
- [Overgate](../control_plane/overgate.md) for API ingress, idempotency, schema validation, and request admission.
- [Overguard](../trust_policy_verification/overguard.md) for data-class, retention, replication, query, and access policy decisions.
- [Overwatch](../control_plane/overwatch.md) for append-only audit, trace, and evidence refs.
- [Overmeter](../execution_scheduling/overmeter.md) for raw state usage events.
- [Overstore](overstore.md) for object refs used by documents, snapshots, exports, and large payload spillover.
- [Overvault](overvault.md) for secret refs and private encrypted records referenced by Overbase records.
- [Overpack](../execution_scheduling/overpack.md) for app provisioning manifests and schema/index declarations.

## Owned Responsibilities

Overbase owns:

- Database, collection, schema, index, vector-index, and event-stream lifecycle.
- Structured record validation against versioned schemas.
- Query contracts, bounded pagination, filtering, projections, and redaction behavior.
- Consistency policy, replication policy, and migration records.
- Backup snapshot and restore-plan metadata for structured state.
- App/tenant boundary enforcement for state reads and writes.
- Raw usage facts for storage, query, index, vector, backup, restore, and migration work.
- Replayable audit refs for mutations and administrative operations.

Overbase must not let app code bypass its policy gates through direct storage access. If a low-level storage engine is used, Overbase remains the authoritative contract boundary.

## Data Model

The first implementation should define:

- `database_scope`: tenant id, app id, environment, service account refs, default data class, default retention, and allowed regions.
- `collection_schema`: collection id, schema version, record type, field definitions, indexes, data class, retention policy, redaction policy, and compatibility flags.
- `document_record`: collection id, record id, schema version, payload hash, object refs for large fields, version, etag, state, and policy refs.
- `kv_record`: collection id, key, namespace prefix, value hash, value ref or inline bounded value, ttl, version, and conflict token.
- `event_stream`: stream id, partition strategy, ordering model, retention, compaction policy, consumer policy, and data class.
- `event_record`: stream id, sequence, producer ref, event type, payload hash, payload ref, schema version, idempotency key, and audit refs.
- `secondary_index`: collection id, index fields, index state, build cursor, last verified version, and query capability flags.
- `vector_index`: collection id, embedding model ref, vector dimension, shard refs, source field refs, access scope, and rebuild policy.
- `consistency_policy`: read/write consistency mode, conflict detection, conflict resolution rule, and stale-read tolerance.
- `replication_group`: shard id, replica refs, placement policy, health state, repair cursor, and failover refs.
- `backup_snapshot`: snapshot id, collection scope, schema versions, object refs, checkpoint, retention, integrity hash, and restore eligibility.
- `restore_plan`: source snapshot, target scope, dry-run report, conflict policy, access checks, and cutover state.
- `migration_record`: source schema, target schema, migration script/package ref, dry-run report, rollout stage, and rollback ref.

Common envelope fields:

- `id`, `tenant_id`, `app_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

The v0 API should stay narrow and explicit:

- `POST /overbase/databases`: create a tenant/app database scope from an approved app manifest.
- `POST /overbase/collections`: create or version a collection schema.
- `GET /overbase/collections/{collection_id}`: read collection metadata and current schema.
- `POST /overbase/collections/{collection_id}/documents`: insert or update a document with schema validation.
- `GET /overbase/collections/{collection_id}/documents/{record_id}`: read one document with redaction and policy checks.
- `POST /overbase/collections/{collection_id}/query`: run bounded filtered queries with stable pagination.
- `PUT /overbase/kv/{collection_id}/{key}` and `GET /overbase/kv/{collection_id}/{key}`: manage bounded key-value records.
- `POST /overbase/streams/{stream_id}/events`: append event records with idempotency and ordering guarantees.
- `GET /overbase/streams/{stream_id}/events`: read event ranges within access and retention policy.
- `POST /overbase/indexes`: create or rebuild secondary and vector indexes.
- `POST /overbase/backups`: create structured-state backup snapshots.
- `POST /overbase/restores`: dry-run or execute restore plans.
- `POST /overbase/migrations`: dry-run, stage, apply, or roll back schema migrations.

API requirements:

- Mutating calls require actor or service-account identity, tenant/app context, trace id, idempotency key, and schema version.
- Query calls must be bounded by page size, time budget, index availability, and data-class policy.
- Vector search must apply the same tenant/app/data-class filters as normal reads.
- Backup, restore, migration, index build, and replication operations must create auditable operation records.
- Errors must use shared Overrid reason codes and include safe remediation hints.

## Event Surface

- `overbase.database_created`
- `overbase.collection_created`
- `overbase.schema_versioned`
- `overbase.document_written`
- `overbase.document_read_denied`
- `overbase.query_executed`
- `overbase.event_appended`
- `overbase.index_build_started`
- `overbase.index_build_completed`
- `overbase.vector_index_rebuilt`
- `overbase.backup_snapshot_created`
- `overbase.restore_dry_run_completed`
- `overbase.restore_applied`
- `overbase.migration_applied`
- `overbase.replica_degraded`

Events should include tenant, app, collection, schema, policy, trace, operation, and usage refs. They must not include raw private payloads where a hash or object ref is sufficient.

## Core Workflow

1. App or service manifest declares required collections, schemas, indexes, event streams, and vector indexes.
2. Overbase validates tenant/app ownership and policy refs.
3. Overbase creates database scope and collection schemas.
4. Clients write documents, key-value records, or event records through validated APIs.
5. Overbase applies schema, data-class, access, consistency, and retention policy.
6. Overbase updates indexes and emits state-change, audit, and usage events.
7. Backup snapshots and migration records are created through explicit operations.
8. Restore plans run as dry runs before destructive or broad changes.
9. Consumers query through bounded APIs, never direct storage reads.

## State Machine

Collection lifecycle:

1. `draft`: schema is being prepared and has no writes.
2. `validating`: schema and policy refs are being checked.
3. `active`: collection accepts reads and writes.
4. `indexing`: one or more indexes are building or rebuilding.
5. `migrating`: schema migration is in progress.
6. `read_only`: writes are blocked for maintenance, policy, restore, or migration.
7. `degraded`: replication, index, or storage health is below policy.
8. `restoring`: restore plan is applying to the collection.
9. `suspended`: tenant/app/policy state prevents normal use.
10. `retired`: collection no longer accepts writes but may be retained for reads.
11. `deleted`: collection tombstone exists and physical cleanup is tracked separately.

Record versions are append-only or correction-based. Deletions must leave tombstone and audit refs according to retention policy.

## Policy And Security

- Every database, collection, stream, index, query, backup, restore, and migration has tenant, app, actor/service-account, data-class, retention, and policy refs.
- Cross-tenant reads are denied unless an explicit grant, delegation, or public data policy allows them.
- Public, public low-sensitivity, tenant private, user private, organization private, secret-bearing, regulated, system-service, and grant-funded public-interest data classes must be distinct.
- Secret-bearing fields must be stored as Overvault refs or encrypted object refs, not raw Overbase payloads.
- Large payloads must spill to Overstore object refs with checksums and policy refs.
- Indexes and vector indexes must not leak private data through shared embeddings or cross-tenant query surfaces.
- Restore and migration operations require signed administrative action and Overwatch evidence.
- Offboarding must preserve required audit and retention records while revoking normal access.

## Metering And Accounting

Overbase emits raw usage events; it does not bill:

- Storage bytes by data class, document writes, key-value writes, event appends, query units, index builds, vector index operations, backup bytes, restore work, replication traffic, and migration work.
- Usage must link to tenant, app, actor/service account, collection, operation type, data class, and policy refs.
- Failed policy-denied operations can emit diagnostic usage without creating chargeable settlement semantics.
- Usage rollups flow to Overmeter and later accounting layers; Overbase never mutates ORU balances or Seal Ledger entries.

## Observability And Operations

- Operators need visibility into collection health, schema versions, index state, vector index state, replication health, backup freshness, restore readiness, migration status, query latency, and policy-denial trends.
- Health checks should cover storage backend access, schema registry, policy dependency, audit emission, usage emission, replication lag, and backup target availability.
- Slow query logs must redact payload values and keep only safe shape, collection, index, and trace refs.
- Migration and restore operations must have dry-run reports and rollback plans before broad application.
- Capacity dashboards should show growth by tenant, app, collection, data class, and replica group.

## Failure Modes And Recovery

- Invalid schema: reject collection or write before side effects.
- Missing policy refs: deny or block rather than applying default access.
- Cross-tenant query: deny and emit audit refs.
- Index unavailable: use explicit degraded query mode or reject if policy requires index-backed access.
- Vector index mismatch: reject query if embedding model or dimension differs from the collection contract.
- Replication lag: mark collection degraded, restrict writes if consistency policy requires it, and schedule repair.
- Backup failure: keep collection active when safe but mark backup freshness degraded.
- Restore conflict: stop at dry-run or blocked state until conflict policy is approved.
- Migration failure: preserve source schema, rollback if possible, and keep migration evidence.

## Validation Plan

The service implementation plan lists these requirements:

- A simple app can create and query state.
- Cross-tenant data access is denied.
- Backup and restore preserve collection consistency.

Additional SDS-level validation:

- Contract tests for database, collection, document, key-value, stream, query, index, vector index, backup, restore, and migration APIs.
- Schema validation tests for accepted, rejected, missing, incompatible, and migrated records.
- Tenant isolation tests for reads, writes, queries, vector search, backups, restores, and indexes.
- Redaction tests for user-facing and operator-facing reads.
- Consistency tests for selected v0 consistency modes.
- Backup and restore tests with integrity hashes and replay verification.
- Migration dry-run and rollback tests.
- Usage emission tests for writes, reads, queries, index builds, backups, restores, and failed policy checks.

## Build Breakdown

1. Define database scope, collection schema, document record, key-value record, event stream, index, backup, restore, and migration schemas.
2. Implement document and key-value collections with tenant/app/data-class policy enforcement.
3. Add bounded query API with stable pagination and reason-coded errors.
4. Add append-only event streams.
5. Add secondary index lifecycle.
6. Add vector index collections for search and authorized RAG metadata.
7. Add consistency policy options and replication metadata.
8. Add backup snapshot and restore dry-run/apply flows.
9. Add Overpack provisioning hooks for app-declared schemas and indexes.

## Handoff And Downstream Use

Overbase supports workspace, directory listings, search, personal AI, encrypted Docdex RAG metadata, native apps, product adapters, and platform services. Downstream services must use Overbase APIs and event refs rather than reading private database files or bypassing policy.

## Open Design Questions

- Resolved: the first stateful app requires single-record authoritative writes with idempotency keys, etags/conflict tokens, and read-your-writes behavior for the same actor, session, or service-account trace. Bounded queries should use a monotonic snapshot token or explicit stale-read tolerance, and event streams need append-only ordering within a declared stream partition. Cross-collection, cross-shard, and global serializable transactions are not mandatory for v0; derived views, secondary indexes, vector indexes, and replication repair can be eventually consistent when the record carries rebuild, lag, and audit refs.
- Resolved: encrypted Docdex RAG requires tenant/app/access-scope-isolated vector indexes, embedding model and dimension refs, source document/object refs, data-class refs, grant refs, rebuild policy, and revocation handling. Vector search must run Overguard access filtering before exposing candidates, must never share private vectors across tenants or apps, and must return redacted result refs rather than raw private context unless the caller already has normal read access. Secret-bearing or private payload bytes stay in Overvault or encrypted Overstore refs; Overbase stores only authorized index metadata, hashes, refs, and policy evidence.
- Resolved: synchronous collection operations are limited to database/collection metadata reads, policy-checked collection creation or schema-version registration, small document and key-value writes, point reads, bounded index-backed queries, and single event appends that complete inside the request time budget. Queued jobs are required for index and vector-index builds or rebuilds, bulk imports/exports, large scans, backup snapshot creation, restore dry-runs and applies, schema migrations with backfill, replication repair, compaction, tombstone cleanup, and any operation that can outlive the caller request or touch many records. Queued jobs must return operation ids, progress state, retry policy, and Overwatch evidence refs.
- Resolved: founder-hardware v0 supports an Overrid-owned embedded storage profile behind the Overbase storage abstraction, with a redb-backed pure-Rust candidate as the first supported local shard engine if benchmarks meet correctness and recovery needs. The contract must keep append-only operation records, BLAKE3 payload/object refs, snapshot/export hooks, deterministic reset/seed for local testing, and migration paths to grid-resident shards. RocksDB-like engines may be benchmarked later as internal shard-engine candidates, but PostgreSQL, MongoDB, DynamoDB, or similar products must not become the Overbase product boundary.
- Resolved: tenant admins may approve additive or compatibility-preserving migrations inside their tenant/app scope, including new optional fields, new secondary indexes, bounded backfills, non-sensitive retention-preserving transforms, and tenant-local rollback plans after a successful dry run. Central governance approval is required for shared/system-service schemas, data-class downgrades, retention reductions, redaction or vector-access changes, secret-bearing or regulated data migrations, cross-tenant migrations, namespace/asset/ledger-affecting migrations, destructive bulk deletes, restore cutovers that affect multiple tenants, and storage-engine, sharding, or replication-policy changes. Every approved migration must carry signed approval refs, dry-run evidence, rollback refs, and Overwatch audit refs.
