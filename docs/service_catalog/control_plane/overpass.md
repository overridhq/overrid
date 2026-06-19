# Overpass Implementation Plan

## Objective

Build Overrid identity and namespace records for people, organizations, nodes, apps, native services, service accounts, system services, swarms, agents, communities, tags, and routes.

## First Build Phase

Overpass-lite in [Phase 1: Control-Plane Skeleton](../../build_plan/phase_01_control_plane_skeleton.md); broader namespace in [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).

## Dependencies

- Shared identity schemas.
- Overgate authentication.
- Overtenant membership.
- Overkey credentials.

## Development Order

1. Implement person, organization, node, app, native-service, service-account, and system-service identities.
2. Add lifecycle states: pending, active, disabled, suspended, and tombstoned.
3. Add identity references into every tenant and manifest record.
4. Add human-readable namespace records in [Phase 8: Data, Storage, and Namespace Platform](../../build_plan/phase_08_data_storage_namespace_platform.md).
5. Add delegation, route binding, verification markers, and dispute hooks.

## Contracts And Interfaces

- Identity API.
- Namespace API.
- Membership and owner refs.
- Verification marker refs.
- Route binding refs.

## Detailed SDS

- [Overpass SDS](../../sds/control_plane/overpass.md)

## Sub-Build Plan

- [SUB BUILD PLAN #10 - Overpass](../../build_plan/sub_build_plan_010_overpass.md)

## Design Alignment

- Treat stable identity ids as the authority; usernames and route names are resolvable namespace records, not primary identity.
- Model people, organizations, nodes, apps, native services, service accounts, system services, swarms, agents, communities, and central AI system actors.
- Preserve tombstones, merge refs, verification markers, dispute refs, and no-reuse protections so old references cannot be silently reassigned.
- Keep credentials in Overkey, tenant authorization in Overtenant, and content/profile behavior in the relevant native services.
- Expose identity and namespace resolution through filtered APIs that downstream services can cache only within allowed scope.

## Validation

- Every command resolves actor identity.
- Suspended identities cannot mutate protected state.
- Namespace records resolve to identity, app, service, route, or asset records.

## Handoff

Overpass feeds Overtenant, Overregistry, Overmesh, Overasset, Seal Ledger, messaging, search, and native apps.
