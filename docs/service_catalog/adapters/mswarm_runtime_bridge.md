# mSwarm Runtime Bridge Implementation Plan

## Objective

Connect Overrid resource control to mSwarm local-first runtime concerns such as identity sessions, sync, discovery, collaboration, and cloud coordination hooks.

## First Build Phase

[Phase 6: First Product Integration](../../build_plan/phase_06_first_product_integration.md) or earlier if required by native runtime.

## Dependencies

- mSwarm runtime APIs.
- Overpass.
- Overtenant.
- Overgate.
- Native app requirements.

## Development Order

1. Define identity/session mapping between mSwarm and Overrid.
2. Add sync event and discovery bridges.
3. Add collaboration hooks.
4. Add runtime capability declarations.
5. Add failure and audit integration.

## Contracts And Interfaces

- Session bridge contract.
- Sync event mapping.
- Discovery mapping.
- Runtime capability record.

## Validation

- mSwarm identities map cleanly to Overrid actors/tenants.
- Sync/discovery events are auditable.
- Native apps can use the bridge without bypassing policy.

## Handoff

mSwarm bridge supports local-first app runtime and native application development.

## Detailed SDS

The detailed design contract is [mSwarm Runtime Bridge SDS](../../sds/adapters/mswarm_runtime_bridge.md).

## Design Alignment

- Treat the bridge as a runtime adapter for local-first sessions, sync, discovery, collaboration, and cloud hooks, not as a replacement for Overrid identity, route, policy, storage, or audit services.
- Require bridge sessions, runtime capability snapshots, sync manifests, sync cursors, discovery announcements, collaboration refs, cloud hook refs, failure refs, and usage refs.
- Keep offline reconciliation permission-bound and append-only; local-first behavior must not widen data access or erase prior policy denials.
- Native apps must integrate through the bridge APIs/events while retaining ownership of their own domain data.
