# Overkey

Overkey is the Overrid credential and key authority service skeleton for SUB BUILD PLAN #9 Phase 2. It is a Rust-first service crate using Tokio, Axum, Tower, Hyper-compatible serving, tracing, canonical JSON envelopes, JSON Schema references, and local Overrid-shaped fixtures.

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

## Phase 2 Boundaries

- Credential metadata is recorded through `CredentialMetadataRepository`.
- The default local implementation is an append-friendly in-memory stub.
- Duplicate credential appends fail with `overkey.duplicate_credential_rejected` instead of replacing stored record identity.
- Records store `secret_ref` handles and hash references, never direct key material.
- Verification routes are internal-only skeletons guarded by service-account headers.
- Readiness reports schema validation, credential metadata storage, Overgate, Overpass, Overtenant, Overwatch, and Overvault stub dependencies.

## Fixtures

- Valid local fixture: `fixtures/valid/phase2_local_credential.valid.json`
- Invalid denial fixture: `fixtures/invalid/phase2_raw_secret.invalid.json`
- Schema source: `packages/schemas/overrid_contracts/v0/overkey_credential.schema.json`

## Validation

- `cargo test -p overrid-overkey`
- `python3 scripts/validate_overkey_phase2.py`
