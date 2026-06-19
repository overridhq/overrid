# Overgate Implementation Plan

## Objective

Build the API ingress layer for authentication, request signing, idempotency, rate limiting, quota prechecks, command validation, and ingress audit.

## First Build Phase

[Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md).

## Dependencies

- Overpass.
- Overtenant.
- Overkey-lite.
- Shared command schemas.
- Overwatch event log.

## Development Order

1. Implement request authentication and actor resolution.
2. Verify tenant context and role permissions.
3. Validate command envelope and request schema.
4. Add idempotency key handling and trace id assignment.
5. Add rate limits and quota prechecks.
6. Emit ingress audit events before forwarding accepted commands.

## Contracts And Interfaces

- Public API.
- Admin API.
- Command envelope.
- Error format.
- Audit event contract.

## Detailed SDS

- [Overgate SDS](../../sds/control_plane/overgate.md)

## Sub-Build Plan

- [SUB BUILD PLAN #8 - Overgate](../../build_plan/sub_build_plan_008_overgate.md)

## Design Alignment

- Treat Overgate as the required ingress and admission boundary for SDK, CLI, admin UI, native apps, mobile clients, adapters, node agents, and service accounts.
- Authenticate, resolve identity and tenant context, validate schemas, reserve idempotency, apply rate-limit and quota prechecks, emit audit events, and forward accepted commands.
- Deny malformed, unsigned, revoked, cross-tenant, duplicate-conflicting, rate-limited, or policy-blocked requests before downstream side effects.
- Persist only hashes and refs for private payloads unless a service contract explicitly requires retained payload content.
- Keep downstream domain state ownership with the target service; Overgate forwards accepted commands rather than writing private service records.

## Validation

- Invalid signatures are denied before side effects.
- Duplicate idempotency keys behave deterministically.
- Every accepted or denied mutating command emits an audit record.

## Handoff

Overgate is the required entry point for SDK, CLI, admin UI, adapters, and native apps.
