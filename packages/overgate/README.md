# Overrid Overgate

Overgate is the Rust-first API ingress and admission boundary for Overrid control-plane commands. This crate provides the Axum route surface, dependency-readiness model, admin authorization guard placeholders, and Phase 3 command-envelope validation needed before credential/signature admission.

## Local Entrypoint

- Service id: `service:overgate`
- Local-stack port owner: `service:api`
- Command: `cargo run -p overrid-overgate --bin overgate`
- Default bind: `127.0.0.1:18080`
- Local base path: `/overgate`
- Health: `GET /v1/healthz` and `GET /overgate/v1/healthz`
- Readiness: `GET /v1/readyz` and `GET /overgate/v1/readyz`

The existing Phase 0 local-stack `service:api` contract remains the reserved loopback API port owner. Overgate Phase 2 attaches the `service:overgate` route surface, `/overgate` base path, and fixture references to that loopback boundary without replacing earlier local-stack service ids.

## Public Routes

- `POST /v1/commands`
- `GET /v1/commands/{command_id}`
- `GET /v1/traces/{trace_id}`
- `GET /v1/limits`
- `POST /v1/policy/dry-run`
- `GET /v1/healthz`
- `GET /v1/readyz`

## Admin Routes

- `GET /v1/admin/ingress/{request_id}`
- `GET /v1/admin/idempotency/{tenant_id}/{idempotency_key}`
- `POST /v1/admin/idempotency/{record_id}/expire`
- `GET /v1/admin/rate-limits`

Admin route shells require placeholder signed-operator headers and deny unauthenticated, non-operator, and cross-tenant requests before returning data.

## Phase 3 Command Validation

`POST /v1/commands` now parses the body as a strict command envelope before any downstream side effect. The handler validates content type, size, required fields, supported shared-schema version, Overgate command type, payload hash refs, signature metadata, private command privacy class, and raw-secret sentinel markers.

Accepted command responses include:

- Shared schema adapter evidence: `overgate.phase3.shared_schema_adapter`
- Canonicalization version: `overgate.canonical.v0.1`
- Hash/ref-only retention policy: `overgate.phase3.hashes_and_refs_only`
- Forwarding state: `not_forwarded_phase3_validation_only`

Denied command responses use stable API error fields: `reason_code`, `trace_id`, `retryability`, `correction_fields`, `correction_hint`, dependency name when relevant, and redacted diagnostics. Private payloads, raw secrets, and credential material must not appear in error bodies.

## Fixtures

- `fixtures/valid/phase2_local_command.valid.json` defines the deterministic local smoke command, service entrypoint, dependency refs, and harness scenario id.
- `fixtures/valid/phase3_command.valid.json` defines the strict command-envelope validation fixture.
- `fixtures/invalid/admin_unsigned.invalid.json` defines the unsigned admin denial case.
- `fixtures/invalid/phase3_missing_tenant.invalid.json` proves missing required command fields deny with a stable schema error.
- `fixtures/invalid/phase3_unknown_private_payload.invalid.json` proves sensitive unknown fields are rejected.
- `fixtures/invalid/phase3_raw_secret.invalid.json` proves raw secret markers are rejected before storage or diagnostics.

Fixtures are local/test scoped and contain no raw secrets or private payloads.
