SDS #28

# Overstore SDS

## Purpose

Provide durable content-addressed persistence for files, media, packages, datasets, model artifacts, snapshots, backups, logs, research outputs, and other large objects used by Overrid services and native apps.

Overstore is the object and artifact storage layer. It owns object identity, chunking, checksums, placement, replication or erasure coding, repair, retention, transfer sessions, and verification evidence. It does not own structured app state, secrets, namespace records, or package manifest semantics.

## Source Documents

| Source | Path |
| --- | --- |
| Service implementation plan | [overstore.md](../../service_catalog/data_storage_namespace/overstore.md) |
| Master SDS | [master_sds.md](../master_sds.md) |
| Master service catalog | [master_services.md](../../service_catalog/master_services.md) |
| Build-plan crosswalk | [service_catalog_alignment.md](../../build_plan/service_catalog_alignment.md) |
| Build phase alignment | [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md) |
| Sub-build plan | [SUB BUILD PLAN #28 - Overstore](../../build_plan/sub_build_plan_028_overstore.md) |

## Service Family

- Family: Data, storage, and namespace
- Owning layer: Content-addressed object persistence, transfer, integrity, redundancy, repair, and retention
- Primary data scope: object records, chunk metadata, content hashes, manifests, upload sessions, download grants, transfer sessions, placement records, replica records, repair jobs, verification events, retention policies, deletion tombstones, and storage usage facts
- First build phase from service plan: [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md)

## Problem Statement

The execution loop, native apps, AI/RAG, deployments, backups, media flows, and research workflows all need large-object persistence. Putting those objects inside structured databases or private vaults would make integrity checks, replication, repair, transfer, and metering difficult.

Overstore provides the platform object layer. Every object must be addressable, verifiable, policy-bound, metered, and recoverable without exposing private data or relying on speculative asset mechanics.

## Goals

- Store and retrieve content-addressed objects with deterministic hashes and chunk metadata.
- Support package artifacts, run outputs, logs, media, model artifacts, datasets, snapshots, and backups.
- Encrypt before placement when policy or data class requires it.
- Track placement, replica, erasure-coding, repair, retention, and verification records.
- Provide resumable upload/download and transfer-session contracts.
- Preserve object integrity evidence for Overwatch, Oververify, Overclaim, and disputes.
- Emit raw storage and bandwidth usage events to Overmeter.

## Non-Goals

- Do not store structured app state as first-class records. Overbase owns state.
- Do not store raw secret values or decide secret access. Overvault owns secrets and private encrypted records.
- Do not define package manifest semantics. Overpack owns manifests and package contracts.
- Do not own human-readable names, route bindings, or ownership rights.
- Do not bypass Overguard data-class, retention, replication, or access policy.
- Do not use object ownership as NFT-style speculative asset mechanics.

## Primary Actors And Clients

- Overrun, storing outputs, logs, artifacts, and diagnostic bundles.
- Overpack, storing and verifying package artifacts and deployment bundles.
- Overbase, referencing large payload spillover, backups, exports, and snapshots.
- Overvault, referencing encrypted payload objects where policy allows.
- Native apps storing files, media, attachments, maps data, workspace files, and public assets.
- Personal AI and encrypted Docdex RAG storing indexes, datasets, and model-related artifacts.
- Deployment planner, backup/restore, release strategy, and package validator services.
- Overmesh, supporting private transfer paths and locality-aware object movement.
- Overwatch, Oververify, Overclaim, and Overmeter consuming evidence and usage refs.

## Dependencies

- [Overguard](../trust_policy_verification/overguard.md) for object data-class, access, retention, replication, and placement policy.
- [Overvault](overvault.md) for encryption policy refs, key refs, and private-object handling.
- [Overmesh](../execution_scheduling/overmesh.md) for transfer sessions, route health, and private connectivity.
- [Overpack](../execution_scheduling/overpack.md) for artifact manifests, package refs, and validation expectations.
- [Overbase](overbase.md) for structured metadata refs, catalog entries, and app state pointers.
- [Universal Namespace Service](universal_namespace_service.md) for optional human-readable object or route bindings.
- [Overcell](../execution_scheduling/overcell.md) and [Overlease](../execution_scheduling/overlease.md) for storage-node execution context where provider storage is used.
- [Overwatch](../control_plane/overwatch.md) for audit and integrity events.
- [Overmeter](../execution_scheduling/overmeter.md) for raw usage events.

## Owned Responsibilities

Overstore owns:

- Object id, content hash, chunk map, object version, and object metadata records.
- Upload session, commit, verification, and resumability behavior.
- Download grant, range read, and redacted metadata read behavior.
- Placement and replica records for local, private-swarm, grid-resident, and later federation storage.
- Replication or erasure-coding policy execution.
- Repair jobs when integrity, redundancy, or placement policy falls below target.
- Retention, expiration, deletion tombstone, and quarantine records.
- Verification evidence for corruption, tampering, repair, and challenge hooks.
- Raw usage events for bytes stored, transferred, replicated, repaired, verified, and deleted.

Overstore must not treat a metadata record as proof of object integrity. Integrity comes from hashes, chunk verification, replica verification, and repair evidence.

## Data Model

The first implementation should define:

- `object_record`: object id, tenant id, app id, owner ref, object kind, data class, content hash, size, media type, version, manifest refs, retention policy, encryption refs, and state.
- `object_version`: object id, version id, root hash, previous version, created-by ref, commit time, compatibility refs, and rollback eligibility.
- `chunk_record`: chunk id, object id, version id, byte range, chunk hash, size, compression flag, encryption envelope ref, replica refs, and verification state.
- `upload_session`: session id, object intent, expected size, expected root hash, chunk size, allowed data class, expiration, resumability token hash, and commit state.
- `download_grant`: grant id, object id/version, actor/service account, allowed ranges, ttl, data-class policy refs, and audit refs.
- `transfer_session`: source ref, destination ref, route refs, protocol profile, bandwidth limits, retry policy, chunk cursor, and final integrity result.
- `placement_record`: object/chunk id, storage node or provider ref, region/locality, trust scope, storage class, redundancy role, and health state.
- `replica_set`: object/chunk id, required replica count or erasure profile, current healthy count, degraded count, and repair target.
- `repair_job`: object/chunk id, reason, source replica refs, destination refs, retry state, verification result, and final evidence refs.
- `verification_event`: object/chunk id, verification type, hash result, signer ref, challenge ref where applicable, and evidence ref.
- `retention_record`: policy ref, retain-until, expiration action, legal/compliance hold ref if applicable, deletion eligibility, and tombstone refs.
- `quarantine_record`: object/chunk id, reason, detected-by ref, access restriction, remediation state, and Overclaim refs.

Common envelope fields:

- `id`, `tenant_id`, `app_id`, `actor_id` or service account.
- `trace_id`, `idempotency_key`, `state`, `created_at`, `updated_at`, `policy_refs`, and `audit_refs`.

## API Surface

The v0 API should support object lifecycle and transfer without hiding integrity behavior:

- `POST /overstore/uploads`: create an upload session with object intent, expected hash, data class, and policy refs.
- `PUT /overstore/uploads/{session_id}/chunks/{chunk_index}`: upload a chunk with hash and range.
- `POST /overstore/uploads/{session_id}/commit`: verify uploaded chunks and create an object version.
- `GET /overstore/objects/{object_id}`: read redacted object metadata.
- `POST /overstore/objects/{object_id}/download-grants`: create a bounded read grant.
- `GET /overstore/objects/{object_id}/versions/{version_id}/content`: read authorized ranges through a grant.
- `POST /overstore/transfers`: create a transfer session for route-bound object movement.
- `POST /overstore/objects/{object_id}/verify`: verify hashes, replica health, or challenge evidence.
- `POST /overstore/objects/{object_id}/repair`: schedule repair for degraded objects or chunks.
- `POST /overstore/objects/{object_id}/retention`: update retention through a policy-checked administrative action.
- `DELETE /overstore/objects/{object_id}`: create deletion tombstone or expiration workflow, never silent deletion.

API requirements:

- Object writes require tenant/app context, data class, policy refs, trace id, and idempotency key.
- Commits must verify expected hashes before an object becomes available.
- Reads must use grants, range bounds, and data-class policy.
- Transfer sessions must record route refs and final integrity checks.
- Delete requests must honor retention, dispute, backup, and audit policy.

## Event Surface

- `overstore.upload_started`
- `overstore.chunk_received`
- `overstore.object_committed`
- `overstore.object_verified`
- `overstore.download_grant_issued`
- `overstore.transfer_started`
- `overstore.transfer_completed`
- `overstore.replica_degraded`
- `overstore.repair_started`
- `overstore.repair_completed`
- `overstore.object_quarantined`
- `overstore.retention_updated`
- `overstore.object_tombstoned`
- `overstore.object_deleted`

Events must include object, version, chunk, placement, policy, trace, and usage refs without embedding private object bytes.

## Core Workflow

1. Client or service requests an upload session with object intent, expected hash, data class, and retention policy.
2. Overstore verifies actor, tenant, app, data-class, and policy refs.
3. Client uploads chunks with hashes and byte ranges.
4. Overstore verifies chunk hashes and commits the object only when the root hash matches.
5. Overstore places chunks according to replication or erasure policy.
6. Overstore emits usage, placement, and integrity events.
7. Reads use bounded download grants and range requests.
8. Verification and repair jobs continuously or periodically check integrity and redundancy.
9. Expiration or deletion creates tombstones and cleanup evidence according to retention policy.

## State Machine

Object version lifecycle:

1. `declared`: upload intent exists.
2. `uploading`: chunks are being accepted.
3. `committing`: root hash, chunk map, and policy are being verified.
4. `available`: object version is readable under policy.
5. `replicating`: redundancy target is not yet fully satisfied.
6. `degraded`: integrity or redundancy is below policy.
7. `repairing`: repair job is active.
8. `quarantined`: object access is restricted due to corruption, abuse, policy, or dispute.
9. `retained`: object is kept by retention or hold policy.
10. `expiring`: expiration workflow is active.
11. `tombstoned`: logical deletion is recorded.
12. `purged`: physical cleanup is complete where policy allows.

Object version and tombstone history are append-only. A new object version replaces normal reads only through explicit pointer or metadata update.

## Policy And Security

- Encryption-before-placement is mandatory for private, secret-bearing, regulated, and policy-marked objects.
- Overstore stores encryption envelope refs and encrypted bytes where applicable; it does not expose raw key material.
- Access policy, data class, retention, replication, and placement policy are first-class object fields.
- Cross-tenant reads require explicit grants or public policy.
- Download grants must be bounded by ttl, actor, object version, byte range, and data class.
- Object metadata must be redacted when size, media type, or placement details could leak sensitive information.
- Quarantine must override normal read grants until cleared by policy.
- Provider storage nodes must not be trusted solely because they hold bytes; integrity checks and challenge hooks are required.

## Metering And Accounting

Overstore emits raw usage events; it does not bill:

- Bytes uploaded, downloaded, stored, replicated, repaired, verified, quarantined, expired, and purged.
- Transfer route, region/locality, storage class, data class, object kind, tenant/app, and service-account dimensions.
- Repair and verification work linked to provider/node refs where applicable.
- Failed policy-denied reads and writes as diagnostic events where useful.
- No external payment calls or ORU balance mutations occur in Overstore.

## Observability And Operations

- Operators need object count, bytes by data class, upload failures, commit hash failures, transfer success, replica health, degraded objects, repair backlog, quarantine count, retention backlog, and purge backlog.
- Health checks should cover storage backend access, chunk hash verification, policy dependency, Overvault key-ref dependency, Overmesh transfer dependency, Overwatch emission, and Overmeter emission.
- Integrity dashboards should show verification freshness and corruption trends by storage class and provider.
- Repair workers must be rate-limited and resumable.
- Storage placement policy changes must be dry-run against sampled objects before broad rollout.

## Failure Modes And Recovery

- Chunk hash mismatch: reject chunk and keep session resumable until expiration.
- Root hash mismatch: reject commit and keep upload evidence.
- Policy denial: deny before storing bytes when possible; quarantine or tombstone if denial is discovered later.
- Storage-node failure: mark replicas degraded and schedule repair.
- Transfer failure: preserve cursor, retry according to policy, and verify destination before success.
- Corrupt replica: quarantine replica, repair from healthy replicas, and emit evidence.
- Retention conflict: block deletion and return safe reason refs.
- Object metadata lost but chunks available: rebuild metadata only from verified chunk map and audit refs.
- Overvault unavailable for encrypted object reads: block reads rather than serving encrypted material incorrectly.

## Validation Plan

The service implementation plan lists these requirements:

- Objects can be stored, retrieved, verified, and repaired.
- Corrupt chunks are detected.
- Access policy prevents unauthorized reads.

Additional SDS-level validation:

- Contract tests for upload session, chunk upload, commit, metadata read, download grant, range read, transfer, verify, repair, retention, and delete APIs.
- Hash validation tests for correct, missing, corrupt, duplicate, and out-of-order chunks.
- Encryption-before-placement tests for private, regulated, and secret-bearing object classes.
- Tenant isolation and grant-bound read tests.
- Replica degradation and repair tests.
- Quarantine tests for corrupt, abusive, disputed, and policy-denied objects.
- Retention, tombstone, expiration, and purge tests.
- Usage emission tests for upload, download, storage, replication, repair, verification, and deletion work.

## Build Breakdown

1. Define object, version, chunk, upload session, download grant, placement, replica, repair, verification, retention, and tombstone schemas.
2. Implement content addressing, chunk upload, checksums, and object commit.
3. Add policy-bound download grants and range reads.
4. Add encryption-before-placement refs and Overvault integration.
5. Add replication or erasure-coding metadata.
6. Add transfer sessions over Overmesh-compatible route refs.
7. Add verification and repair jobs.
8. Add retention, quarantine, tombstone, and purge workflows.
9. Add proof/challenge hooks for untrusted providers in later trust phases.

## Handoff And Downstream Use

Overstore backs Overrun artifacts, Overpack packages, Docdex indexes, workspace files, media, datasets, model artifacts, backups, native app assets, and research outputs. Consumers should store object refs and integrity refs, not copy raw object bytes into unrelated records.

## Open Design Questions

- Resolved: founder-hardware v0 should use a native Overstore local shard backend: content-addressed chunk files on directly attached storage, BLAKE3 root and chunk hashes, append-only object/version/chunk metadata behind the Overrid storage abstraction, resumable upload sessions, and deterministic repair scans. S3, MinIO, Ceph, or cloud object stores may be compared or bridged later, but they must not become the Overstore product boundary.
- Resolved: the default pre-federation redundancy profile is replication with verified repair, not mandatory erasure coding. Overstore should model erasure profiles in the schema from v0, but enable only a private/grid-resident cold-object profile such as `rs_4_2_private` after there are at least six independent same-trust storage targets, successful full-object restore tests, and repair evidence through Overwatch. Public or cross-trust erasure shards wait for trusted federation/public-provider phases.
- Resolved: user-facing listings for private data classes may show only caller-authorized display refs, object kind, version label, state, coarse size bucket, coarse created/updated time, retention or hold state, data-class label when policy allows it, and explicit grant status. They must not expose content hashes, chunk ids, exact byte layout, storage paths, placement refs, provider/node ids, replica health, encryption/key refs, private route refs, inferred media details, or cross-tenant existence signals.
- Resolved: untrusted public providers require signed storage leases plus Oververify/Challenge Task Service evidence before becoming eligible. The first proof path should use nonce-bound random chunk or byte-range challenges against BLAKE3 chunk/root commitments, require bounded retrieval proofs over encrypted bytes, record every challenge and failure in Overwatch, and feed repair, quarantine, dispute, and payout-hold decisions. Provider metadata alone is never proof of storage.
- Resolved: deduplication is scoped by tenant, app/service, data class, encryption policy, retention policy, and caller authorization. Private, regulated, secret-bearing, user-private, and organization-private objects must not use global visible dedup or return "already exists" signals across tenants; physical coalescing is allowed only inside an equivalent policy scope and without exposing hashes to users. Global dedup is allowed only for public or explicitly shared artifacts, packages, models, or datasets whose data class permits public content-hash disclosure.
