# Overmesh Implementation Plan

## Objective

Build connectivity, namespace resolution, service discovery, artifact transfer, priority bandwidth leases, geographic routing, and traffic shaping.

## First Build Phase

Private discovery in [Phase 4: Trust, Policy, and Verification](../../build_plan/phase_04_trust_policy_verification.md); route resolution in [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).

## Dependencies

- Overpass namespace.
- Overtenant boundaries.
- Overcell node endpoints.
- Overguard policy.

## Development Order

1. Add trusted private node discovery and health-aware endpoint records.
2. Add tenant-scoped service discovery.
3. Add artifact transfer paths for Overstore and Overrun.
4. Add namespace route resolution in [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).
5. Add bandwidth lease hints and traffic shaping as scheduling matures.

## Contracts And Interfaces

- Endpoint record schema.
- Service discovery API.
- Route resolution API.
- Artifact transfer contract.
- Connectivity health events.

## Detailed SDS

The detailed design contract lives in [Overmesh SDS](../../sds/execution_scheduling/overmesh.md).

## Design Alignment

- Treat Overmesh as policy-bound private connectivity and route metadata, not an unmanaged public network or generic VPN.
- Start with trusted private node endpoint discovery and tenant-scoped service discovery; expand to namespace route resolution after Phase 8 primitives exist.
- Deny cross-tenant discovery, routing, and transfer by default unless Overguard explicitly allows the route.
- Keep endpoint health, route decisions, artifact transfer sessions, and route binding changes auditable and metered.

## Validation

- Cross-tenant routing is denied by default.
- Route changes are auditable.
- Scheduler can use connectivity and locality facts.

## Handoff

Overmesh supports private execution, application routes, native apps, storage transfer, and grid-resident service connectivity.
