# SUB BUILD PLAN #28 - Overstore

Attached SDS: [docs/sds/data_storage_namespace/overstore.md](../sds/data_storage_namespace/overstore.md)

## Purpose

This sub-build plan turns SDS #28 into an implementation sequence for Overstore. It stays aligned with the master build plan, the service catalog, the SDS layer, and the accepted Rust-first tech stack.

Overstore is the native content-addressed object and artifact persistence layer for files, media, packages, datasets, model artifacts, snapshots, backups, logs, research outputs, and other large objects. It owns object identity, chunking, BLAKE3 root and chunk hashes, upload sessions, download grants, transfer sessions, placement, replica health, repair, verification evidence, retention, quarantine, tombstones, and raw usage events. It is not structured app state, a private vault, package manifest semantics, namespace ownership, an accounting mutator, or a conventional S3/MinIO/Ceph/cloud-object-storage product boundary.

## Source Alignment

| Source | Alignment rule |
| --- | --- |
| [SDS #28: Overstore](../sds/data_storage_namespace/overstore.md) | Controls Overstore purpose, non-goals, actors, dependencies, owned responsibilities, data model, APIs, events, workflows, state machine, policy/security, metering boundaries, operations, failure modes, validation, build breakdown, handoff, and resolved open-question decisions. |
| [Overstore service plan](../service_catalog/data_storage_namespace/overstore.md) | Controls the service-catalog objective, first build phase, dependencies, development order, contracts, validation, design alignment, and handoff. |
| [Master build plan](master_plan.md) | Controls the canonical build order from master Phase 0 through master Phase 13. |
| [Phase 0: Foundation](phase_00_foundation.md) | Supplies shared schemas, canonical JSON/JSON Schema discipline, signed envelopes, idempotency, trace ids, stable reason codes, local fixtures, deterministic harnesses, and integration-test prerequisites. |
| [Phase 1: Control-Plane Skeleton](phase_01_control_plane_skeleton.md) | Supplies Overgate, Overpass, Overtenant, Overkey, Overregistry, Overwatch, Overqueue, identity, tenant, key, registry, command, and audit primitives that Overstore consumes. |
| [Phase 3: Private Execution Loop](phase_03_private_execution_loop.md) | Supplies Overpack package refs, Overrun output/log/artifact refs, Oversched locality facts, Overlease storage-node context, and Overmeter raw usage paths that depend on object refs. |
| [Phase 4: Trust, Policy, and Verification](phase_04_trust_policy_verification.md) | Supplies Overguard data-class, access, egress, retention, replication, placement, challenge, trust, dispute, and provider eligibility policies consumed by Overstore. |
| [Phase 5: Metering, ORU, Seal Ledger, and Overbill](phase_05_metering_oru_seal_ledger_overbill.md) | Consumes Overstore raw usage through Overmeter while keeping rollups, pricing, ledger mutation, billing, settlement, payout, and external payment rails outside Overstore. |
| [Phase 7: Grid-Resident Backbone](phase_07_grid_resident_backbone.md) | Supplies protected system-service operation, backup/restore service patterns, failover/recovery expectations, and grid-resident storage-node readiness prerequisites. |
| [Phase 8: Data, Storage, and Namespace Platform](phase_08_data_storage_namespace_platform.md) | Controls Overstore's first build point as the object/artifact member of the native data/storage/namespace platform. |
| [Phase 9: Overpack Deployment Platform](phase_09_overpack_deployment_platform.md) | Supplies package, app, release, rollback, health, and deployment-artifact refs that Overstore persists without owning manifest semantics. |
| [Phase 10: Trusted Federation and Public-Interest Pools](phase_10_trusted_federation_public_interest_pools.md) | Adds trusted federation and public-interest storage scopes, purpose tags, grant refs, and partner storage handoff without weakening integrity or data-class controls. |
| [Phase 11: Limited Public Low-Sensitivity Pool](phase_11_limited_public_low_sensitivity_pool.md) | Adds untrusted public-provider storage only for eligible low-sensitivity or public objects with signed leases, challenge evidence, fraud controls, payout holds, and redaction. |
| [Phase 12: Native Application Layer](phase_12_native_application_layer.md) | Consumes Overstore object refs for workspace files, media, attachments, maps data, search assets, personal AI artifacts, datasets, backups, and native app assets. |
| [Phase 13: Governance, Compliance, and Scale Hardening](phase_13_governance_compliance_scale_hardening.md) | Supplies retention, compliance hold, incident, threat-model, migration, audit-export, PIP, and governance hardening for object contracts, repair, deletion, and provider proof. |
| [Build-plan crosswalk](service_catalog_alignment.md) | Keeps SDS #28 first build work aligned to master Phase 8, with prerequisites from earlier phases and later federation/public-provider, native-app, and governance gates. |
| [Tech stack decision](../overrid_tech_stack_choice.md) | Requires Rust-first core services, Tokio, Axum/Tower/Hyper-style HTTP where a service boundary is needed, canonical JSON plus JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, native Overrid service boundaries, and no conventional object-store product boundary. |

## Master Phase Mapping

| Sub-build phase | Primary master phase alignment | Reason |
| --- | --- | --- |
| 1 | Master Phases 0, 1, 3, 4, 5, 7, 8, 10, 11, 12, and 13 | Attach SDS #28, freeze Overstore scope, preserve Phase 8 as first build point, and record prerequisite plus later provider/native/governance gates. |
| 2 | Master Phases 0, 1, 7, and 8 | Build Rust contracts, JSON Schemas, local shard abstraction, fixture sets, and deterministic harnesses before object side effects. |
| 3 | Master Phases 1, 4, and 8 | Implement upload sessions, chunk receipt, BLAKE3 validation, commit, idempotency, and append-only object/version/chunk records. |
| 4 | Master Phases 4, 8, and 12 | Add policy-bound metadata reads, privacy-preserving listings, download grants, range reads, and redaction for native app consumption. |
| 5 | Master Phases 4, 8, and 13 | Add encryption-before-placement, Overvault key/envelope refs, policy admission, scoped dedup, and protected metadata behavior. |
| 6 | Master Phases 7, 8, 10, 11, and 13 | Add placement, replication-first redundancy, erasure-profile schema gates, degradation handling, and repair jobs. |
| 7 | Master Phases 3, 4, 8, 9, 10, and 11 | Add transfer sessions over Overmesh-compatible routes and storage-node handoffs for artifacts, backups, federation, and later public pools. |
| 8 | Master Phases 4, 8, 11, and 13 | Add verification, challenge hooks, quarantine, retention, tombstones, expiration, purge, disputes, and compliance evidence. |
| 9 | Master Phases 3, 6, 8, 9, 12, and 13 | Connect Overrun, Overpack, Overbase, Overvault, native apps, AI/RAG, SDK/CLI/admin reads, backup/restore, and operations dashboards. |
| 10 | Master Phase 0 through Phase 13 | Validate structure, links, tech-stack alignment, queue state, progress evidence, and implementation handoff gates. |

## Tech Stack Guardrails

- Overstore core is a Rust service/module using shared contract types, Tokio for async transfer/repair workers, and Axum/Tower/Hyper-style HTTP only where an internal service boundary is needed.
- Overstore contracts use canonical JSON plus JSON Schema for docs-facing examples, fixtures, upload intents, grants, transfer sessions, events, reason codes, and replay reports. Compact Protobuf contracts may be added only through the shared contract layer where needed.
- Mutating calls use signed envelopes, tenant/app context, actor or service-account identity, idempotency keys, trace ids, schema versions, stable reason codes, policy refs, and append-only audit events.
- Ed25519 is used where signatures are required. BLAKE3/content hashes are used for object roots, chunks, manifests, artifact refs, repair evidence, transfer evidence, challenge commitments, snapshots, and replay bundles.
- The v0 founder-hardware backend is a native Overstore local shard: content-addressed chunk files on directly attached storage plus append-only object/version/chunk metadata behind the Overrid storage abstraction.
- S3, MinIO, Ceph, cloud object storage, PostgreSQL, MySQL, MongoDB, DynamoDB, Redis, NATS, Kafka, RabbitMQ, Vault, cloud KMS, Kubernetes-first orchestration, blockchain, NFT, pricing, revenue, customer-count, or external-payment assumptions must not become Overstore's product boundary.
- Overstore must not store structured app state, raw secret values, private vault payload semantics, package manifest semantics, human-readable namespace ownership, route ownership, Overasset rights records, ORU balances, Seal Ledger entries, invoices, payouts, or billing state.
- Deduplication is allowed only inside an equivalent policy scope unless an object is public or explicitly shared and policy permits content-hash disclosure.

## Phase 1: SDS Attachment, Object Scope, And Phase Gates

### Work Items

- **1.1 Attach the build plan to SDS #28.**
  - Design: Link this document from the numbered Overstore SDS, service plan, master build plan, and build-plan crosswalk so builders can move from SDS scope to implementation order without guessing.
  - Output: Stable links between this file, `docs/sds/data_storage_namespace/overstore.md`, `docs/service_catalog/data_storage_namespace/overstore.md`, `docs/build_plan/master_plan.md`, and `docs/build_plan/service_catalog_alignment.md`.
  - Validation: Markdown link checks pass and Docdex search for SDS #28 returns both the Overstore SDS and this sub-build plan.

- **1.2 Freeze Overstore as the object/artifact persistence authority.**
  - Design: Record that Overstore owns object ids, versions, chunk maps, hashes, upload sessions, download grants, transfer sessions, placement records, replica sets, repair jobs, verification events, retention records, quarantine records, tombstones, and storage usage facts.
  - Output: Ownership checklist for implementation reviews.
  - Validation: Architecture review confirms Overstore does not own structured app state, raw secrets, namespace ownership, package semantics, policy authority, accounting settlement, or direct billing mutation.

- **1.3 Preserve master Phase 8 as the first build point.**
  - Design: Keep first implementation in master Phase 8 because Overstore depends on signed identity/tenant/request/audit rails, policy controls, usage emission, and protected grid/storage readiness.
  - Output: Phase-gate note that Phases 0, 1, 3, 4, 5, and 7 are prerequisites and Phase 8 proves native object persistence before broader native app consumption.
  - Validation: Review proves this plan does not move Overstore into earlier master phases and does not change the master Phase 0 through Phase 13 order.

- **1.4 Carry forward resolved SDS decisions.**
  - Design: Preserve SDS #28 decisions for a native local shard backend, replication with verified repair as the default pre-federation redundancy profile, privacy-preserving listings, signed storage leases plus challenge proofs for untrusted providers, and policy-scoped deduplication.
  - Output: Resolved-decision checklist tied to SDS #28 open-question answers.
  - Validation: Review rejects S3/MinIO/Ceph/cloud object-store boundaries, mandatory erasure coding before enough independent same-trust targets, global visible private dedup, provider metadata as proof of storage, or exact private placement/hash exposure in user listings.

- **1.5 Define object authority boundaries.**
  - Design: Create a boundary matrix for Overtenant, Overpass, Overkey, Overgate, Overguard, Overwatch, Overmeter, Overvault, Overbase, Overpack, Overrun, Overmesh, Universal Namespace Service, Overasset, Challenge Task Service, Oververify, Overclaim, native apps, SDK, CLI, admin UI, and governance services.
  - Output: Boundary matrix listing consumed refs, emitted refs, denial behavior, retry owner, redaction profile, and replay evidence for each dependency.
  - Validation: Review confirms every dependency has an owning service and Overstore receives private facts through explicit APIs/refs rather than direct storage access.

## Phase 2: Rust Contracts, Schemas, Local Shard, And Fixtures

### Work Items

- **2.1 Create the Overstore Rust contract module.**
  - Design: Add contract types for object records, object versions, chunk records, upload sessions, download grants, transfer sessions, placement records, replica sets, repair jobs, verification events, retention records, quarantine records, tombstones, usage refs, and reason codes.
  - Output: Rust module/crate skeleton, serde bindings, schema-version enums, state enums, API error types, reason-code mapping, and test entry points.
  - Validation: Compile checks pass once implementation exists and crate layout review confirms Overstore contracts remain separate from Overbase, Overvault, Overpack, namespace, policy, trust, and accounting logic.

- **2.2 Define canonical JSON Schemas and examples.**
  - Design: Add versioned schemas for upload intent, chunk receipt, commit report, object metadata view, download grant, range read request, transfer session, placement record, replica set, repair job, verification event, retention update, quarantine, tombstone, purge report, and API errors.
  - Output: Schema files, valid examples, invalid examples, schema index, generated validators, and compatibility notes.
  - Validation: Schema tests reject missing tenant, app, actor/service account, trace, idempotency, data class, policy refs, expected hash, chunk hash, object id, version id, grant, and audit refs where required.

- **2.3 Define the native local shard abstraction.**
  - Design: Model content-addressed chunk files on directly attached storage, append-only object/version/chunk metadata, shard ids, storage root refs, deterministic reset/seed, scan cursors, fsync/checkpoint strategy, and recovery hooks without exposing the internal layout as the product API.
  - Output: Local shard traits, storage abstraction, chunk file layout contract, metadata store abstraction, recovery scanner, benchmark hooks, and fixture backend.
  - Validation: Boundary tests prove public API contracts do not expose host paths, S3-style buckets, MinIO/Ceph concepts, or internal engine details.

- **2.4 Define object lifecycle and state transitions.**
  - Design: Model declared, uploading, committing, available, replicating, degraded, repairing, quarantined, retained, expiring, tombstoned, and purged states as append-only version/tombstone history.
  - Output: State transition table, legal transition rules, terminal/overlay state semantics, reason codes, and event payload refs.
  - Validation: State tests reject available before root hash verification, silent deletion, purge before retention eligibility, normal reads from quarantined objects, and metadata-only integrity claims.

- **2.5 Create deterministic fixtures and harness scenarios.**
  - Design: Build fixtures for valid upload/commit, corrupt chunk, duplicate chunk, out-of-order chunks, root hash mismatch, policy denial, private encrypted object, download grant, range read, transfer retry, replica degradation, repair, quarantine, retention block, tombstone, purge, and scoped dedup.
  - Output: Fixture directory, expected API responses, object records, chunk records, events, usage facts, audit refs, and replay hashes.
  - Validation: Fixture tests produce stable output and prove ambiguous or missing facts fail closed as denied, blocked, degraded, or quarantined states.

## Phase 3: Upload Sessions, Chunking, Hashing, And Commit

### Work Items

- **3.1 Implement upload session creation.**
  - Design: Accept object intent, expected size, expected root hash, object kind, media type, tenant/app/owner refs, data class, retention policy, encryption refs where required, policy refs, idempotency key, and expiration.
  - Output: `POST /overstore/uploads`, upload session record, duplicate handling, allowed chunk profile, and `overstore.upload_started` event.
  - Validation: Upload tests reject missing tenant/app refs, unsupported data class, invalid retention, missing expected hash, unauthorized actor, incompatible policy refs, and duplicate incompatible idempotency keys.

- **3.2 Implement chunk receipt and validation.**
  - Design: Receive chunks by session id and chunk index with byte range, chunk hash, size, compression flag, encryption envelope ref, content stream hash, and resumability cursor.
  - Output: `PUT /overstore/uploads/{session_id}/chunks/{chunk_index}`, chunk record, content-addressed local write, resumability state, and `overstore.chunk_received` event.
  - Validation: Chunk tests reject mismatched hash, wrong byte range, duplicate conflicting chunk, oversized chunk, expired session, wrong tenant/app, missing envelope refs where required, and path escape attempts.

- **3.3 Implement root hash commit.**
  - Design: Verify all required chunks, order, byte ranges, chunk hashes, expected size, root BLAKE3 hash, encryption requirements, object policy, and idempotent commit behavior before object availability.
  - Output: `POST /overstore/uploads/{session_id}/commit`, object record, object version, chunk map, commit report, and `overstore.object_committed` event.
  - Validation: Commit tests reject missing chunks, out-of-order coverage gaps, root hash mismatch, policy denial, quarantine-required content, and commit attempts that would create available metadata without verified bytes.

- **3.4 Implement version and manifest refs.**
  - Design: Support immutable object versions, previous-version refs, manifest refs, rollback eligibility, package/artifact refs, dataset/model refs, snapshot refs, and compatibility refs without owning package manifest semantics.
  - Output: Object version schema, version read model, manifest-ref validation, version pointer update contract, and compatibility notes.
  - Validation: Version tests prove normal reads only move to a new version through explicit pointer/metadata update and registered Overpack manifests remain Overpack-owned.

- **3.5 Emit upload and commit evidence.**
  - Design: Emit raw usage for uploaded bytes, committed bytes, rejected bytes, chunk validation work, hash verification, local shard writes, failed commits, and duplicate/idempotent retries without billing semantics.
  - Output: Overmeter raw usage event contract, Overwatch audit mapping, source refs, replay bundle fields, and redacted diagnostics.
  - Validation: Evidence tests prove Overstore never mutates ORU balances, Seal Ledger entries, invoices, pricing, payouts, or settlement state.

## Phase 4: Metadata Reads, Download Grants, Range Reads, And Redaction

### Work Items

- **4.1 Implement redacted object metadata reads.**
  - Design: Read object metadata through tenant/app/data-class/access policy with views for owner, service account, operator, audit, public, and unauthorized callers.
  - Output: `GET /overstore/objects/{object_id}`, redacted metadata view, safe not-found/denied semantics, pagination fields, and audit refs.
  - Validation: Metadata tests prove private listings do not expose content hashes, chunk ids, exact byte layout, storage paths, placement refs, provider/node ids, replica health, encryption/key refs, private route refs, or cross-tenant existence signals.

- **4.2 Implement bounded download grants.**
  - Design: Create grants with object/version refs, actor or service-account refs, allowed byte ranges, ttl, data-class policy refs, redaction policy, grant status, trace id, and audit refs.
  - Output: `POST /overstore/objects/{object_id}/download-grants`, download grant record, grant lifecycle, denial reasons, and `overstore.download_grant_issued` event.
  - Validation: Grant tests reject unauthorized actors, missing data-class refs, expired object versions, quarantined objects, retention-hold conflicts where policy blocks reads, and overbroad ranges.

- **4.3 Implement grant-bound range reads.**
  - Design: Serve authorized byte ranges through a valid grant with version pinning, range bounds, hash verification, retry cursor, transfer usage refs, and redacted error behavior.
  - Output: `GET /overstore/objects/{object_id}/versions/{version_id}/content`, range response, verification refs, bandwidth usage event, and read audit record.
  - Validation: Range tests reject missing/expired/revoked grants, mismatched object/version, range escape, cross-tenant reads, corrupted chunks, and direct object reads without grants.

- **4.4 Implement privacy-preserving listings.**
  - Design: Provide user/app listings that show only caller-authorized display refs, object kind, version label, state, coarse size bucket, coarse created/updated time, retention/hold state, data-class label where policy allows, and explicit grant status.
  - Output: Listing API/read model, redaction profile, user/operator/audit field matrix, and safe pagination.
  - Validation: Listing tests prove exact hashes, chunk ids, placement, provider ids, encryption refs, route refs, and inferred private media details stay hidden from ordinary user-facing listings.

- **4.5 Emit read and grant evidence.**
  - Design: Emit raw usage for metadata reads, grant creation, range reads, bytes downloaded, denied reads, expired grants, corrupt-read detection, and redaction decisions where useful.
  - Output: Usage/audit event mapping, redacted diagnostics, grant replay report, and source-ref bundle.
  - Validation: Evidence tests prove read logs and diagnostics can support disputes without exposing private object bytes, raw secrets, or provider-private topology.

## Phase 5: Encryption, Policy Admission, And Scoped Deduplication

### Work Items

- **5.1 Enforce encryption-before-placement.**
  - Design: Require encryption envelope refs before bytes are written for private, secret-bearing, regulated, user-private, organization-private, and policy-marked object classes.
  - Output: Encryption admission checks, envelope ref fields, missing-envelope reason codes, commit gate, and encrypted-object fixtures.
  - Validation: Security tests prove policy-required encrypted objects cannot commit with plaintext placement and Overstore never exposes raw key material.

- **5.2 Integrate Overvault key and envelope refs.**
  - Design: Store encryption envelope refs, key policy refs, key status refs, rotation refs, and private-object handling metadata while delegating raw secret/key authority to Overvault.
  - Output: Overvault client contract, key-ref compatibility checks, read-block behavior on dependency outage, rotation evidence refs, and redacted diagnostics.
  - Validation: Dependency tests block encrypted reads when Overvault is unavailable or key refs are revoked, rather than serving encrypted material incorrectly or exposing key metadata.

- **5.3 Gate writes and reads through Overguard policy.**
  - Design: Call policy refs for data class, access, retention, replication, placement, dedup, transfer, quarantine, deletion, and public-provider eligibility before side effects.
  - Output: Policy-admission client, policy input bundle, deny/block reason mapping, audit refs, and remediation hints.
  - Validation: Policy tests prove missing policy refs deny or block instead of silently accepting permissive defaults.

- **5.4 Implement scoped deduplication.**
  - Design: Deduplicate only within equivalent tenant, app/service, data class, encryption policy, retention policy, and caller authorization scope unless the object is public or explicitly shared and policy permits hash disclosure.
  - Output: Dedup scope schema, equivalence checks, physical coalescing metadata, public-artifact dedup mode, and redacted user response behavior.
  - Validation: Dedup tests prove private, regulated, secret-bearing, user-private, and organization-private objects never leak `already exists` or hash signals across tenants or incompatible scopes.

- **5.5 Emit policy and encryption evidence.**
  - Design: Record policy decision refs, encryption envelope refs, key status refs, denial reasons, dedup scope decisions, redaction decisions, and retryable dependency outages without storing raw keys or private payloads.
  - Output: Overwatch evidence mapping, Overmeter diagnostic usage facts, policy replay refs, and redacted operator diagnostics.
  - Validation: Evidence tests prove security reviews can trace why data was accepted, denied, encrypted, deduped, blocked, or quarantined without exposing sensitive details.

## Phase 6: Placement, Replication, Erasure Profiles, And Repair

### Work Items

- **6.1 Implement placement records.**
  - Design: Track object/chunk id, storage node or provider ref, region/locality, trust scope, storage class, redundancy role, route refs where relevant, health state, policy refs, and audit refs.
  - Output: Placement record schema, placement write/read model, health updates, and placement events.
  - Validation: Placement tests reject unknown storage nodes, incompatible trust scope, missing policy refs, cross-tenant leakage, and user-facing exposure of private placement details.

- **6.2 Implement replication-first redundancy.**
  - Design: Support required replica counts, current healthy count, degraded count, repair target, verified copy behavior, and policy-controlled redundancy profiles before enabling erasure coding by default.
  - Output: Replica set record, replication worker, health read model, degraded state event, and repair trigger.
  - Validation: Replication tests prove pre-federation default redundancy is replication with verified repair and that metadata-only replica claims never count as healthy bytes.

- **6.3 Model erasure coding profiles behind explicit gates.**
  - Design: Add erasure-profile schema and cold-object readiness checks while enabling profiles such as `rs_4_2_private` only after at least six independent same-trust targets, restore tests, and Overwatch repair evidence exist.
  - Output: Erasure profile schema, disabled-by-default gate, readiness report, restore-test evidence requirements, and policy refs.
  - Validation: Gate tests reject public or cross-trust erasure shards before trusted federation/public-provider phases and reject erasure profiles without enough independent targets.

- **6.4 Implement repair jobs.**
  - Design: Schedule repair when integrity, redundancy, placement policy, or verification freshness falls below target, using source replica refs, destination refs, retry state, final verification, and evidence refs.
  - Output: `POST /overstore/objects/{object_id}/repair`, repair job record, repair worker, `overstore.repair_started`, `overstore.repair_completed`, and degraded-state resolution behavior.
  - Validation: Repair tests cover corrupt replica, missing replica, failed destination, policy-ineligible target, retry exhaustion, successful repair, and evidence replay.

- **6.5 Emit placement, replication, and repair usage.**
  - Design: Emit raw usage for bytes stored, replicated, repaired, verified, transferred for repair, and deleted from unhealthy replicas by data class, storage class, tenant/app, and node/provider refs.
  - Output: Overmeter usage contract, Overwatch evidence mapping, provider/node refs, and redacted diagnostics.
  - Validation: Usage tests prove repair/provider work can be audited without mutating payout state or revealing private object contents.

## Phase 7: Transfer Sessions, Overmesh Routes, And Storage-Node Handoff

### Work Items

- **7.1 Implement transfer session creation.**
  - Design: Create transfer sessions with source ref, destination ref, object/chunk refs, route refs, protocol profile, bandwidth limits, retry policy, data class, policy refs, chunk cursor, and expected final integrity result.
  - Output: `POST /overstore/transfers`, transfer session record, state machine, `overstore.transfer_started` event, and transfer fixtures.
  - Validation: Transfer tests reject missing route refs, unsupported protocol profiles, unauthorized destinations, incompatible data class, and transfer intents that bypass Overmesh policy.

- **7.2 Integrate Overmesh-compatible routes.**
  - Design: Resolve private, grid-resident, trusted federation, and later public-provider transfer paths through Overmesh refs without embedding private route topology in user-facing records.
  - Output: Overmesh client contract, route health input, retry/fallback rules, redacted route diagnostics, and route-ref evidence.
  - Validation: Route tests prove route failures preserve cursors, retry according to policy, and do not leak private node/provider topology to unauthorized callers.

- **7.3 Support storage-node handoff.**
  - Design: Use Overcell/Overlease storage-node context where provider storage is used, binding transfer sessions to storage-node capability, lease/context refs, trust scope, locality, and policy eligibility.
  - Output: Storage-node handoff contract, capability checks, lease/context refs, transfer admission report, and handoff events.
  - Validation: Handoff tests reject stale node state, wrong storage class, wrong tenant visibility, revoked provider eligibility, and unleased public-provider storage paths.

- **7.4 Support artifact, backup, and deployment transfer profiles.**
  - Design: Add transfer profiles for Overrun outputs/logs, Overpack package artifacts, model artifacts, datasets, snapshots, backups, and deployment bundles with compatible retention and verification requirements.
  - Output: Profile matrix, required refs, transfer examples, retry policies, and downstream owner notes.
  - Validation: Profile tests prove Overstore persists bytes while Overrun, Overpack, backup/restore, and deployment services retain semantic ownership of runs, manifests, release strategy, and restore workflows.

- **7.5 Emit transfer evidence.**
  - Design: Record transfer start/completion/failure, route refs, bandwidth dimensions, chunk cursor, retry count, final hash verification, source/destination refs, and redacted failure reasons.
  - Output: `overstore.transfer_completed` event, transfer usage facts, Overwatch evidence, replay bundle, and diagnostics.
  - Validation: Replay tests reconstruct failed, retried, cancelled, and successful transfers without exposing private object bytes or route internals.

## Phase 8: Verification, Challenge Hooks, Quarantine, And Retention

### Work Items

- **8.1 Implement object and chunk verification.**
  - Design: Verify root hash, chunk hashes, replica health, placement policy, verification freshness, and optional challenge evidence by object/version/chunk refs.
  - Output: `POST /overstore/objects/{object_id}/verify`, verification job, verification event, `overstore.object_verified`, and verification freshness read model.
  - Validation: Verification tests detect missing chunks, corrupt chunks, stale verification, mismatched root hash, unhealthy replicas, and metadata-only integrity claims.

- **8.2 Add challenge hooks for untrusted providers.**
  - Design: Use signed storage leases plus Oververify/Challenge Task Service evidence for nonce-bound random chunk or byte-range challenges against BLAKE3 chunk/root commitments over encrypted bytes.
  - Output: Challenge request/response contract, provider proof refs, failure evidence, payout-hold refs for owning accounting services, and repair/quarantine triggers.
  - Validation: Challenge tests prove provider metadata alone is never proof of storage and challenge failures feed repair, quarantine, dispute, and payout-hold decisions through owning services.

- **8.3 Implement quarantine workflows.**
  - Design: Quarantine corrupt, abusive, disputed, policy-denied, suspicious, or challenge-failed objects/chunks by restricting reads, recording detected-by refs, remediation state, and Overclaim refs.
  - Output: Quarantine record, `overstore.object_quarantined` event, grant override behavior, remediation workflow, and appeal/dispute refs.
  - Validation: Quarantine tests prove quarantine overrides normal read grants until cleared by policy and preserves evidence for repair, dispute, incident, or governance review.

- **8.4 Implement retention, tombstone, expiration, and purge.**
  - Design: Apply retain-until, expiration action, compliance/legal hold refs, deletion eligibility, tombstone refs, cleanup state, purge evidence, and policy-checked administrative actions.
  - Output: `POST /overstore/objects/{object_id}/retention`, `DELETE /overstore/objects/{object_id}`, retention record, tombstone record, purge workflow, and `overstore.object_tombstoned` or `overstore.object_deleted` events.
  - Validation: Retention tests block deletion under hold, create tombstones instead of silent deletion, preserve audit evidence, and purge bytes only where policy allows.

- **8.5 Emit verification, quarantine, and retention evidence.**
  - Design: Emit raw usage and audit events for verification work, challenge work, quarantine, repair triggers, retention updates, tombstone creation, expiration, purge attempts, and deletion completion.
  - Output: Overmeter usage mapping, Overwatch evidence mapping, dispute/governance refs, and redacted diagnostics.
  - Validation: Evidence tests prove compliance, incident, and dispute workflows can audit object lifecycle without exposing private bytes, exact layouts, raw secrets, or unauthorized hashes.

## Phase 9: Product, Native App, Backup, And Operations Integration

### Work Items

- **9.1 Integrate Overrun and Overpack artifact flows.**
  - Design: Support Overrun outputs/logs/diagnostic bundles and Overpack package/deployment artifacts through object refs, content hashes, retention policy, data class, and verification evidence.
  - Output: Artifact handoff contract, object-ref profile, manifest/ref compatibility report, and fixture set.
  - Validation: Integration tests prove Overstore stores bytes while Overrun owns run semantics and Overpack owns package/manifest semantics.

- **9.2 Integrate Overbase, Overvault, and namespace refs.**
  - Design: Support Overbase large-payload spillover, backups, exports, snapshots, Overvault encrypted payload object refs, and optional Universal Namespace display/route bindings without moving ownership into Overstore.
  - Output: Ref compatibility matrix, handoff schemas, redacted metadata views, and owner-service notes.
  - Validation: Handoff tests prove structured state remains in Overbase, raw secrets/private encrypted records remain in Overvault, and names/routes remain in the Universal Namespace Service.

- **9.3 Support native app and AI/RAG object consumers.**
  - Design: Provide object flows for workspace files, media, attachments, maps data, search assets, personal AI artifacts, encrypted Docdex RAG indexes, datasets, model artifacts, public assets, and research outputs.
  - Output: Native-app object profiles, AI/RAG storage profile, grant/read examples, retention defaults, and redaction notes.
  - Validation: Native-app tests prove apps use download grants, object refs, and data-class policy instead of direct file paths or hidden external object stores.

- **9.4 Add operations dashboards and health checks.**
  - Design: Publish health for local shard access, chunk hash verification, policy dependency, Overvault dependency, Overmesh transfer dependency, Overwatch emission, Overmeter emission, replica health, repair backlog, quarantine count, retention backlog, and purge backlog.
  - Output: Health endpoint/schema, metrics/events, dashboard fields, safe diagnostics, and operator follow-up refs.
  - Validation: Operations tests prove diagnostics include evidence refs and reason codes but not private object bytes, key refs outside authority, exact private placement, or provider-private topology.

- **9.5 Prepare backup/restore and migration handoffs.**
  - Design: Document how Backup and Restore Service, Deployment Planner, Release Strategy Service, Package Validator, native apps, adapters, and governance services consume object refs, snapshots, artifacts, transfer refs, verification refs, repair refs, retention refs, and tombstone refs.
  - Output: Handoff matrix, dependency owners, ref lifecycles, rollback/restore notes, and phased readiness notes.
  - Validation: Handoff review confirms downstream consumers can use Overstore without weakening Overbase, Overvault, Overpack, namespace, Overmeter, Overwatch, Overguard, or governance boundaries.

## Phase 10: Validation, Documentation, Queue State, And Handoff

### Work Items

- **10.1 Validate contract and schema coverage.**
  - Design: Run focused checks for object records, object versions, chunks, upload sessions, download grants, transfer sessions, placement records, replica sets, repair jobs, verification events, retention records, quarantine records, tombstones, APIs, events, and reason codes.
  - Output: Schema-test report, state-machine test report, fixture coverage matrix, failure notes, and remediation list.
  - Validation: Tests pass before implementation advances beyond each documented gate; any blocker is recorded in build-plan progress.

- **10.2 Validate Phase 8 object persistence end to end.**
  - Design: Prove one signed tenant/app flow creates an upload session, writes chunks, commits an object version, creates a grant, performs a range read, records placement, verifies hashes, schedules repair when degraded, and emits usage/audit evidence.
  - Output: End-to-end object fixture, source-ref bundle, operation records, usage ref, audit trail, and replay report.
  - Validation: Replay confirms successful, denied, blocked, corrupt, degraded, quarantined, retention-blocked, tombstoned, and purge-failed paths produce distinct auditable states.

- **10.3 Validate security, privacy, and tech-stack alignment.**
  - Design: Scan implementation and docs for raw private payload leakage, raw secret storage, unauthorized hash exposure, exact private placement exposure, direct dependency storage reads, namespace ownership drift, package-semantics drift, billing/ledger mutation, conventional object-store product boundaries, blockchain/NFT mechanics, pricing/revenue/customer-count assumptions, and TypeScript core-runtime drift.
  - Output: Security/privacy checklist, tech-stack alignment report, negative-control scan results, and remediation notes.
  - Validation: Review confirms Overstore remains Rust-first/native-Overrid infrastructure and uses canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, policy refs, and native service boundaries.

- **10.4 Validate master-plan and downstream handoff alignment.**
  - Design: Confirm SDS #28, the Overstore service plan, master build plan, build-plan crosswalk, Phase 8 plan, Phase 9 plan, Phase 11 plan, Phase 12 plan, Phase 13 plan, queue state, and progress docs link to this plan and preserve the Phase 8 first build point.
  - Output: Updated source-document links, sub-build-plan index entries, progress evidence, queue status, and downstream handoff matrix.
  - Validation: Markdown link checks pass and review confirms no master Phase 0 through Phase 13 ordering change was required.

- **10.5 Validate implementation handoff readiness.**
  - Design: Prepare the handoff for builders by listing required crates/modules, schemas, reason-code catalogs, local shard abstraction, service clients, fixture groups, transfer harness scenarios, acceptance tests, and phase gates.
  - Output: Implementation handoff checklist, validation command list, known blockers, dependency owners, and first object-backed app fixture target.
  - Validation: Handoff review confirms a builder can start Overstore Phase 8 implementation without reading informal agent notes or weakening SDS boundaries.

## Alignment Review

- The sub-build plan keeps Overstore first build work in master Phase 8, matching SDS #28, the service catalog entry, Phase 8 plan, master build plan, and build-plan crosswalk.
- The plan treats Phases 0, 1, 3, 4, 5, and 7 as prerequisites for shared schemas, local fixtures, signed identity/tenant/request/audit rails, package/run artifact refs, policy controls, usage emission, and grid-resident storage readiness rather than as Overstore's first implementation phase.
- The plan keeps Overbase, Overvault, Overpack, Universal Namespace Service, Overmesh, Overasset, Overmeter, Overwatch, Overguard, Oververify, Challenge Task Service, Overclaim, native apps, and governance authority outside Overstore while defining the refs and evidence Overstore consumes or emits.
- The plan preserves SDS #28 v0 storage decisions: native local shard backend, BLAKE3 root/chunk hashes, append-only metadata behind an Overrid storage abstraction, replication with verified repair as the pre-federation default, erasure profiles gated behind enough same-trust targets, privacy-preserving listings, challenge evidence for untrusted providers, and policy-scoped dedup.
- The plan gates public-provider storage behind Phase 11 low-sensitivity/public-provider constraints with signed storage leases, nonce-bound challenge proofs, fraud controls, dispute refs, payout-hold refs owned by accounting services, and repair/quarantine evidence.
- The plan keeps long-running transfer, replication, repair, verification, challenge, retention, purge, migration, and backup/restore work in queued or resumable jobs instead of ordinary request budgets.
- The plan preserves the master Phase 0 through Phase 13 order and uses later phases only for deployment artifact flows, federation/public-provider expansion, native-app consumption, and governance/compliance hardening.
- The plan aligns with `docs/overrid_tech_stack_choice.md`: Rust-first Overstore core, native Overrid boundaries, Tokio, Axum/Tower/Hyper-style service boundaries, canonical JSON/JSON Schema, optional Protobuf, Ed25519 signatures, BLAKE3/content hashes, signed envelopes, native local shard abstraction, and no conventional database, object-store, vault, queue, blockchain, NFT, pricing, revenue, customer-count, or external-payment assumptions.
