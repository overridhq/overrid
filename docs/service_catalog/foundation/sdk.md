# SDK Implementation Plan

## Objective

Give applications, native services, adapters, and developers a safe client layer for Overrid APIs.

## First Build Phase

Thin client in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); hardened SDK in [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Dependencies

- Shared schema package.
- Overgate API conventions.
- Overkey credentials.
- Stable error format.

## Development Order

1. Generate typed clients from schemas or API specs.
2. Add signing, idempotency, trace id, retry, and error decoding helpers.
3. Add manifest validation and workload submission helpers.
4. Add job status, result, usage, receipt, and policy dry-run helpers.
5. Add native app and adapter convenience layers after [Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md).

## Contracts And Interfaces

- Typed API client.
- Auth and signing helpers.
- Manifest and workload builders.
- Usage, receipt, and result readers.

## Detailed SDS

- [SDK SDS](../../sds/foundation/sdk.md)

## Sub-Build Plan

- [SUB BUILD PLAN #6 - SDK](../../build_plan/sub_build_plan_006_sdk.md)

## Design Alignment

- Treat the SDK as a versioned client package, not a runtime authority or deployed service.
- Route mutating calls through Overgate with actor, tenant, trace id, idempotency key, command type, and schema version.
- Use Overkey-compatible credential references for signing without storing private keys in SDK state.
- Preserve reason codes, trace ids, audit refs, usage refs, and receipt refs returned by runtime services.
- Generate supported language bindings from the shared schema package instead of maintaining hand-written public objects.

## Validation

- SDK can submit a signed workload without manual internal API calls.
- Duplicate idempotency behavior is handled predictably.
- SDK error objects preserve reason codes and trace ids.

## Handoff

This is the client foundation for Docdex, Mcoda, Codali, native apps, and mobile SDK work.
