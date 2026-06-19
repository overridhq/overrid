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

## Development Order

1. Implement content addressing, chunking, checksums, and object metadata.
2. Add encryption-before-placement and access policy refs.
3. Add replication or erasure coding.
4. Add repair jobs when redundancy falls below policy.
5. Add proof or challenge hooks for untrusted providers.

## Contracts And Interfaces

- Object record schema.
- Chunk metadata.
- Storage lease refs.
- Transfer API.
- Repair and verification events.

## Detailed SDS

The detailed design contract lives in [Overstore SDS](../../sds/data_storage_namespace/overstore.md).

## Design Alignment

- Treat Overstore as content-addressed object persistence for files, media, packages, datasets, model artifacts, snapshots, backups, logs, and research outputs.
- Keep structured app state in Overbase, secret/private access in Overvault, package semantics in Overpack, and names/routes in the Universal Namespace Service.
- Verify chunks and root hashes before commit, track placement/replica/repair records, and enforce encryption-before-placement where policy requires it.
- Use bounded download grants, transfer sessions, retention records, quarantine records, tombstones, and usage events instead of raw ad hoc file access.

## Validation

- Objects can be stored, retrieved, verified, and repaired.
- Corrupt chunks are detected.
- Access policy prevents unauthorized reads.

## Handoff

Overstore backs Overrun artifacts, Overpack packages, Docdex indexes, workspace files, media, datasets, backups, and native apps.
