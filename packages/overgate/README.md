# Overrid Overgate

Overgate is the Rust-first API ingress and admission boundary for Overrid control-plane commands. This Phase 2 crate provides the service skeleton, Axum route surface, dependency-readiness model, admin authorization guard placeholders, and local/test fixture references needed before Phase 3 command-envelope validation.

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

## Fixtures

- `fixtures/valid/phase2_local_command.valid.json` defines the deterministic local smoke command, service entrypoint, dependency refs, and harness scenario id.
- `fixtures/invalid/admin_unsigned.invalid.json` defines the unsigned admin denial case.

Fixtures are local/test scoped and contain no raw secrets or private payloads.
