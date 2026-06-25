# Overkey Implementation Plan

## Objective

Manage API credentials, signing keys, delegated access, rotation, revocation, and secret references.

## First Build Phase

Overkey-lite in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); broader key services in [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).

## Dependencies

- Overpass identities.
- Overtenant scope.
- Overgate authentication.
- Overvault for protected secrets.

## Development Order

1. Implement API key and signing public key records.
2. Add credential enrollment and revocation.
3. Add rotation metadata.
4. Add service account key records.
5. Integrate secret refs and delegated access with Overvault.

## Contracts And Interfaces

- Key record schema.
- Credential enrollment API.
- Revocation API.
- Signature verification helper.
- Secret reference contract.

## Detailed SDS

- [Overkey SDS](../../sds/control_plane/overkey.md)

## Sub-Build Plan

- [SUB BUILD PLAN #9 - Overkey](../../build_plan/sub_build_plan_009_overkey.md)

## Design Alignment

- Treat Overkey as the credential and public verification metadata authority, not as the raw secret vault.
- Store API key hashes, public signing key metadata, service-account credential records, rotation links, revocation records, last-used metadata, and Overvault secret refs.
- Never persist raw private keys, seed phrases, bearer tokens, or unencrypted secret values.
- Provide restricted internal verification helpers for Overgate and signed administrative lifecycle APIs for enrollment, rotation, revocation, and secret-ref binding.
- Preserve append-only credential history so rotation and revocation remain auditable.

## Phase 1 Implementation Gates

| Gate | Phase 1 output | State |
| --- | --- | --- |
| Link attachment | SDS #9, this service catalog entry, the master plan, the build-plan crosswalk, SUB BUILD PLAN #9, and the Phase 1 planning/progress trail are linked. | `attached` |
| Credential metadata authority | Overkey owns credential records, API key hashes, public signing key metadata, service-account key records, delegated access metadata, rotation links, revocation records, last-used metadata, verification results, and secret-ref contracts. | `credential_metadata_authority_frozen` |
| Master phase ownership | First build point remains Phase 1, with Phase 0 prerequisites and broader key/secret-ref expansion through Phase 8 plus later policy, product, grid-resident, and governance hardening. | `master_phase_1_owned` |
| Resolved SDS decisions | Ed25519 command signatures, BLAKE3 canonical hashes and fingerprints, Explicit non-secret credential and key ids, Production protection classes, Short verification caches, Revocation epoch invalidation, and Signed break-glass revocation through Overgate are carried forward. | `resolved_decision_carried` |
| Runtime authority | Overkey owns public verification metadata while Overgate owns lifecycle command admission, Overvault owns raw secret custody, Overpass and Overtenant own identity and tenant scope, Overguard owns policy finality, and accounting services own settlement. | `overkey_owned` |

Overkey must not store raw private keys, seed phrases, passwords, bearer tokens, raw API keys, or unencrypted secret values. The forbidden set includes raw secret custody, business authorization, tenant membership, policy finality, request admission, accounting settlement, ORU mutation, Seal Ledger entries, and any runtime path that bypasses Overgate admission or approved internal service-account authentication; those records and decisions remain `downstream_owned` or `forbidden_in_overkey`.

Any new Overkey credential class, lifecycle state, allowed use, verification helper, service-account scope, delegation field, rotation or revocation rule, secret-ref contract, protection class, cache rule, break-glass path, event payload, or authority exception must update the shared schema package, the Overkey SDS, SUB BUILD PLAN #9, the owning downstream service plan, and the build-plan crosswalk before implementation.

## Validation

- Revoked credentials cannot authenticate.
- Rotated credentials preserve audit history.
- Services verify signatures from key records rather than hardcoded secrets.

## Handoff

Overkey supports Overgate, Overrun secret mounts, Overvault access, SDK, CLI, and signed operator actions.
