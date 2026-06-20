# Overstore Implementation Plan

## Objective

Build durable content-addressed persistence for files, media, packages, datasets, models, snapshots, backups, and research outputs.

## First Build Phase

[Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).

## Dependencies

- Overvault encryption policy.
- Overmesh transfer paths.
- Overpack artifact manifests.
- Overguard data policy.
- Overbase metadata refs, backup refs, and large-payload spillover refs.
- Overcell and Overlease storage-node context where provider storage is used.
- Overwatch integrity, audit, challenge, quarantine, repair, retention, and deletion evidence.
- Overmeter raw storage, bandwidth, replication, verification, repair, and deletion usage events.

## Development Order

1. Implement content addressing, chunking, checksums, and object metadata.
2. Add encryption-before-placement and access policy refs.
3. Add policy-bound download grants, range reads, and privacy-preserving listings.
4. Add replication-first redundancy and model erasure-coding profiles behind readiness gates.
5. Add transfer sessions, placement records, and repair jobs when redundancy falls below policy.
6. Add proof or challenge hooks for untrusted providers.
7. Add retention, quarantine, tombstone, expiration, and purge workflows.

## Contracts And Interfaces

- Object record schema.
- Chunk metadata.
- Storage lease refs.
- Transfer API.
- Repair and verification events.

## Detailed SDS

The detailed design contract lives in [Overstore SDS](../../sds/data_storage_namespace/overstore.md).

- [SUB BUILD PLAN #28 - Overstore](../../build_plan/sub_build_plan_028_overstore.md)

## Design Alignment

- Treat Overstore as content-addressed object persistence for files, media, packages, datasets, model artifacts, snapshots, backups, logs, and research outputs.
- Keep structured app state in Overbase, secret/private access in Overvault, package semantics in Overpack, and names/routes in the Universal Namespace Service.
- Verify chunks and root hashes before commit, track placement/replica/repair records, and enforce encryption-before-placement where policy requires it.
- Use bounded download grants, transfer sessions, retention records, quarantine records, tombstones, and usage events instead of raw ad hoc file access.
- Use a native Overstore local shard backend for founder-hardware v0; S3, MinIO, Ceph, or cloud object stores may be compared or bridged later but must not become the product boundary.
- Default pre-federation redundancy to replication with verified repair; erasure-coding profiles remain schema-modeled and readiness-gated until enough independent same-trust storage targets and restore evidence exist.
- Scope deduplication by tenant, app/service, data class, encryption policy, retention policy, and caller authorization unless an object is public or explicitly shared and policy permits hash disclosure.

## Validation

- Objects can be stored, retrieved, verified, and repaired.
- Corrupt chunks are detected.
- Access policy prevents unauthorized reads.
- Private listings do not expose content hashes, chunk ids, exact byte layouts, storage paths, placement refs, provider/node ids, replica health, encryption/key refs, private route refs, or cross-tenant existence signals.
- Untrusted providers require signed storage leases plus nonce-bound challenge evidence before eligibility.

## Handoff

Overstore backs Overrun artifacts, Overpack packages, Docdex indexes, workspace files, media, datasets, backups, and native apps.
