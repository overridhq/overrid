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

## Phase 1 Implementation Gates

Before the Rust service skeleton starts in Phase 2, Phase 1 freezes the authority boundary that all later Overgate code must preserve.

| Gate | Required outcome | Phase 1 state |
| --- | --- | --- |
| SDS attachment | `docs/build_plan/sub_build_plan_008_overgate.md` links the SDS, service catalog, master plan, crosswalk, and tech-stack guardrails. | `attached` |
| Ingress boundary | Every mutating SDK, CLI, admin UI, native app, mobile client, adapter, node agent, service-account, and operator-tool path enters through Overgate or an explicitly isolated local test shortcut. | `ingress_boundary_frozen` |
| Master phase ownership | First build point remains Phase 1, with Phase 0 prerequisites and later policy, metering, product, and grid-resident hardening. | `master_phase_1_owned` |
| Resolved SDS decisions | Synchronous Phase 1 command scope, Classed idempotency retention, Unsigned low-risk bodyless reads, Rust-owned emergency audit WAL, and Conservative pre-ORU quota precheck refs are carried forward. | `resolved_decision_carried` |
| Runtime authority | Overgate owns admission records, idempotency records, forwarding records, rate-limit buckets, quota-precheck refs, and ingress audit emission, while downstream services own domain state. | `overgate_owned` |

Overgate must not write private downstream records directly. The forbidden set includes settlement, policy finality, identity lifecycle, key lifecycle, private storage mutation, queue domain state, accounting finality, and native-service business state; those records remain `downstream_owned` or `forbidden_in_overgate`.

Any new Overgate command type, unsigned-read rule, idempotency class, forwarding target, quota-precheck input, degraded audit mode, or admin route must update the shared schema package, the Overgate SDS, SUB BUILD PLAN #8, the owning downstream service plan, and the build-plan crosswalk before implementation.

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
