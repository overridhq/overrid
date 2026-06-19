# Overtenant Implementation Plan

## Objective

Build tenant, subtenant, role, quota, suspension, private-swarm, white-label, and offboarding boundaries.

## First Build Phase

[Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md).

## Dependencies

- Overpass identity records.
- Shared tenant schemas.
- Overgate command context.

## Development Order

1. Implement tenant creation and lifecycle.
2. Add membership and role bindings.
3. Add tenant-scoped quotas and budget refs.
4. Add suspension and read-only states.
5. Add private-swarm and white-label metadata.
6. Add offboarding records and cleanup hooks.

## Contracts And Interfaces

- Tenant API.
- Role binding API.
- Quota and budget refs.
- Suspension state checks.
- Offboarding event contracts.

## Detailed SDS

- [Overtenant SDS](../../sds/control_plane/overtenant.md)

## Design Alignment

- Treat Overtenant as the authority for tenant lifecycle, membership, role binding, quota scope, suspension, private-swarm refs, and offboarding refs.
- Every tenant-scoped object must carry tenant id unless Protocol Core marks it as global metadata.
- Provide fast tenant-check APIs for Overgate and services while preserving audit evidence for lifecycle, role, and suspension changes.
- Keep identity proof in Overpass, credentials in Overkey, policy in Overguard, and accounting finality in the accounting services.
- Use suspension, read-only, offboarding, and tombstone states instead of deleting tenant history.

## Validation

- Every tenant-scoped object carries tenant context.
- Role checks are enforced through Overgate and Overguard.
- Suspended tenants cannot submit workloads.

## Handoff

Overtenant is required by Overgate, Overguard, Overregistry, Seal Ledger, Overbill, federation, and native apps.
