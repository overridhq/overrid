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

## Validation

- Revoked credentials cannot authenticate.
- Rotated credentials preserve audit history.
- Services verify signatures from key records rather than hardcoded secrets.

## Handoff

Overkey supports Overgate, Overrun secret mounts, Overvault access, SDK, CLI, and signed operator actions.
