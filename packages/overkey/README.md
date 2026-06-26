# Overkey

Overkey is the Overrid credential and key authority service for SUB BUILD PLAN #9. Its validator-compatible baseline covers Phase 2 through Phase 4, and its current documented coverage extends Phase 2 through Phase 5 with lifecycle hardening layered on top. It is a Rust-first service crate using Tokio, Axum, Tower, Hyper-compatible serving, tracing, canonical JSON envelopes, JSON Schema references, BLAKE3 hash refs, and local Overrid-shaped fixtures.

## Local Surface

- Service id: `service:overkey`
- Local port owner: `service:api`
- Bind address: `127.0.0.1:18080`
- Base path: `/overkey`
- Run command: `cargo run -p overrid-overkey --bin overkey`

## Routes

- `POST /v1/credentials/api-keys`
- `POST /v1/credentials/signing-keys`
- `POST /v1/credentials/service-accounts`
- `POST /v1/credentials/{credential_id}/rotate`
- `POST /v1/credentials/{credential_id}/revoke`
- `GET /v1/credentials/{credential_id}`
- `POST /v1/verify/signature`
- `POST /v1/verify/api-key`
- `POST /v1/usage/last-used`
- `GET /v1/healthz`
- `GET /v1/readyz`

## Phase 2, Phase 3, Phase 4, And Phase 5 Boundaries

- Credential metadata is recorded through `CredentialMetadataRepository`.
- The default local implementation is an append-friendly in-memory stub.
- Duplicate credential appends fail with `overkey.duplicate_credential_rejected` instead of replacing stored record identity.
- Duplicate active tenant-scoped key ids fail with `overkey.duplicate_active_key_rejected`.
- Records store `secret_ref` handles and hash references, never direct key material.
- API key enrollment stores non-secret prefixes and BLAKE3 hash refs while discarding raw input immediately.
- Signing public key enrollment requires Ed25519, `overrid.canonical_json.v0`, expiry metadata, protection-class metadata, and tenant-scoped key ids.
- Service-account credential enrollment requires signed internal-service headers and narrow allowed-service/command-class scopes.
- Metadata reads are tenant-scoped and return redacted lifecycle metadata, last-used summaries, rotation refs, revocation refs, and safe protection-class labels.
- Lifecycle transitions are append-only and reject invalid resurrection from revoked or tombstoned states.
- Verification routes are internal-only Phase 4 helpers guarded by signed service-account headers and approved internal service-account refs.
- Signature verification checks credential id, key id, key version, Ed25519, canonicalization, timestamp, replay-window marker, tenant scope, subject refs, allowed use, command class, status, expiry, revocation epoch, and dependency state hints before returning an Overgate-facing result.
- API-key verification checks prefix, BLAKE3 keyed lookup hash refs, tenant scope, subject refs, allowed use, command class, status, expiry, revocation epoch, and dependency state hints without returning raw key material.
- Verification results use `overkey.phase4.response.v0`, stable `auth.*` reason codes, BLAKE3 request/evidence/cache refs, revocation epoch, retryability, cache guidance, audit refs, and redaction markers.
- Rotation responses use `overkey.phase5.response.v0` and append `rotation_record` data with predecessor/successor credential refs, grace windows, activation timing, initiated-by refs, evidence refs, revocation epoch bumps, cache invalidation, propagation status, and affected inventory.
- Revocation responses use `overkey.phase5.response.v0` and append `revocation_record` data with revoked-by refs, effective times, affected command classes, incident/evidence refs, expected current state, audit refs, revocation epoch bumps, cache invalidation, propagation status, and follow-up tasks.
- Signed break-glass revocation requires a signed Overgate service-account command, operator/admin role, high-risk protection class, evidence refs, and an idempotency key before accepting emergency revocation.
- Readiness reports schema validation, credential metadata storage, Overgate, Overpass, Overtenant, Overwatch, and Overvault stub dependencies.

## Fixtures

- Valid local fixture: `fixtures/valid/phase2_local_credential.valid.json`
- Valid Phase 3 fixture: `fixtures/valid/phase3_enrollment_metadata.valid.json`
- Valid Phase 4 fixture: `fixtures/valid/phase4_verification.valid.json`
- Valid Phase 5 fixture: `fixtures/valid/phase5_lifecycle.valid.json`
- Invalid denial fixture: `fixtures/invalid/phase2_raw_secret.invalid.json`
- Invalid Phase 3 denial fixture: `fixtures/invalid/phase3_raw_key_diagnostic.invalid.json`
- Invalid Phase 4 denial fixture: `fixtures/invalid/phase4_verification_denials.invalid.json`
- Invalid Phase 5 denial fixture: `fixtures/invalid/phase5_break_glass_denials.invalid.json`
- Schema source: `packages/schemas/overrid_contracts/v0/overkey_credential.schema.json`

## Validation

- `cargo test -p overrid-overkey`
- `python3 scripts/validate_overkey_phase2.py`
- `python3 scripts/validate_overkey_phase3.py`
- `python3 scripts/validate_overkey_phase4.py`
- `python3 scripts/validate_overkey_phase5.py`
